import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { App } from './app';
import { AuthProvider } from './auth/provider';
import { initializeLocale } from './i18n';
import '@fontsource-variable/dm-sans';
import './styles.css';

const client: QueryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: 15_000, retry: 1 }, mutations: { retry: false } },
});
await initializeLocale();
createRoot(document.getElementById('root')!).render(
  <QueryClientProvider client={client}>
    <BrowserRouter>
      <AuthProvider>
        <App />
      </AuthProvider>
    </BrowserRouter>
  </QueryClientProvider>,
);
