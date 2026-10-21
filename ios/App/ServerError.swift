import Foundation

private let serverReasons: Set<String> = [
    "validation_error", "unauthorized", "invalid_credentials", "forbidden", "not_found",
    "version_conflict", "payload_too_large", "service_unavailable", "unknown_error",
]

// 上游响应中的 reason 是业务本地化键，未提供时按 HTTP 状态映射。
func serverErrorCode(status: Int, reason: String?) -> String {
    if let reason, serverReasons.contains(reason) { return reason }
    switch status {
    case 400, 415, 422: return "validation_error"
    case 401: return "unauthorized"
    case 403: return "forbidden"
    case 404, 405: return "not_found"
    case 409: return "version_conflict"
    case 413: return "payload_too_large"
    case 503: return "service_unavailable"
    default: return "unknown_error"
    }
}
