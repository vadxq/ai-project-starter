import type {
  Identity,
  LoginRequest,
  RefreshRequest,
  TokenResponse,
} from '../api/generated/types.gen';
import { readConfig } from '../config';

const REFRESH_KEY: string = 'starter.refresh';
const REFRESH_MARGIN_MS: number = 30_000;
const REQUEST_TIMEOUT_MS: number = 10_000;

export class AuthFailure extends Error {
  readonly status: number;
  constructor(details: { code: string; status: number }) {
    super(details.code);
    this.name = 'AuthFailure';
    this.status = details.status;
  }
}

type Listener = (user: Identity | null) => void;

// 认证连接器负责网络与会话；access token 只在内存，refresh token 随当前标签页存活。
class TokenSession {
  private access: string | null = null;
  private expiresAt: number = 0;
  private user: Identity | null = null;
  private refreshing: Promise<Identity> | null = null;
  private generation: number = 0;
  private readonly listeners: Set<Listener> = new Set();

  subscribe(listener: Listener): () => void {
    this.listeners.add(listener);
    return (): void => {
      this.listeners.delete(listener);
    };
  }

  clear(): void {
    this.generation += 1;
    this.access = null;
    this.expiresAt = 0;
    this.user = null;
    sessionStorage.removeItem(REFRESH_KEY);
    for (const listener of this.listeners) listener(null);
  }

  reject(token: string): void {
    if (this.access === token) this.clear();
  }

  private async request(path: string, body: LoginRequest | RefreshRequest): Promise<Response> {
    const response: Response = await fetch(new URL(path, readConfig().apiBaseUrl), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept-Language': document.documentElement.lang,
      },
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS),
      cache: 'no-store',
    });
    if (!response.ok) {
      const problem: unknown = await response.json();
      const code: string =
        typeof problem === 'object' &&
        problem !== null &&
        'code' in problem &&
        typeof problem.code === 'string'
          ? problem.code
          : 'unknown_error';
      throw new AuthFailure({ code, status: response.status });
    }
    return response;
  }

  private accept(session: TokenResponse, started: number): Identity {
    if (started !== this.generation) throw new AuthFailure({ code: 'unauthorized', status: 401 });
    sessionStorage.setItem(REFRESH_KEY, session.refreshToken);
    this.access = session.accessToken;
    this.expiresAt = Date.now() + session.expiresIn * 1000;
    this.user = session.user;
    for (const listener of this.listeners) listener(this.user);
    return session.user;
  }

  async signIn(input: LoginRequest): Promise<Identity> {
    const started: number = ++this.generation;
    const response: Response = await this.request('/api/v1/auth/login', input);
    const session: TokenResponse = (await response.json()) as TokenResponse;
    return this.accept(session, started);
  }

  async restore(): Promise<Identity | null> {
    if (this.user && this.access && Date.now() < this.expiresAt - REFRESH_MARGIN_MS)
      return this.user;
    if (!sessionStorage.getItem(REFRESH_KEY)) return null;
    return this.refresh();
  }

  private refresh(): Promise<Identity> {
    if (this.refreshing) return this.refreshing;
    const token: string | null = sessionStorage.getItem(REFRESH_KEY);
    if (!token) return Promise.reject(new AuthFailure({ code: 'unauthorized', status: 401 }));
    const started: number = this.generation;
    this.refreshing = this.request('/api/v1/auth/refresh', { refreshToken: token })
      .then(async (response: Response): Promise<Identity> =>
        this.accept((await response.json()) as TokenResponse, started),
      )
      .catch((error: unknown): never => {
        if (started === this.generation && error instanceof AuthFailure && error.status === 401)
          this.clear();
        throw error;
      })
      .finally((): void => {
        this.refreshing = null;
      });
    return this.refreshing;
  }

  async accessToken(): Promise<string> {
    if (!this.access || Date.now() >= this.expiresAt - REFRESH_MARGIN_MS) await this.refresh();
    if (!this.access) throw new AuthFailure({ code: 'unauthorized', status: 401 });
    return this.access;
  }

  async signOut(): Promise<void> {
    // 等待正在轮换的请求，确保撤销最新 refresh token，然后立即清空本地状态。
    if (this.refreshing) {
      try {
        await this.refreshing;
      } catch (error: unknown) {
        if (!(error instanceof AuthFailure && error.status === 401)) {
          this.clear();
          throw error;
        }
      }
    }
    const token: string | null = sessionStorage.getItem(REFRESH_KEY);
    this.clear();
    if (token) await this.request('/api/v1/auth/logout', { refreshToken: token });
  }
}

const session: TokenSession = new TokenSession();
export function authManager(): TokenSession {
  return session;
}
export function accessToken(): Promise<string> {
  return session.accessToken();
}
export function authErrorCode(error: unknown): string {
  if (error instanceof AuthFailure) return error.message;
  if (
    error instanceof TypeError ||
    (error instanceof DOMException && error.name === 'TimeoutError')
  )
    return 'network_error';
  return 'auth_error';
}
