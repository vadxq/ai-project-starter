package com.example.starter.network

/** 上游 code 是 HTTP 状态字符串，界面只依赖稳定 reason 或状态语义。 */
private val serverReasons: Set<String> = setOf(
    "validation_error", "unauthorized", "invalid_credentials", "forbidden", "not_found",
    "version_conflict", "payload_too_large", "service_unavailable", "unknown_error",
)

fun serverErrorCode(status: Int, reason: String?): String = reason?.takeIf { it in serverReasons } ?: when (status) {
    400, 415, 422 -> "validation_error"
    401 -> "unauthorized"
    403 -> "forbidden"
    404, 405 -> "not_found"
    409 -> "version_conflict"
    413 -> "payload_too_large"
    503 -> "service_unavailable"
    else -> "unknown_error"
}
