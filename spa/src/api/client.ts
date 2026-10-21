import { createClient, createConfig } from './generated/client';
import { accessToken, authManager } from '../auth/manager';
import { readConfig } from '../config';
import { serverErrorCode } from './problem';

export class ApiFailure extends Error {
  readonly code: string;
  readonly status: number;
  readonly requestId: string | null;

  constructor(details: { code: string; status: number; requestId: string | null }) {
    super(details.code);
    this.name = 'ApiFailure';
    this.code = details.code;
    this.status = details.status;
    this.requestId = details.requestId;
  }
}

export const api = createClient(
  createConfig({
    auth: accessToken,
    throwOnError: true,
  }),
);

api.interceptors.request.use((request: Request): Request => {
  request.headers.set('Accept-Language', document.documentElement.lang);
  return request;
});
api.interceptors.error.use(
  (error: unknown, response: Response | undefined, request: Request | undefined): ApiFailure => {
    if (!response) return new ApiFailure({ code: 'network_error', status: 0, requestId: null });
    if (response.status === 401)
      authManager().reject(request?.headers.get('Authorization')?.slice(7) ?? '');
    const code: string = serverErrorCode(error, response.status);
    return new ApiFailure({
      code,
      status: response.status,
      requestId: response.headers.get('x-request-id'),
    });
  },
);

export function apiOptions(): { client: typeof api; baseUrl: string; throwOnError: true } {
  return { client: api, baseUrl: readConfig().apiBaseUrl, throwOnError: true };
}

export function errorCode(error: unknown): string {
  if (error instanceof ApiFailure) return error.code;
  if (error instanceof Error && error.message === 'unauthorized') return 'unauthorized';
  if (error instanceof TypeError) return 'network_error';
  return 'unknown_error';
}
