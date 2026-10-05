import APIClient
import Foundation
import Observation

@MainActor @Observable
final class AuthSession {
    private(set) var signedIn: Bool = false
    private(set) var busy: Bool = false
    private(set) var errorCode: String?
    private var refreshToken: String?
    private var access: String?
    private var expiresAt: Date = .distantPast
    private var refreshing: Task<Components.Schemas.TokenResponse, any Error>?
    private let config: Configuration
    private let service: String
    private var generation: Int = 0

    enum Failure: Error { case unauthorized, invalidResponse }

    init(config: Configuration) throws {
        self.config = config
        service = "starter.jwt.\(config.environment)"
        if let data = try KeychainStore.read(service: service) {
            refreshToken = String(data: data, encoding: .utf8)
            signedIn = refreshToken != nil
        }
    }

    private func request(path: String, body: Data) async throws -> Data {
        var request = URLRequest(url: config.apiURL.appendingPathComponent(path))
        request.httpMethod = "POST"
        request.timeoutInterval = 10
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue(Locale.current.language.languageCode?.identifier == "zh" ? "zh-CN" : "en", forHTTPHeaderField: "Accept-Language")
        request.httpBody = body
        let (data, response) = try await URLSession.shared.data(for: request)
        guard let response = response as? HTTPURLResponse else { throw Failure.invalidResponse }
        if response.statusCode >= 400 {
            let problem = try JSONDecoder().decode(Components.Schemas.Problem.self, from: data)
            throw APIError(code: problem.code, status: response.statusCode, requestID: problem.requestId)
        }
        return data
    }

    private func token(path: String, body: Data) async throws -> Components.Schemas.TokenResponse {
        try JSONDecoder().decode(Components.Schemas.TokenResponse.self, from: await request(path: path, body: body))
    }

    private func accept(_ session: Components.Schemas.TokenResponse, started: Int) throws {
        guard generation == started, !Task.isCancelled else { throw Failure.unauthorized }
        try KeychainStore.save(data: Data(session.refreshToken.utf8), service: service)
        refreshToken = session.refreshToken
        access = session.accessToken
        expiresAt = Date().addingTimeInterval(TimeInterval(session.expiresIn))
        signedIn = true
    }

    private func report(_ error: any Error) {
        if let api = error as? APIError { errorCode = api.code }
        else if error is URLError { errorCode = "network_error" }
        else if error is Failure { errorCode = "unauthorized" }
        else { errorCode = "auth_error" }
    }

    func signIn(username: String, password: String) async {
        guard !busy else { return }
        busy = true
        errorCode = nil
        let started: Int = generation
        defer { busy = false }
        do {
            let body = try JSONEncoder().encode(Components.Schemas.LoginRequest(password: password, username: username))
            try accept(try await token(path: "api/v1/auth/login", body: body), started: started)
        } catch { if !Task.isCancelled { report(error) } }
    }

    func accessToken() async throws -> String {
        if let access, expiresAt.timeIntervalSinceNow > 30 { return access }
        guard let refreshToken else { throw Failure.unauthorized }
        let started: Int = generation
        let task: Task<Components.Schemas.TokenResponse, any Error>
        if let refreshing { task = refreshing }
        else {
            task = Task {
                let body = try JSONEncoder().encode(Components.Schemas.RefreshRequest(refreshToken: refreshToken))
                return try await self.token(path: "api/v1/auth/refresh", body: body)
            }
            refreshing = task
        }
        defer { refreshing = nil }
        do {
            try accept(try await task.value, started: started)
            guard let access else { throw Failure.unauthorized }
            return access
        } catch {
            if generation == started, let api = error as? APIError, api.status == 401 { try invalidate() }
            throw error
        }
    }

    func invalidate() throws {
        generation += 1
        try KeychainStore.remove(service: service)
        refreshToken = nil
        access = nil
        signedIn = false
    }

    func reject(token: String) throws {
        if access == token { try invalidate() }
    }

    func signOut() async {
        busy = true
        errorCode = nil
        defer { busy = false }
        do {
            if let refreshing {
                do { try accept(try await refreshing.value, started: generation) }
                catch { if let api = error as? APIError, api.status == 401 { try invalidate() } else { throw error } }
            }
            let previous: String? = refreshToken
            try invalidate()
            if let previous {
                let body = try JSONEncoder().encode(Components.Schemas.RefreshRequest(refreshToken: previous))
                _ = try await request(path: "api/v1/auth/logout", body: body)
            }
        } catch {
            do { try invalidate() } catch { report(error); return }
            report(error)
        }
    }
}
