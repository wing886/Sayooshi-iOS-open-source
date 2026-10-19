import Foundation
import AppKit
import SwiftUI

// -----------------------------
// Rust launcher API (game_launcher)
// -----------------------------
@_silgen_name("game_scan_json")
private func game_scan_json(_ path: UnsafePointer<CChar>, _ depth: Int32, _ coverCacheDir: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("game_probe_json")
private func game_probe_json(_ root: UnsafePointer<CChar>, _ nls: UnsafePointer<CChar>?, _ coverCacheDir: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("game_string_free")
private func game_string_free(_ ptr: UnsafeMutablePointer<CChar>?) -> Void

private func takeString(_ ptr: UnsafeMutablePointer<CChar>?) -> String? {
    guard let ptr else { return nil }
    let out = String(cString: ptr)
    game_string_free(ptr)
    return out
}

// -----------------------------
// Model
// -----------------------------
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
    /// "image" (full-bleed picture) or "icon" (generated icon card).
    var coverKind: String?
    /// Text encoding for RealLive/AVG32/UK2 (nil for SiglusEngine).
    var nls: String?
    var nlsOptions: [NlsOption]

    init(id: String, title: String, rootPath: String, addedAtUnix: Int64,
         lastPlayedAtUnix: Int64? = nil, coverPath: String? = nil,
         engine: String = "siglus", engineName: String = "SiglusEngine",
         coverKind: String? = nil, nls: String? = nil, nlsOptions: [NlsOption] = []) {
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

    init(from decoder: Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        id = try c.decode(String.self, forKey: .id)
        title = try c.decode(String.self, forKey: .title)
        rootPath = try c.decode(String.self, forKey: .rootPath)
        addedAtUnix = try c.decode(Int64.self, forKey: .addedAtUnix)
        lastPlayedAtUnix = try c.decodeIfPresent(Int64.self, forKey: .lastPlayedAtUnix)
        coverPath = try c.decodeIfPresent(String.self, forKey: .coverPath)
        // Libraries written before multi-engine support only held Siglus games.
        engine = try c.decodeIfPresent(String.self, forKey: .engine) ?? "siglus"
        engineName = try c.decodeIfPresent(String.self, forKey: .engineName) ?? "SiglusEngine"
        coverKind = try c.decodeIfPresent(String.self, forKey: .coverKind)
        nls = try c.decodeIfPresent(String.self, forKey: .nls)
        nlsOptions = try c.decodeIfPresent([NlsOption].self, forKey: .nlsOptions) ?? []
    }

    var nlsLabel: String? {
        guard let nls else { return nil }
        return nlsOptions.first(where: { $0.id == nls })?.label ?? nls
    }
}

/// Summary shown after an import.
struct ImportReport: Identifiable {
    let id = UUID()
    let imported: [String]
    let updated: [String]
    let unsupported: [(String, String)]
    let empty: [String]

    var message: String {
        var lines: [String] = []
        if !imported.isEmpty {
            lines.append("Added \(imported.count) game(s): " + imported.joined(separator: ", "))
        }
        if !updated.isEmpty {
            lines.append("Refreshed \(updated.count) game(s) already in the library.")
        }
        for (title, reason) in unsupported {
            lines.append("Not supported: \(title) — \(reason)")
        }
        for path in empty {
            lines.append("No game found in \(path). Choose the folder that contains the game files (Gameexe.ini, Scene.pck, SEEN.TXT or UK2.CFG), or a folder of such folders.")
        }
        return lines.joined(separator: "\n\n")
    }

    var title: String {
        if imported.isEmpty && updated.isEmpty {
            return "Nothing imported"
        }
        return unsupported.isEmpty && empty.isEmpty ? "Import complete" : "Import finished with notes"
    }
}

@MainActor
final class GameLibrary: ObservableObject {
    @Published var games: [GameEntry] = []
    @Published var showError: Bool = false
    @Published var errorMessage: String = ""
    @Published var importReport: ImportReport? = nil
    @Published var isImporting: Bool = false
    @Published var searchText: String = ""
    // Set by main.swift (launcher host) to receive a launch request.
    // This must only stop the modal loop; the actual game entry is called outside SwiftUI.
    var onLaunchRequest: ((GameEntry) -> Void)? = nil

    private let fm = FileManager.default

    private var supportDir: URL {
        // ~/Library/Application Support/SiglusLauncher
        let appSupport = fm.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let dir = appSupport.appendingPathComponent("SiglusLauncher", isDirectory: true)
        if !fm.fileExists(atPath: dir.path) {
            try? fm.createDirectory(at: dir, withIntermediateDirectories: true)
        }
        return dir
    }

    private var libraryURL: URL { supportDir.appendingPathComponent("library.json") }
    private var coverCacheDir: URL { supportDir.appendingPathComponent("covers", isDirectory: true) }

    var visibleGames: [GameEntry] {
        let query = searchText.trimmingCharacters(in: .whitespacesAndNewlines).lowercased()
        let sorted = games.sorted { a, b in
            let ap = a.lastPlayedAtUnix ?? 0
            let bp = b.lastPlayedAtUnix ?? 0
            if ap != bp { return ap > bp }
            return a.title.localizedStandardCompare(b.title) == .orderedAscending
        }
        if query.isEmpty { return sorted }
        return sorted.filter {
            $0.title.lowercased().contains(query) || $0.engineName.lowercased().contains(query)
        }
    }

    init() {
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
        refreshValidation()
    }

    func save() {
        do {
            let data = try JSONEncoder().encode(games)
            try data.write(to: libraryURL, options: [.atomic])
        } catch {
            // best-effort
        }
    }

    // -----------------------------
    // Import (single game, several games, or a folder of games)
    // -----------------------------
    func importGameFolders() {
        let panel = NSOpenPanel()
        panel.canChooseFiles = false
        panel.canChooseDirectories = true
        panel.allowsMultipleSelection = true
        panel.prompt = "Import"
        panel.message = "Choose game folders, or a folder that contains several games."

        panel.begin { [weak self] response in
            guard let self else { return }
            if response != .OK { return }
            let urls = panel.urls
            Task { @MainActor in self.importFolders(urls) }
        }
    }

    /// Scans each folder (and up to three levels of subfolders) for games,
    /// adds the supported ones and reports the rest.
    func importFolders(_ urls: [URL]) {
        guard !urls.isEmpty else { return }
        isImporting = true
        let cacheDir = coverCacheDir.path
        DispatchQueue.global(qos: .userInitiated).async {
            var found: [(URL, [ProbedGame])] = []
            for url in urls {
                let json = url.path.withCString { path in
                    cacheDir.withCString { cache in
                        takeString(game_scan_json(path, 3, cache))
                    }
                }
                let games = json.flatMap { try? JSONDecoder().decode([ProbedGame].self, from: Data($0.utf8)) } ?? []
                found.append((url, games))
            }
            let result = found
            Task { @MainActor in
                self.finishImport(result)
            }
        }
    }

    private func finishImport(_ found: [(URL, [ProbedGame])]) {
        isImporting = false
        let now = Int64(Date().timeIntervalSince1970)
        var imported: [String] = []
        var updated: [String] = []
        var unsupported: [(String, String)] = []
        var empty: [String] = []
        for (url, probes) in found {
            if probes.isEmpty {
                empty.append(url.path)
                continue
            }
            for probe in probes {
                guard probe.supported else {
                    unsupported.append((probe.title, probe.unsupported_reason ?? "Unsupported engine"))
                    continue
                }
                let id = probe.id ?? stableId(for: probe.root)
                if let idx = games.firstIndex(where: { $0.id == id || $0.rootPath == probe.root }) {
                    apply(probe, to: &games[idx])
                    updated.append(probe.title)
                } else {
                    var entry = GameEntry(id: id, title: probe.title, rootPath: probe.root, addedAtUnix: now)
                    apply(probe, to: &entry)
                    games.append(entry)
                    imported.append(probe.title)
                }
            }
        }
        save()
        importReport = ImportReport(imported: imported, updated: updated, unsupported: unsupported, empty: empty)
    }

    private func apply(_ probe: ProbedGame, to entry: inout GameEntry) {
        entry.title = probe.title
        entry.rootPath = probe.root
        entry.coverPath = probe.cover
        entry.coverKind = probe.cover_kind
        entry.engine = probe.engine
        entry.engineName = probe.engine_name ?? probe.engine
        entry.nlsOptions = probe.nls_options ?? []
        if entry.nls == nil || !entry.nlsOptions.contains(where: { $0.id == entry.nls }) {
            entry.nls = probe.nls
        }
    }

    /// Re-reads titles and covers (after an encoding change or file edits).
    func reprobe(game: GameEntry) {
        guard let idx = games.firstIndex(where: { $0.id == game.id }) else { return }
        let cacheDir = coverCacheDir.path
        let json = game.rootPath.withCString { root in
            cacheDir.withCString { cache in
                if let nls = game.nls {
                    return nls.withCString { takeString(game_probe_json(root, $0, cache)) }
                }
                return takeString(game_probe_json(root, nil, cache))
            }
        }
        guard let json, let probe = try? JSONDecoder().decode(ProbedGame.self, from: Data(json.utf8)) else { return }
        apply(probe, to: &games[idx])
        save()
    }

    func refreshAll() {
        refreshValidation()
        for game in games { reprobe(game: game) }
    }

    func setNls(_ nls: String, for game: GameEntry) {
        guard let idx = games.firstIndex(where: { $0.id == game.id }) else { return }
        games[idx].nls = nls
        save()
        reprobe(game: games[idx])
    }

    func refreshValidation() {
        var changed = false
        games.removeAll { g in
            var isDir: ObjCBool = false
            let ok = fm.fileExists(atPath: g.rootPath, isDirectory: &isDir) && isDir.boolValue
            if !ok { changed = true }
            return !ok
        }
        if changed { save() }
    }

    // -----------------------------
    // Launch request (blocking entry is called from main.swift)
    // -----------------------------
    func launch(game: GameEntry) {
        if let idx = games.firstIndex(where: { $0.id == game.id }) {
            games[idx].lastPlayedAtUnix = Int64(Date().timeIntervalSince1970)
            save()
        }

        guard let cb = onLaunchRequest else {
            showError("Launcher host is not ready.")
            return
        }
        cb(game)
    }

    // -----------------------------
    // UI actions
    // -----------------------------
    func remove(game: GameEntry) {
        games.removeAll { $0.id == game.id }
        save()
    }

    func revealInFinder(game: GameEntry) {
        NSWorkspace.shared.activateFileViewerSelecting([URL(fileURLWithPath: game.rootPath)])
    }

    func loadCoverImage(game: GameEntry) -> NSImage? {
        guard let p = game.coverPath, fm.fileExists(atPath: p) else { return nil }
        return NSImage(contentsOfFile: p)
    }

    // -----------------------------
    // Helpers
    // -----------------------------
    private func showError(_ msg: String) {
        errorMessage = msg
        showError = true
    }

    private func stableId(for path: String) -> String {
        // FNV-1a, stable across launches (String.hashValue is not).
        var hash: UInt64 = 0xcbf2_9ce4_8422_2325
        for byte in path.utf8 {
            hash ^= UInt64(byte)
            hash = hash &* 0x0100_0000_01b3
        }
        return String(format: "%016llx", hash)
    }
}
