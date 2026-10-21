// 后端采用上游 HTTP 状态字符串；业务 reason 仅用于本地化，不依赖服务端文案。
const statusCodes: Readonly<Record<number, string>> = {
  400: 'validation_error',
  401: 'unauthorized',
  403: 'forbidden',
  404: 'not_found',
  405: 'not_found',
  409: 'version_conflict',
  413: 'payload_too_large',
  415: 'validation_error',
  422: 'validation_error',
  500: 'unknown_error',
  503: 'service_unavailable',
};
const reasons: ReadonlySet<string> = new Set([
  ...Object.values(statusCodes),
  'invalid_credentials',
]);

export function serverErrorCode(error: unknown, status: number): string {
  if (typeof error === 'object' && error !== null && 'details' in error) {
    const details: unknown = error.details;
    if (
      typeof details === 'object' &&
      details !== null &&
      'reason' in details &&
      typeof details.reason === 'string' &&
      reasons.has(details.reason)
    )
      return details.reason;
  }
  return statusCodes[status] ?? 'unknown_error';
}
