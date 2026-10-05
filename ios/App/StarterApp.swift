import OSLog
import SwiftUI

@MainActor
struct AppDependencies {
    let auth: AuthSession
    let preferences: Preferences
    let items: ItemsModel

    init() throws {
        let config = try Configuration(bundle: .main)
        let preferences = Preferences(defaults: .standard)
        let auth = try AuthSession(config: config)
        self.auth = auth
        self.preferences = preferences
        items = ItemsModel(service: ItemService(config: config, auth: auth, preferences: preferences))
    }
}

@main
struct StarterApp: App {
    private let dependencies: AppDependencies?

    init() {
        do { dependencies = try AppDependencies() }
        catch {
            Logger(subsystem: "starter", category: "configuration").error("Application initialization failed: \(String(reflecting: error), privacy: .public)")
            dependencies = nil
        }
    }

    var body: some Scene {
        WindowGroup {
            if let dependencies {
                ContentView(auth: dependencies.auth, preferences: dependencies.preferences, model: dependencies.items)
                    .environment(\.locale, dependencies.preferences.locale)
                    .preferredColorScheme(dependencies.preferences.colorScheme)
                    .tint(Color(red: 0.17, green: 0.43, blue: 0.29))
            } else {
                ContentUnavailableView("startup_error", systemImage: "exclamationmark.triangle")
            }
        }
    }
}
