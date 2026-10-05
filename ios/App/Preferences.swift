import Foundation
import Observation
import SwiftUI

@MainActor @Observable
final class Preferences {
    var language: String {
        didSet { UserDefaults.standard.set(language, forKey: "language") }
    }
    var theme: String {
        didSet { UserDefaults.standard.set(theme, forKey: "theme") }
    }

    init(defaults: UserDefaults) {
        let preferred: String = Locale.preferredLanguages.first ?? "en"
        language = defaults.string(forKey: "language") ?? (preferred.hasPrefix("zh") ? "zh-Hans" : "en")
        theme = defaults.string(forKey: "theme") ?? "system"
    }

    var locale: Locale { Locale(identifier: language) }
    var apiLocale: String { language == "zh-Hans" ? "zh-CN" : "en" }
    var colorScheme: ColorScheme? { theme == "system" ? nil : (theme == "dark" ? .dark : .light) }

    func text(_ key: String) -> String {
        guard let path = Bundle.main.path(forResource: language, ofType: "lproj"), let bundle = Bundle(path: path) else {
            preconditionFailure("Missing localization catalog")
        }
        return String(localized: String.LocalizationValue(key), bundle: bundle, locale: locale)
    }
}
