'use client';

import { useState, type ReactNode } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ThemeProvider } from 'next-themes';
import { AuthProvider } from '~/lib/auth/provider';
import { PreferencesProvider } from './PreferencesProvider';

export function AppProviders({ children }: { children: ReactNode }): ReactNode {
  const [client] = useState<QueryClient>(
    () =>
      new QueryClient({
        defaultOptions: { queries: { staleTime: 15_000, retry: 1 }, mutations: { retry: false } },
      }),
  );
  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem disableTransitionOnChange>
      <PreferencesProvider>
        <QueryClientProvider client={client}>
          <AuthProvider>{children}</AuthProvider>
        </QueryClientProvider>
      </PreferencesProvider>
    </ThemeProvider>
  );
}
