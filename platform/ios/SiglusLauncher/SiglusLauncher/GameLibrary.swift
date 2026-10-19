import Foundation
import SwiftUI
import UIKit
import CoreText
import Darwin

// MARK: - Rust launcher API (game_launcher)

@_silgen_name("game_scan_json")
private func game_scan_json(_ path: UnsafePointer<CChar>, _ depth: Int32, _ coverCacheDir: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("game_probe_json")
private func game_probe_json(_ root: UnsafePointer<CChar>, _ nls: UnsafePointer<CChar>?, _ coverCacheDir: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("game_string_free")
private func game_string_free(_ ptr: UnsafeMutablePointer<CChar>?) -> Void

@_silgen_name("game_add_font_file")
private func game_add_font_file(_ path: UnsafePointer<CChar>) -> Int32

private func takeString(_ ptr: UnsafeMutablePointer<CChar>?) -> String? {
    guard let ptr else { return nil }
    let out = String(cString: ptr)
    game_string_free(ptr)
    return out
}

/// Hands the system CJK font files to the engines (RealLive, AVG32 and UK2
/// rasterise text themselves and iOS keeps its fonts outside fixed paths).
func registerSystemFontsForGames() {
    for name in ["HiraginoSans-W3", "PingFangSC-Regular", "PingFangTC-Regular", "AppleSDGothicNeo-Regular"] {
        let font = CTFontCreateWithName(name as CFString, 16, nil)
        guard let url = CTFontCopyAttribute(font, kCTFontURLAttribute) as? URL else { continue }
        _ = url.path.withCString { game_add_font_file($0) }
    }
}

// MARK: - Model

struct NlsOption: Codable, Equatable, Hashable {
    let id: String
    let label: String
}

/// One result of the Rust probe (see `GameInfo::to_json`).
struct ProbedGame: Decodable {
    let id: String?
    let root: String
    let engine: String
    let engine_name: String?
    let supported: Bool
    let unsupported_reason: String?
    let title: String
    let cover: String?
    let cover_kind: String?
    let nls: String?
    let nls_options: [NlsOption]?
}

struct GameEntry: Identifiable, Codable, Equatable {
    let id: String
    var title: String
    var rootPath: String

    var addedAtUnix: Int64
    var lastPlayedAtUnix: Int64?
    var coverPath: String?
    /// "siglus", "reallive", "avg32" or "uk2".
    var engine: String
    var engineName: String
    var coverKind: String?
    /// Text encoding for RealLive/AVG32/UK2 (nil for SiglusEngine).
    var nls: String?
    var nlsOptions: [NlsOption]

    init(
        id: String,
        title: String,
        rootPath: String,
        addedAtUnix: Int64,
        lastPlayedAtUnix: Int64? = nil,
        coverPath: String? = nil,
        engine: String = "siglus",
        engineName: String = "SiglusEngine",
        coverKind: String? = nil,
        nls: String? = nil,
        nlsOptions: [NlsOption] = []
    ) {
        self.id = id
        self.title = title
        self.rootPath = rootPath
        self.addedAtUnix = addedAtUnix
        self.lastPlayedAtUnix = lastPlayedAtUnix
        self.coverPath = coverPath
        self.engine = engine
        self.engineName = engineName
        self.coverKind = coverKind
        self.nls = nls
        self.nlsOptions = nlsOptions
    }

    enum CodingKeys: String, CodingKey {
        case id, title, rootPath, addedAtUnix, lastPlayedAtUnix, coverPath
        case engine, engineName, coverKind, nls, nlsOptions
    }

    init(from decoder: Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        id = try c.decode(String.self, forKey: .id)
        title = try c.decode(String.self, forKey: .title)
        rootPath = try c.decode(String.self, forKey: .rootPath)
        addedAtUnix = try c.decode(Int64.self, forKey: .addedAtUnix)
        lastPlayedAtUnix = try c.decodeIfPresent(Int64.self, forKey: .lastPlayedAtUnix)
        coverPath = try c.decodeIfPresent(String.self, forKey: .coverPath)
        engine = try c.decodeIfPresent(String.self, forKey: .engine) ?? "siglus"
        engineName = try c.decodeIfPresent(String.self, forKey: .engineName) ?? "SiglusEngine"
        coverKind = try c.decodeIfPresent(String.self, forKey: .coverKind)
        nls = try c.decodeIfPresent(String.self, forKey: .nls)
        nlsOptions = try c.decodeIfPresent([NlsOption].self, forKey: .nlsOptions) ?? []
    }

    var isSiglus: Bool { engine == "siglus" }

    var nlsLabel: String? {
        guard let nls else { return nil }
        return nlsOptions.first(where: { $0.id == nls })?.label ?? nls
    }
}

struct UnsupportedGame: Identifiable, Equatable {
    var id: String { path }
    let path: String
    let title: String
    let reason: String
}

final class GameLibrary: ObservableObject {
    @Published var games: [GameEntry] = []
    @Published var unsupported: [UnsupportedGame] = []
    @Published var showError: Bool = false
    @Published var errorMessage: String = ""
    @Published var isBusy: Bool = false
    @Published var notice: String? = nil

    // When non-nil, present the in-app player (iOS host-mode).
    @Published var activeGame: GameEntry? = nil

    private let fm = FileManager.default

    // MARK: - Storage (settings only)
    private var appSupportDir: URL {
        let base = fm.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let dir = base.appendingPathComponent("SiglusLauncher", isDirectory: true)
        if !fm.fileExists(atPath: dir.path) {
            try? fm.createDirectory(at: dir, withIntermediateDirectories: true)
        }
        return dir
    }

    private var coverCacheDir: URL {
        appSupportDir.appendingPathComponent("covers", isDirectory: true)
    }

    // Games live in Documents/siglus so the user can copy folders in via the Files app.
    private var documentsDir: URL {
        fm.urls(for: .documentDirectory, in: .userDomainMask).first!
    }

    var documentsGamesDir: URL {
        let dir = documentsDir.appendingPathComponent("siglus", isDirectory: true)
        if !fm.fileExists(atPath: dir.path) {
            try? fm.createDirectory(at: dir, withIntermediateDirectories: true)
        }
        return dir
    }

    private var libraryURL: URL {
        appSupportDir.appendingPathComponent("library.json")
    }

    init() {
        // Ensure the Files-visible folder exists as early as possible.
        _ = documentsGamesDir
        load()
    }

    func load() {
        do {
            if fm.fileExists(atPath: libraryURL.path) {
                let data = try Data(contentsOf: libraryURL)
                games = try JSONDecoder().decode([GameEntry].self, from: data)
            } else {
                games = []
            }
        } catch {
            games = []
        }
        // Always rebuild the list from Documents/siglus.
        rescanFromDocuments()
    }

    func save() {
        do {
            let data = try JSONEncoder().encode(games)
            try data.write(to: libraryURL, options: [.atomic])
        } catch {
            // best-effort
        }
    }

    // MARK: - Scan games in Documents/siglus (any depth up to three folders)

    /// Rebuilds the library.  `announceNew` reports games that were not in
    /// the library before, and every unsupported folder.
    func rescanFromDocuments(announceNew: Bool = false) {
        isBusy = true
        let root = documentsGamesDir.path
        let cache = coverCacheDir.path
        let previous = games
        DispatchQueue.global(qos: .userInitiated).async {
            let json = root.withCString { path in
                cache.withCString { takeString(game_scan_json(path, 3, $0)) }
            }
            let probes = json.flatMap { try? JSONDecoder().decode([ProbedGame].self, from: Data($0.utf8)) } ?? []
            DispatchQueue.main.async {
                self.applyScan(probes, previous: previous, announceNew: announceNew)
            }
        }
    }

    private func applyScan(_ probes: [ProbedGame], previous: [GameEntry], announceNew: Bool) {
        let savedById = Dictionary(previous.map { ($0.id, $0) }, uniquingKeysWith: { a, _ in a })
        let now = Int64(Date().timeIntervalSince1970)
        var out: [GameEntry] = []
        var skipped: [UnsupportedGame] = []
        var added: [String] = []
        for probe in probes {
            guard probe.supported else {
                skipped.append(UnsupportedGame(path: probe.root, title: probe.title, reason: probe.unsupported_reason ?? "Unsupported engine"))
                continue
            }
            let id = probe.id ?? stableId(for: probe.root)
            let saved = savedById[id]
            var entry = GameEntry(
                id: id,
                title: probe.title,
                rootPath: probe.root,
                addedAtUnix: saved?.addedAtUnix ?? now,
                lastPlayedAtUnix: saved?.lastPlayedAtUnix,
                coverPath: probe.cover,
                engine: probe.engine,
                engineName: probe.engine_name ?? probe.engine,
                coverKind: probe.cover_kind,
                nls: probe.nls,
                nlsOptions: probe.nls_options ?? []
            )
            if let savedNls = saved?.nls, entry.nlsOptions.contains(where: { $0.id == savedNls }) {
                entry.nls = savedNls
                if savedNls != probe.nls { refreshTitle(&entry) }
            }
            if saved == nil { added.append(entry.title) }
            out.append(entry)
        }

        // Recently played first, then newest.
        out.sort { a, b in
            let ap = a.lastPlayedAtUnix ?? 0
            let bp = b.lastPlayedAtUnix ?? 0
            if ap != bp { return ap > bp }
            return a.addedAtUnix > b.addedAtUnix
        }

        games = out
        unsupported = skipped
        isBusy = false
        save()

        if announceNew {
            var lines: [String] = []
            if !added.isEmpty {
                lines.append("Added: " + added.joined(separator: ", "))
            }
            for game in skipped {
                lines.append("Not supported: \(game.title) — \(game.reason)")
            }
            if lines.isEmpty {
                lines.append(probes.isEmpty
                    ? "No game was found. Each game needs its own folder with its files (Gameexe.ini, Scene.pck, SEEN.TXT or UK2.CFG)."
                    : "No new games.")
            }
            notice = lines.joined(separator: "\n\n")
        }
    }

    private func refreshTitle(_ entry: inout GameEntry) {
        let json = entry.rootPath.withCString { root -> String? in
            guard let nls = entry.nls else { return takeString(game_probe_json(root, nil, nil)) }
            return nls.withCString { takeString(game_probe_json(root, $0, nil)) }
        }
        if let json, let probe = try? JSONDecoder().decode(ProbedGame.self, from: Data(json.utf8)) {
            entry.title = probe.title
        }
    }

    // MARK: - Import from Files (copies into Documents/siglus)

    func importFolders(_ urls: [URL]) {
        guard !urls.isEmpty else { return }
        isBusy = true
        let destinationRoot = documentsGamesDir
        DispatchQueue.global(qos: .userInitiated).async {
            var failures: [String] = []
            for url in urls {
                let scoped = url.startAccessingSecurityScopedResource()
                defer { if scoped { url.stopAccessingSecurityScopedResource() } }
                var target = destinationRoot.appendingPathComponent(url.lastPathComponent, isDirectory: true)
                var suffix = 2
                while self.fm.fileExists(atPath: target.path) {
                    target = destinationRoot.appendingPathComponent("\(url.lastPathComponent) \(suffix)", isDirectory: true)
                    suffix += 1
                }
                do {
                    try self.fm.copyItem(at: url, to: target)
                } catch {
                    failures.append("\(url.lastPathComponent): \(error.localizedDescription)")
                }
            }
            DispatchQueue.main.async {
                if !failures.isEmpty {
                    self.showError("Some folders could not be copied:\n" + failures.joined(separator: "\n"))
                }
                self.rescanFromDocuments(announceNew: true)
            }
        }
    }

    func setNls(_ nls: String, for game: GameEntry) {
        guard let idx = games.firstIndex(where: { $0.id == game.id }) else { return }
        games[idx].nls = nls
        refreshTitle(&games[idx])
        save()
    }

    func remove(game: GameEntry) {
        // Remove from library and delete the game folder (Documents/siglus/...)
        games.removeAll { $0.id == game.id }
        save()

        // Best-effort: remove the folder pointed by rootPath.
        let root = URL(fileURLWithPath: game.rootPath)
        try? fm.removeItem(at: root)
    }

    // MARK: - Launch
    func launch(game: GameEntry) {
        if let idx = games.firstIndex(where: { $0.id == game.id }) {
            games[idx].lastPlayedAtUnix = Int64(Date().timeIntervalSince1970)
            save()
        }
        // Present the in-app player (SwiftUI owns the main loop).
        activeGame = game
    }

    // MARK: - Helpers
    func loadCoverImage(game: GameEntry) -> UIImage? {
        guard let coverPath = game.coverPath, !coverPath.isEmpty else { return nil }
        return UIImage(contentsOfFile: coverPath)
    }

    private func stableId(for path: String) -> String {
        var hash: UInt64 = 0xcbf2_9ce4_8422_2325
        for byte in path.utf8 {
            hash ^= UInt64(byte)
            hash = hash &* 0x0100_0000_01b3
        }
        return String(format: "%016llx", hash)
    }

    func showError(_ msg: String) {
        errorMessage = msg
        showError = true
    }
}
