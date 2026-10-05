export interface PublicConfig {
  apiBaseUrl: string;
}

export function readConfig(): PublicConfig {
  const config: PublicConfig = { apiBaseUrl: process.env.NEXT_PUBLIC_API_BASE_URL ?? '' };
  if (!config.apiBaseUrl) throw new Error('Missing public configuration: apiBaseUrl');
  const parsed: URL = new URL(config.apiBaseUrl);
  if (
    parsed.protocol !== 'https:' &&
    !(parsed.protocol === 'http:' && ['localhost', '127.0.0.1'].includes(parsed.hostname))
  ) {
    throw new Error('API endpoint requires HTTPS outside localhost');
  }
  return config;
}
