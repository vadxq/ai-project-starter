import APIClient
import Foundation
import Observation

@MainActor @Observable
final class ItemsModel {
    private(set) var items: [Item] = []
    private(set) var total: Int = 0
    private(set) var offset: Int = 0
    private(set) var loading: Bool = false
    private(set) var saving: Bool = false
    private(set) var errorCode: String?
    private let service: ItemService
    private var generation: Int = 0
    private var loadSequence: Int = 0

    init(service: ItemService) { self.service = service }

    func clear() {
        generation += 1
        loadSequence += 1
        items = []
        total = 0
        offset = 0
        errorCode = nil
        loading = false
        saving = false
    }

    func load() async {
        let started: Int = generation
        loadSequence += 1
        let sequence: Int = loadSequence
        loading = true
        errorCode = nil
        defer { if generation == started && loadSequence == sequence { loading = false } }
        do {
            _ = try await service.me()
            let page = try await service.list(offset: offset)
            guard !Task.isCancelled, generation == started, loadSequence == sequence else { return }
            items = page.items
            total = Int(page.total)
        } catch {
            if generation == started && loadSequence == sequence && !Task.isCancelled { report(error) }
        }
    }

    private func report(_ error: any Error) {
        if let client = error as? ClientError { report(client.underlyingError); return }
        if let api = error as? APIError { errorCode = api.code }
        else if error is AuthSession.Failure { errorCode = "unauthorized" }
        else if error is URLError { errorCode = "network_error" }
        else { errorCode = "unknown_error" }
    }

    func create(title: String) async -> Bool {
        let normalized: String = title.trimmingCharacters(in: CharacterSet(charactersIn: " \t\r\n"))
        guard (1...240).contains(normalized.utf8.count) else { errorCode = "invalidTitle"; return false }
        return await write(action: { try await self.service.create(title: normalized) }, didSave: { self.offset = 0 })
    }

    func update(item: Item, title: String?, completed: Bool?) async -> Bool {
        if let title, !(1...240).contains(title.trimmingCharacters(in: CharacterSet(charactersIn: " \t\r\n")).utf8.count) {
            errorCode = "invalidTitle"
            return false
        }
        return await write(action: { try await self.service.update(item: item, title: title, completed: completed) }, didSave: {})
    }

    func delete(item: Item) async {
        _ = await write(action: { try await self.service.delete(item: item) }, didSave: {
            if self.items.count == 1 { self.offset = max(0, self.offset - 20) }
        })
    }

    private func write(action: () async throws -> Void, didSave: () -> Void) async -> Bool {
        guard !saving else { return false }
        let started: Int = generation
        saving = true
        errorCode = nil
        defer { if generation == started { saving = false } }
        do {
            try await action()
            guard generation == started, !Task.isCancelled else { return false }
            didSave()
            await load()
            return true
        } catch {
            if generation == started && !Task.isCancelled { report(error) }
            return false
        }
    }

    func page(forward: Bool) async {
        offset = max(0, offset + (forward ? 20 : -20))
        await load()
    }
}
