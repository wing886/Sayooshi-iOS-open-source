import SwiftUI
import UIKit
import QuartzCore

// MARK: - Rust framebuffer runtime (RealLive, AVG32, UK2)

@_silgen_name("game_fb_open")
private func game_fb_open(_ root: UnsafePointer<CChar>, _ engine: UnsafePointer<CChar>?, _ nls: UnsafePointer<CChar>?, _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> OpaquePointer?

@_silgen_name("game_fb_step")
private func game_fb_step(_ game: OpaquePointer?, _ dtMs: UInt32) -> Int32

@_silgen_name("game_fb_frame")
private func game_fb_frame(_ game: OpaquePointer?, _ width: UnsafeMutablePointer<UInt32>?, _ height: UnsafeMutablePointer<UInt32>?) -> UnsafePointer<UInt8>?

@_silgen_name("game_fb_pointer_move")
private func game_fb_pointer_move(_ game: OpaquePointer?, _ x: Int32, _ y: Int32)

@_silgen_name("game_fb_pointer_button")
private func game_fb_pointer_button(_ game: OpaquePointer?, _ button: Int32, _ pressed: Int32)

@_silgen_name("game_fb_wheel")
private func game_fb_wheel(_ game: OpaquePointer?, _ up: Int32)

@_silgen_name("game_fb_key")
private func game_fb_key(_ game: OpaquePointer?, _ code: UInt32, _ pressed: Int32)

@_silgen_name("game_fb_close")
private func game_fb_close(_ game: OpaquePointer?)

@_silgen_name("game_string_free")
private func game_string_free(_ ptr: UnsafeMutablePointer<CChar>?)

private enum FbKey {
    static let enter: UInt32 = 1
    static let escape: UInt32 = 2
    static let ctrl: UInt32 = 14
}

/// Presents the RGBA frames of a software-rendered engine, letterboxed with
/// nearest-neighbour scaling, and turns touches into mouse input:
/// tap = left click, two-finger tap = right click (menu / back),
/// drag = pointer move, vertical two-finger swipe = wheel.
final class FramebufferGameView: UIView {
    fileprivate var game: OpaquePointer?
    fileprivate var frameSize = CGSize(width: 640, height: 480)

    override init(frame: CGRect) {
        super.init(frame: frame)
        backgroundColor = .black
        isMultipleTouchEnabled = true
        layer.contentsGravity = .resizeAspect
        layer.magnificationFilter = .nearest
        layer.minificationFilter = .linear
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    /// The letterboxed rectangle the frame occupies, in view points.
    private var contentRect: CGRect {
        let bounds = self.bounds
        let scale = min(bounds.width / frameSize.width, bounds.height / frameSize.height)
        let w = frameSize.width * scale
        let h = frameSize.height * scale
        return CGRect(x: (bounds.width - w) / 2, y: (bounds.height - h) / 2, width: w, height: h)
    }

    private func gamePoint(_ p: CGPoint) -> (Int32, Int32) {
        let rect = contentRect
        let x = (p.x - rect.minX) * frameSize.width / max(rect.width, 1)
        let y = (p.y - rect.minY) * frameSize.height / max(rect.height, 1)
        let cx = min(max(x, 0), frameSize.width - 1)
        let cy = min(max(y, 0), frameSize.height - 1)
        return (Int32(cx), Int32(cy))
    }

    func display(pixels: UnsafePointer<UInt8>, width: Int, height: Int) {
        frameSize = CGSize(width: width, height: height)
        let bytes = width * height * 4
        guard let data = CFDataCreate(nil, pixels, bytes),
              let provider = CGDataProvider(data: data),
              let image = CGImage(
                width: width, height: height,
                bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: width * 4,
                space: CGColorSpaceCreateDeviceRGB(),
                bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipLast.rawValue),
                provider: provider, decode: nil, shouldInterpolate: false,
                intent: .defaultIntent)
        else { return }
        CATransaction.begin()
        CATransaction.setDisableActions(true)
        layer.contents = image
        CATransaction.commit()
    }

    private var twoFingerStart: CGPoint? = nil
    private var twoFingerScrolled = false
    private var tracking = false

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        let all = event?.allTouches ?? touches
        if all.count >= 2 {
            twoFingerStart = centroid(all)
            twoFingerScrolled = false
            if tracking {
                // A second finger turns the gesture into a right click / scroll.
                tracking = false
            }
            return
        }
        guard let t = touches.first else { return }
        let (x, y) = gamePoint(t.location(in: self))
        game_fb_pointer_move(game, x, y)
        game_fb_pointer_button(game, 0, 1)
        tracking = true
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        let all = event?.allTouches ?? touches
        if let start = twoFingerStart, all.count >= 2 {
            let now = centroid(all)
            let dy = now.y - start.y
            if abs(dy) > 24 {
                game_fb_wheel(game, dy > 0 ? 1 : 0)
                twoFingerStart = now
                twoFingerScrolled = true
            }
            return
        }
        guard tracking, let t = touches.first else { return }
        let (x, y) = gamePoint(t.location(in: self))
        game_fb_pointer_move(game, x, y)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        if twoFingerStart != nil {
            let remaining = (event?.allTouches ?? touches).filter { $0.phase != .ended && $0.phase != .cancelled }
            if remaining.isEmpty {
                if !twoFingerScrolled {
                    game_fb_pointer_button(game, 1, 1)
                    game_fb_pointer_button(game, 1, 0)
                }
                twoFingerStart = nil
            }
            return
        }
        guard tracking, let t = touches.first else { return }
        let (x, y) = gamePoint(t.location(in: self))
        game_fb_pointer_move(game, x, y)
        game_fb_pointer_button(game, 0, 0)
        tracking = false
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        if tracking {
            game_fb_pointer_button(game, 0, 0)
        }
        tracking = false
        twoFingerStart = nil
    }

    private func centroid(_ touches: Set<UITouch>) -> CGPoint {
        var x: CGFloat = 0
        var y: CGFloat = 0
        for t in touches {
            let p = t.location(in: self)
            x += p.x
            y += p.y
        }
        let n = CGFloat(max(touches.count, 1))
        return CGPoint(x: x / n, y: y / n)
    }
}

final class FramebufferPlayerViewController: UIViewController {
    private let game: GameEntry
    private let onExit: () -> Void
    private var handle: OpaquePointer? = nil
    private var displayLink: CADisplayLink? = nil
    private var lastTimestamp: CFTimeInterval? = nil
    private var skipHeld = false
    private let gameView = FramebufferGameView(frame: .zero)
    private let toolbar = UIStackView()
    private var skipButton: UIButton? = nil

    init(game: GameEntry, onExit: @escaping () -> Void) {
        self.game = game
        self.onExit = onExit
        super.init(nibName: nil, bundle: nil)
        modalPresentationStyle = .fullScreen
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func loadView() {
        view = UIView()
        view.backgroundColor = .black
        gameView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(gameView)
        NSLayoutConstraint.activate([
            gameView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            gameView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            gameView.topAnchor.constraint(equalTo: view.topAnchor),
            gameView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])

        toolbar.axis = .vertical
        toolbar.spacing = 10
        toolbar.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(toolbar)
        NSLayoutConstraint.activate([
            toolbar.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -8),
            toolbar.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 8),
        ])
        toolbar.addArrangedSubview(makeButton("xmark", #selector(exitTapped)))
        toolbar.addArrangedSubview(makeButton("list.bullet", #selector(menuTapped)))
        let skip = makeButton("forward.fill", #selector(skipTapped))
        skipButton = skip
        toolbar.addArrangedSubview(skip)
    }

    private func makeButton(_ symbol: String, _ action: Selector) -> UIButton {
        let button = UIButton(type: .system)
        button.setImage(UIImage(systemName: symbol), for: .normal)
        button.tintColor = .white
        button.backgroundColor = UIColor.black.withAlphaComponent(0.35)
        button.layer.cornerRadius = 18
        button.widthAnchor.constraint(equalToConstant: 36).isActive = true
        button.heightAnchor.constraint(equalToConstant: 36).isActive = true
        button.addTarget(self, action: action, for: .touchUpInside)
        return button
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        if #available(iOS 16.0, *) {
            view.window?.windowScene?.requestGeometryUpdate(.iOS(interfaceOrientations: .landscape))
        }
        if handle == nil {
            open()
        }
        startDisplayLink()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        stopDisplayLink()
    }

    deinit {
        stopDisplayLink()
        if let handle {
            game_fb_close(handle)
        }
    }

    private func open() {
        var error: UnsafeMutablePointer<CChar>? = nil
        let opened: OpaquePointer? = game.rootPath.withCString { root in
            game.engine.withCString { engine in
                if let nls = game.nls {
                    return nls.withCString { game_fb_open(root, engine, $0, &error) }
                }
                return game_fb_open(root, engine, nil, &error)
            }
        }
        handle = opened
        gameView.game = opened
        if opened == nil {
            var message = "The game could not be started."
            if let error {
                message = String(cString: error)
                game_string_free(error)
            }
            let alert = UIAlertController(title: game.title, message: message, preferredStyle: .alert)
            alert.addAction(UIAlertAction(title: "OK", style: .default) { [weak self] _ in self?.onExit() })
            present(alert, animated: true)
        }
    }

    private func startDisplayLink() {
        guard displayLink == nil, handle != nil else { return }
        let link = CADisplayLink(target: self, selector: #selector(onFrame(_:)))
        if #available(iOS 15.0, *) {
            link.preferredFrameRateRange = CAFrameRateRange(minimum: 30, maximum: 60, preferred: 60)
        } else {
            link.preferredFramesPerSecond = 60
        }
        link.add(to: .main, forMode: .common)
        displayLink = link
        lastTimestamp = nil
    }

    private func stopDisplayLink() {
        displayLink?.invalidate()
        displayLink = nil
    }

    @objc private func onFrame(_ link: CADisplayLink) {
        guard let handle else { return }
        let dt = lastTimestamp.map { link.timestamp - $0 } ?? link.duration
        lastTimestamp = link.timestamp
        let dtMs = UInt32(min(max(dt, 0), 0.2) * 1000)
        if game_fb_step(handle, dtMs) != 0 {
            stopDisplayLink()
            onExit()
            return
        }
        var w: UInt32 = 0
        var h: UInt32 = 0
        if let pixels = game_fb_frame(handle, &w, &h), w > 0, h > 0 {
            gameView.display(pixels: pixels, width: Int(w), height: Int(h))
        }
    }

    @objc private func exitTapped() {
        let alert = UIAlertController(title: "Quit \(game.title)?", message: "Unsaved progress will be lost.", preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "Cancel", style: .cancel))
        alert.addAction(UIAlertAction(title: "Quit", style: .destructive) { [weak self] _ in
            self?.stopDisplayLink()
            self?.onExit()
        })
        present(alert, animated: true)
    }

    @objc private func menuTapped() {
        guard let handle else { return }
        game_fb_pointer_button(handle, 1, 1)
        game_fb_pointer_button(handle, 1, 0)
    }

    @objc private func skipTapped() {
        guard let handle else { return }
        skipHeld.toggle()
        game_fb_key(handle, FbKey.ctrl, skipHeld ? 1 : 0)
        skipButton?.backgroundColor = skipHeld
            ? UIColor.systemBlue.withAlphaComponent(0.7)
            : UIColor.black.withAlphaComponent(0.35)
    }

    override var prefersStatusBarHidden: Bool { true }
    override var prefersHomeIndicatorAutoHidden: Bool { true }
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask { .landscape }
    override var preferredInterfaceOrientationForPresentation: UIInterfaceOrientation { .landscapeRight }
}

struct FramebufferPlayerContainer: UIViewControllerRepresentable {
    let game: GameEntry
    let onExit: () -> Void

    func makeUIViewController(context: Context) -> FramebufferPlayerViewController {
        FramebufferPlayerViewController(game: game, onExit: onExit)
    }

    func updateUIViewController(_ uiViewController: FramebufferPlayerViewController, context: Context) {}
}

/// Chooses the player for the game's engine.
struct GamePlayerScreen: View {
    @EnvironmentObject var library: GameLibrary
    let game: GameEntry

    var body: some View {
        Group {
            if game.isSiglus {
                SiglusPlayerScreen(game: game)
            } else {
                FramebufferPlayerContainer(game: game) {
                    DispatchQueue.main.async { library.activeGame = nil }
                }
                .ignoresSafeArea()
                .statusBarHidden(true)
            }
        }
    }
}
