export const site = {
  name: 'Items',
  url: new URL(process.env.NEXT_PUBLIC_APP_URL ?? 'http://localhost:3000'),
  repository: 'https://github.com/vadxq/nextjs-ai-starter',
} as const;
