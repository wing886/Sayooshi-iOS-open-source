import SwiftUI

@main
struct SiglusLauncherApp: App {
    @StateObject private var library = GameLibrary()

    init() {
        registerSystemFontsForGames()
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(library)
        }
    }
}
