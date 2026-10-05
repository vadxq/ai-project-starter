import Foundation

struct Configuration: Sendable {
    let environment: String
    let apiURL: URL

    enum Failure: Error { case missing(String), invalidURL(String), insecureURL(String) }

    init(bundle: Bundle) throws {
        guard let environment = bundle.object(forInfoDictionaryKey: "AppEnvironment") as? String, !environment.isEmpty else {
            throw Failure.missing("AppEnvironment")
        }
        guard let value = bundle.object(forInfoDictionaryKey: "APIBaseURL") as? String,
              let apiURL = URL(string: value), apiURL.host != nil else { throw Failure.invalidURL("APIBaseURL") }
        self.environment = environment
        self.apiURL = apiURL
        #if DEBUG
        let localHTTPAllowed: Bool = environment == "dev" && ["localhost", "127.0.0.1"].contains(apiURL.host ?? "")
        #else
        let localHTTPAllowed: Bool = false
        #endif
        if apiURL.scheme != "https" && !(apiURL.scheme == "http" && localHTTPAllowed) {
            throw Failure.insecureURL(apiURL.host ?? "")
        }
    }
}
