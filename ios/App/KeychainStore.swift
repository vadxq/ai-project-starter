import Foundation
import Security

enum KeychainStore {
    enum Failure: Error { case status(OSStatus), invalidData }

    private static func query(service: String) -> [String: Any] {
        [kSecClass as String: kSecClassGenericPassword,
         kSecAttrService as String: service,
         kSecAttrAccount as String: "refresh-token"]
    }

    static func read(service: String) throws -> Data? {
        var parameters: [String: Any] = query(service: service)
        parameters[kSecReturnData as String] = true
        parameters[kSecMatchLimit as String] = kSecMatchLimitOne
        var result: CFTypeRef?
        let status: OSStatus = SecItemCopyMatching(parameters as CFDictionary, &result)
        if status == errSecItemNotFound { return nil }
        guard status == errSecSuccess else { throw Failure.status(status) }
        guard let data = result as? Data else { throw Failure.invalidData }
        return data
    }

    static func save(data: Data, service: String) throws {
        let parameters: [String: Any] = query(service: service)
        let update: OSStatus = SecItemUpdate(parameters as CFDictionary, [kSecValueData as String: data] as CFDictionary)
        if update == errSecSuccess { return }
        guard update == errSecItemNotFound else { throw Failure.status(update) }
        var insertion: [String: Any] = parameters
        insertion[kSecValueData as String] = data
        insertion[kSecAttrAccessible as String] = kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
        let status: OSStatus = SecItemAdd(insertion as CFDictionary, nil)
        guard status == errSecSuccess else { throw Failure.status(status) }
    }

    static func remove(service: String) throws {
        let status: OSStatus = SecItemDelete(query(service: service) as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else { throw Failure.status(status) }
    }
}
