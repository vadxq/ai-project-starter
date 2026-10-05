import APIClient
import Foundation
import HTTPTypes

typealias Item = Components.Schemas.Item

struct APIError: Error {
    let code: String
    let status: Int
    let requestID: String?
}

struct BearerMiddleware: ClientMiddleware {
    let token: @Sendable () async throws -> String
    let locale: @Sendable () async -> String
    let unauthorized: @Sendable (String) async throws -> Void

    func intercept(_ request: HTTPRequest, body: HTTPBody?, baseURL: URL, operationID: String,
        next: @Sendable (HTTPRequest, HTTPBody?, URL) async throws -> (HTTPResponse, HTTPBody?)) async throws -> (HTTPResponse, HTTPBody?) {
        var authenticated: HTTPRequest = request
        let current: String = try await token()
        authenticated.headerFields[.authorization] = "Bearer \(current)"
        authenticated.headerFields[.acceptLanguage] = await locale()
        let (response, body) = try await next(authenticated, body, baseURL)
        if response.status.code == 401 { try await unauthorized(current) }
        if response.status.code >= 400 {
            guard let body else { throw APIError(code: "unknown_error", status: response.status.code, requestID: nil) }
            let data: Data = try await Data(collecting: body, upTo: 64 * 1024)
            let problem: Components.Schemas.Problem = try JSONDecoder().decode(Components.Schemas.Problem.self, from: data)
            throw APIError(code: problem.code, status: response.status.code, requestID: problem.requestId)
        }
        return (response, body)
    }
}

struct ItemService: Sendable {
    let client: Client

    @MainActor init(config: Configuration, auth: AuthSession, preferences: Preferences) {
        client = Client(serverURL: config.apiURL, configuration: .init(dateTranscoder: .iso8601WithFractionalSeconds), transport: URLSessionTransport(), middlewares: [
            BearerMiddleware(token: { try await auth.accessToken() }, locale: { await preferences.apiLocale }, unauthorized: { try await auth.reject(token: $0) }),
        ])
    }

    func me() async throws -> Components.Schemas.Identity {
        try await client.getMe().ok.body.json
    }

    func list(offset: Int) async throws -> Components.Schemas.ItemPage {
        try await client.listItems(query: .init(limit: 20, offset: Int64(offset))).ok.body.json
    }

    func create(title: String) async throws {
        _ = try await client.createItem(body: .json(.init(completed: false, title: title))).created
    }

    func update(item: Item, title: String?, completed: Bool?) async throws {
        _ = try await client.updateItem(path: .init(id: item.id), body: .json(.init(
            completed: completed, title: title, version: item.version))).ok
    }

    func delete(item: Item) async throws {
        _ = try await client.deleteItem(path: .init(id: item.id), query: .init(version: item.version)).noContent
    }
}
