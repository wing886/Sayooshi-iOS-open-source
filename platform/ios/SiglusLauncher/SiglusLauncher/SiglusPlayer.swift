import SwiftUI
import UIKit
import QuartzCore

// MARK: - Rust FFI (iOS host-mode)

@_silgen_name("siglus_ios_create")
private func siglus_ios_create(
    _ uiView: UnsafeMutableRawPointer,
    _ widthPx: UInt32,
    _ heightPx: UInt32,
    _ nativeScaleFactor: Double,
    _ gameRootUtf8: UnsafePointer<CChar>
) -> UnsafeMutableRawPointer?

@_silgen_name("siglus_ios_step")
private func siglus_ios_step(_ handle: UnsafeMutableRawPointer?, _ dtMs: UInt32) -> Int32

@_silgen_name("siglus_ios_resize")
private func siglus_ios_resize(_ handle: UnsafeMutableRawPointer?, _ widthPx: UInt32, _ heightPx: UInt32) -> Void

@_silgen_name("siglus_ios_resize_viewport")
private func siglus_ios_resize_viewport(
    _ handle: UnsafeMutableRawPointer?,
    _ widthPx: UInt32,
    _ heightPx: UInt32,
    _ viewportX: UInt32,
    _ viewportY: UInt32,
    _ viewportWidth: UInt32,
    _ viewportHeight: UInt32
) -> Void

@_silgen_name("siglus_ios_logical_size")
private func siglus_ios_logical_size(
    _ handle: UnsafeMutableRawPointer?,
    _ widthOut: UnsafeMutablePointer<UInt32>?,
    _ heightOut: UnsafeMutablePointer<UInt32>?
) -> Void

@_silgen_name("siglus_ios_destroy")
private func siglus_ios_destroy(_ handle: UnsafeMutableRawPointer?) -> Void

@_silgen_name("siglus_ios_touch")
private func siglus_ios_touch(_ handle: UnsafeMutableRawPointer?, _ phase: Int32, _ xPoints: Double, _ yPoints: Double) -> Void

typealias SiglusMessageboxCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UInt64,
    Int32,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?
) -> Void

@_silgen_name("siglus_ios_set_native_messagebox_callback")
private func siglus_ios_set_native_messagebox_callback(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: SiglusMessageboxCallback?,
    _ userData: UnsafeMutableRawPointer?
) -> Void

@_silgen_name("siglus_ios_submit_messagebox_result")
private func siglus_ios_submit_messagebox_result(_ handle: UnsafeMutableRawPointer?, _ requestId: UInt64, _ value: Int64) -> Void

private func siglusFallbackMessageboxValue(kind: Int32) -> Int64 {
    switch kind {
    case 0: return 0
    case 1: return 1
    case 2: return 1
    case 3: return 2
    default: return 0
    }
}

@_cdecl("siglus_ios_messagebox_callback")
func siglus_ios_messagebox_callback(
    userData: UnsafeMutableRawPointer?,
    requestId: UInt64,
    kind: Int32,
    titleUtf8: UnsafePointer<CChar>?,
    messageUtf8: UnsafePointer<CChar>?
) {
    guard let userData else { return }
    let controller = Unmanaged<SiglusPlayerViewController>.fromOpaque(userData).takeUnretainedValue()
    let title = titleUtf8.map { String(cString: $0) } ?? "Siglus"
    let message = messageUtf8.map { String(cString: $0) } ?? ""
    DispatchQueue.main.async {
        controller.showNativeMessagebox(requestId: requestId, kind: kind, title: title, message: message)
    }
}

// MARK: - Metal-backed UIView for wgpu

final class SiglusMetalView: UIView {
    override class var layerClass: AnyClass { CAMetalLayer.self }

    // phase: 0 began, 1 moved, 2 ended, 3 cancelled
    var onTouch: ((Int32, Double, Double) -> Void)?
    override init(frame: CGRect) {
        super.init(frame: frame)
        isOpaque = true
        backgroundColor = .black
        isUserInteractionEnabled = true
        isMultipleTouchEnabled = false
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        isOpaque = true
        backgroundColor = .black
        isUserInteractionEnabled = true
        isMultipleTouchEnabled = false
    }

    func configureScale(_ scale: CGFloat) {
        contentScaleFactor = scale
        if let layer = self.layer as? CAMetalLayer {
            layer.contentsScale = scale
        }
    }

    func configureDrawableSize(_ size: CGSize) {
        if let layer = self.layer as? CAMetalLayer {
            layer.drawableSize = size
        }
    }

    private func send(_ phase: Int32, _ touches: Set<UITouch>) {
        guard let t = touches.first else { return }
        let p = t.location(in: self) // points
        onTouch?(phase, Double(p.x), Double(p.y))
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) { send(0, touches) }
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) { send(1, touches) }
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) { send(2, touches) }
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) { send(3, touches) }
}

// MARK: - UIViewController that owns the engine + CADisplayLink

final class SiglusPlayerViewController: UIViewController {
    private let gameRoot: String
    private let onExit: () -> Void

    private var metalView: SiglusMetalView { view as! SiglusMetalView }

    private var handle: UnsafeMutableRawPointer? = nil
    private var displayLink: CADisplayLink? = nil
    private var lastTimestamp: CFTimeInterval? = nil

    private var lastDrawableSizePx: (UInt32, UInt32) = (0, 0)
    private var lastScale: Double = 0.0
    private var logicalSize: (UInt32, UInt32) = (1280, 720)
    private var viewportPx: (UInt32, UInt32, UInt32, UInt32) = (0, 0, 1, 1)
    private var viewportPoints: CGRect = CGRect(x: 0, y: 0, width: 1, height: 1)

    init(gameRoot: String, onExit: @escaping () -> Void) {
        self.gameRoot = gameRoot
        self.onExit = onExit
        super.init(nibName: nil, bundle: nil)
        modalPresentationStyle = .fullScreen
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        view = SiglusMetalView(frame: .zero)
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black

        metalView.onTouch = { [weak self] phase, x, y in
            guard let self = self else { return }
            guard let h = self.handle else { return }
            let gamePoint = self.mapPointToGame(x: x, y: y)
            siglus_ios_touch(h, phase, gamePoint.x, gamePoint.y)
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()

        let sizePoints = view.bounds.size
        if sizePoints.width <= 0 || sizePoints.height <= 0 { return }

        let screen = view.window?.screen ?? UIScreen.main
        let scale = screen.scale
        metalView.configureScale(CGFloat(scale))

        let wPx = UInt32(max(1.0, (sizePoints.width * scale).rounded(.toNearestOrAwayFromZero)))
        let hPx = UInt32(max(1.0, (sizePoints.height * scale).rounded(.toNearestOrAwayFromZero)))
        metalView.configureDrawableSize(CGSize(width: CGFloat(wPx), height: CGFloat(hPx)))
        recomputeViewport(wPx: wPx, hPx: hPx, scale: scale, reason: "layout")

        if handle == nil {
            createEngineIfNeeded(wPx: wPx, hPx: hPx, scale: scale)
        } else {
            if wPx != lastDrawableSizePx.0 || hPx != lastDrawableSizePx.1 || scale != lastScale {
                lastDrawableSizePx = (wPx, hPx)
                lastScale = scale
                siglus_ios_resize_viewport(handle, wPx, hPx, viewportPx.0, viewportPx.1, viewportPx.2, viewportPx.3)
            }
        }
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)

        if #available(iOS 16.0, *) {
            if let scene = view.window?.windowScene {
                scene.requestGeometryUpdate(.iOS(interfaceOrientations: .landscape))
            }
        }

        setNeedsStatusBarAppearanceUpdate()
        startDisplayLink()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        stopDisplayLink()
    }

    deinit {
        stopDisplayLink()
        if handle != nil {
            siglus_ios_set_native_messagebox_callback(handle, nil, nil)
            siglus_ios_destroy(handle)
            handle = nil
        }
    }

    private func createEngineIfNeeded(wPx: UInt32, hPx: UInt32, scale: Double) {
        let viewPtr = UnsafeMutableRawPointer(Unmanaged.passUnretained(metalView).toOpaque())

        gameRoot.withCString { gameC in
            let hnd = siglus_ios_create(viewPtr, wPx, hPx, scale, gameC)
            self.handle = hnd
            if hnd != nil {
                var lw: UInt32 = 1280
                var lh: UInt32 = 720
                siglus_ios_logical_size(hnd, &lw, &lh)
                self.logicalSize = (max(1, lw), max(1, lh))
                self.recomputeViewport(wPx: wPx, hPx: hPx, scale: scale, reason: "create")
                siglus_ios_resize_viewport(hnd, wPx, hPx, self.viewportPx.0, self.viewportPx.1, self.viewportPx.2, self.viewportPx.3)
                let userData = UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque())
                siglus_ios_set_native_messagebox_callback(hnd, siglus_ios_messagebox_callback, userData)
            }
            self.lastDrawableSizePx = (wPx, hPx)
            self.lastScale = scale
        }

        if handle == nil {
            onExit()
        }
    }

    private func recomputeViewport(wPx: UInt32, hPx: UInt32, scale: Double, reason: String) {
        let lw = Double(max(1, logicalSize.0))
        let lh = Double(max(1, logicalSize.1))
        let sw = Double(max(1, wPx))
        let sh = Double(max(1, hPx))
        let fit = min(sw / lw, sh / lh)
        let vw = UInt32(max(1.0, min(sw, (lw * fit).rounded())))
        let vh = UInt32(max(1.0, min(sh, (lh * fit).rounded())))
        let vx = (max(1, wPx) - vw) / 2
        let vy = (max(1, hPx) - vh) / 2
        viewportPx = (vx, vy, vw, vh)
        viewportPoints = CGRect(
            x: CGFloat(Double(vx) / max(scale, 1.0)),
            y: CGFloat(Double(vy) / max(scale, 1.0)),
            width: CGFloat(Double(vw) / max(scale, 1.0)),
            height: CGFloat(Double(vh) / max(scale, 1.0))
        )
        logViewport(reason: reason, wPx: wPx, hPx: hPx, scale: scale)
    }

    private func mapPointToGame(x: Double, y: Double) -> (x: Double, y: Double) {
        let rect = viewportPoints
        let gx = (x - Double(rect.minX)) * Double(logicalSize.0) / max(Double(rect.width), 1.0)
        let gy = (y - Double(rect.minY)) * Double(logicalSize.1) / max(Double(rect.height), 1.0)
        return (gx, gy)
    }

    private func logViewport(reason: String, wPx: UInt32, hPx: UInt32, scale: Double) {
        let orientation = view.window?.windowScene?.interfaceOrientation.rawValue ?? -1
        let insets = view.safeAreaInsets
        let drawable = (metalView.layer as? CAMetalLayer)?.drawableSize ?? .zero
        print(
            "[SIGLUS_IOS_VIEWPORT] swift reason=\(reason) bounds_points=\(view.bounds.width)x\(view.bounds.height) " +
            "screen_scale=\(scale) drawable_px=\(wPx)x\(hPx) layer_drawable=\(drawable.width)x\(drawable.height) " +
            "safe_area=(\(insets.top),\(insets.left),\(insets.bottom),\(insets.right)) orientation=\(orientation) " +
            "rust_resize=\(wPx)x\(hPx) logical=\(logicalSize.0)x\(logicalSize.1) " +
            "viewport_px=\(viewportPx.2)x\(viewportPx.3)+\(viewportPx.0)+\(viewportPx.1) " +
            "viewport_points=\(viewportPoints.width)x\(viewportPoints.height)+\(viewportPoints.minX)+\(viewportPoints.minY)"
        )
    }

    private func startDisplayLink() {
        if displayLink != nil { return }
        let link = CADisplayLink(target: self, selector: #selector(onDisplayLink(_:)))
        link.add(to: .main, forMode: .common)
        displayLink = link
        lastTimestamp = nil
    }

    private func stopDisplayLink() {
        displayLink?.invalidate()
        displayLink = nil
        lastTimestamp = nil
    }

    @objc private func onDisplayLink(_ link: CADisplayLink) {
        guard let handle else { return }

        let now = link.timestamp
        let dtSec: Double
        if let last = lastTimestamp {
            dtSec = now - last
        } else {
            dtSec = link.duration
        }
        lastTimestamp = now

        let clamped = min(max(dtSec, 0.0), 0.2)
        let dtMs = UInt32((clamped * 1000.0).rounded(.toNearestOrAwayFromZero))

        let status = siglus_ios_step(handle, dtMs)
        if status == 1 {
            print("[SIGLUS_IOS_STATUS] swift engine requested exit; returning to launcher")
            onExit()
        } else if status != 0 {
            print("[SIGLUS_IOS_ERROR] swift engine step failed status=\(status); keeping player open for diagnostics")
            stopDisplayLink()
        }
    }

    func showNativeMessagebox(requestId: UInt64, kind: Int32, title: String, message: String) {
        guard let handle else {
            return
        }
        let alert = UIAlertController(title: title.isEmpty ? "Siglus" : title, message: message, preferredStyle: .alert)

        switch kind {
        case 0:
            alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 0)
            })
        case 1:
            alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 0)
            })
            alert.addAction(UIAlertAction(title: "Cancel", style: .cancel) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 1)
            })
        case 2:
            alert.addAction(UIAlertAction(title: "Yes", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 0)
            })
            alert.addAction(UIAlertAction(title: "No", style: .cancel) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 1)
            })
        case 3:
            alert.addAction(UIAlertAction(title: "Yes", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 0)
            })
            alert.addAction(UIAlertAction(title: "No", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 1)
            })
            alert.addAction(UIAlertAction(title: "Cancel", style: .cancel) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 2)
            })
        default:
            alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in
                siglus_ios_submit_messagebox_result(handle, requestId, 0)
            })
        }

        if presentedViewController != nil {
            dismiss(animated: false) { [weak self] in
                self?.present(alert, animated: true)
            }
        } else {
            present(alert, animated: true)
        }
    }

    // MARK: - Fullscreen / orientation (mobile semantics)

    override var prefersStatusBarHidden: Bool { true }
    override var prefersHomeIndicatorAutoHidden: Bool { true }

    override var supportedInterfaceOrientations: UIInterfaceOrientationMask { .landscape }
    override var preferredInterfaceOrientationForPresentation: UIInterfaceOrientation { .landscapeRight }
}

// MARK: - SwiftUI bridge

struct SiglusPlayerContainer: UIViewControllerRepresentable {
    let gameRoot: String
    let onExit: () -> Void

    func makeUIViewController(context: Context) -> SiglusPlayerViewController {
        SiglusPlayerViewController(gameRoot: gameRoot, onExit: onExit)
    }

    func updateUIViewController(_ uiViewController: SiglusPlayerViewController, context: Context) {
        // No-op
    }
}

struct SiglusPlayerScreen: View {
    @EnvironmentObject var library: GameLibrary
    let game: GameEntry

    var body: some View {
        SiglusPlayerContainer(
            gameRoot: game.rootPath,
            onExit: {
                DispatchQueue.main.async {
                    library.activeGame = nil
                }
            }
        )
        .ignoresSafeArea()
        .statusBarHidden(true)
    }
}
