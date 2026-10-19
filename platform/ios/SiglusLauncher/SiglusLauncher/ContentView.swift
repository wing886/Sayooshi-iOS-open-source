import SwiftUI
import UIKit
import UniformTypeIdentifiers

struct ContentView: View {
    @EnvironmentObject var library: GameLibrary

    @State private var isLaunching: Bool = false
    @State private var showPicker: Bool = false

    private let columns: [GridItem] = [
        GridItem(.adaptive(minimum: 160, maximum: 240), spacing: 12, alignment: .top),
    ]

    var body: some View {
        ZStack {
            VStack(spacing: 0) {
                header
                if library.games.isEmpty && library.unsupported.isEmpty {
                    emptyState
                } else {
                    ScrollView(.vertical) {
                        LazyVGrid(columns: columns, alignment: .leading, spacing: 12) {
                            ForEach(library.games) { game in
                                GameTileView(game: game, isLaunching: $isLaunching)
                            }
                        }
                        .padding(14)
                        if !library.unsupported.isEmpty {
                            unsupportedSection
                        }
                    }
                }
            }
            .background(Color(UIColor.systemGroupedBackground).ignoresSafeArea())
            .alert(isPresented: $library.showError) {
                Alert(
                    title: Text("Error"),
                    message: Text(library.errorMessage),
                    dismissButton: .default(Text("OK"))
                )
            }

            if isLaunching || library.isBusy {
                Color.black.opacity(0.35).ignoresSafeArea()
                VStack(spacing: 12) {
                    ProgressView()
                    Text(isLaunching ? "Launching…" : "Scanning…")
                        .foregroundColor(.white)
                }
                .padding(18)
                .background(RoundedRectangle(cornerRadius: 12).fill(Color.black.opacity(0.6)))
            }
        }
        .sheet(isPresented: $showPicker) {
            FolderPicker { urls in
                showPicker = false
                library.importFolders(urls)
            }
        }
        .fullScreenCover(item: $library.activeGame, onDismiss: {
            isLaunching = false
        }) { game in
            GamePlayerScreen(game: game)
                .environmentObject(library)
        }
        .background(
            EmptyView().alert(item: Binding(
                get: { library.notice.map { NoticeItem(text: $0) } },
                set: { if $0 == nil { library.notice = nil } }
            )) { item in
                Alert(title: Text("Import"), message: Text(item.text), dismissButton: .default(Text("OK")))
            }
        )
    }

    private var header: some View {
        HStack(spacing: 10) {
            VStack(alignment: .leading, spacing: 1) {
                Text("Games")
                    .font(.title2)
                    .bold()
                Text("SiglusEngine · RealLive · AVG32 · UK2")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button {
                library.rescanFromDocuments(announceNew: true)
            } label: {
                Image(systemName: "arrow.clockwise")
            }

            Button {
                showPicker = true
            } label: {
                Label("Import", systemImage: "plus")
            }
        }
        .padding(.horizontal, 14)
        .padding(.vertical, 10)
    }

    private var emptyState: some View {
        VStack(spacing: 12) {
            Spacer()
            Image(systemName: "square.stack.3d.up")
                .font(.system(size: 48))
                .foregroundColor(.secondary)
            Text("No games yet")
                .font(.title3)
                .bold()
            Text("Tap Import to choose one or more game folders, or copy them into Files → On My iPhone → Siglus → siglus and tap refresh. SiglusEngine, RealLive, AVG32 and UK2 games are detected automatically.")
                .font(.footnote)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 30)
            Button("Import Games") { showPicker = true }
            Spacer()
        }
    }

    private var unsupportedSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Not supported")
                .font(.headline)
            ForEach(library.unsupported) { game in
                VStack(alignment: .leading, spacing: 2) {
                    Text(game.title).font(.subheadline).bold()
                    Text(game.reason).font(.caption).foregroundColor(.secondary)
                }
                .padding(10)
                .frame(maxWidth: .infinity, alignment: .leading)
                .background(RoundedRectangle(cornerRadius: 10).fill(Color(UIColor.secondarySystemGroupedBackground)))
            }
        }
        .padding(.horizontal, 14)
        .padding(.bottom, 20)
    }
}

private struct NoticeItem: Identifiable {
    var id: String { text }
    let text: String
}

/// Multi-select folder picker (opened in place; the library copies them).
struct FolderPicker: UIViewControllerRepresentable {
    let onPick: ([URL]) -> Void

    func makeCoordinator() -> Coordinator { Coordinator(onPick: onPick) }

    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        let picker = UIDocumentPickerViewController(forOpeningContentTypes: [.folder], asCopy: false)
        picker.allowsMultipleSelection = true
        picker.delegate = context.coordinator
        return picker
    }

    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}

    final class Coordinator: NSObject, UIDocumentPickerDelegate {
        let onPick: ([URL]) -> Void

        init(onPick: @escaping ([URL]) -> Void) { self.onPick = onPick }

        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            onPick(urls)
        }

        func documentPickerWasCancelled(_ controller: UIDocumentPickerViewController) {
            onPick([])
        }
    }
}

struct EngineBadge: View {
    let engine: String
    let name: String

    private var color: Color {
        switch engine {
        case "siglus": return Color(red: 0.35, green: 0.45, blue: 0.95)
        case "reallive": return Color(red: 0.90, green: 0.45, blue: 0.25)
        case "avg32": return Color(red: 0.25, green: 0.65, blue: 0.45)
        case "uk2": return Color(red: 0.70, green: 0.35, blue: 0.80)
        default: return Color.gray
        }
    }

    var body: some View {
        Text(name)
            .font(.system(size: 10, weight: .semibold))
            .foregroundColor(.white)
            .padding(.horizontal, 7)
            .padding(.vertical, 3)
            .background(Capsule().fill(color.opacity(0.92)))
    }
}

struct GameTileView: View {
    @EnvironmentObject var library: GameLibrary

    let game: GameEntry
    @Binding var isLaunching: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // A fixed 16:9 frame that follows the column width. The artwork is an
            // overlay, so a fill-scaled image can never widen the tile.
            Color.clear
                .aspectRatio(16.0 / 9.0, contentMode: .fit)
                .overlay(cover)
                .overlay(
                    EngineBadge(engine: game.engine, name: game.engineName).padding(6),
                    alignment: .topLeading
                )
                .clipShape(RoundedRectangle(cornerRadius: 10))
                .contentShape(Rectangle())
                .onTapGesture { play() }

            HStack(spacing: 6) {
                VStack(alignment: .leading, spacing: 2) {
                    Text(game.title)
                        .font(.system(size: 14, weight: .semibold))
                        .lineLimit(1)
                    Text(game.nlsLabel ?? game.engineName)
                        .font(.caption2)
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                }
                Spacer(minLength: 4)
                Menu {
                    tileMenu
                } label: {
                    Image(systemName: "ellipsis.circle")
                        .font(.title3)
                        .padding(.vertical, 4)
                }
                .fixedSize()
            }
            .padding(.horizontal, 10)
            .padding(.vertical, 8)
        }
        .background(RoundedRectangle(cornerRadius: 12).fill(Color(UIColor.secondarySystemGroupedBackground)))
        .contextMenu { tileMenu }
    }

    @ViewBuilder
    private var tileMenu: some View {
        Button("Play") { play() }
        if !game.nlsOptions.isEmpty {
            Menu("Text Encoding") {
                ForEach(game.nlsOptions, id: \.id) { option in
                    Button((option.id == game.nls ? "✓ " : "") + option.label) {
                        library.setNls(option.id, for: game)
                    }
                }
            }
        }
        Button("Delete") { library.remove(game: game) }
    }

    private func play() {
        isLaunching = true
        DispatchQueue.main.async {
            library.launch(game: game)
        }
    }

    @ViewBuilder
    private var cover: some View {
        if let image = library.loadCoverImage(game: game) {
            Image(uiImage: image)
                .resizable()
                .interpolation(game.coverKind == "icon" ? .none : .medium)
                .scaledToFill()
        } else {
            ZStack {
                LinearGradient(
                    gradient: Gradient(colors: [Color(red: 0.20, green: 0.22, blue: 0.30), Color(red: 0.08, green: 0.09, blue: 0.13)]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                Text(game.title)
                    .font(.headline)
                    .foregroundColor(.white)
                    .multilineTextAlignment(.center)
                    .lineLimit(3)
                    .padding(10)
            }
        }
    }
}
