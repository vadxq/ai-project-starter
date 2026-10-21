'use client';

import { useEffect, type ReactNode } from 'react';
import { useTranslations } from 'next-intl';
import { Button } from '~/components/ui/button';

type ErrorProps = { error: Error & { digest?: string }; reset: () => void };

export default function ErrorPage({ error, reset }: ErrorProps): ReactNode {
  const t = useTranslations('error');

  useEffect(() => {
    console.error({ event: 'route_error', message: error.message, digest: error.digest });
  }, [error]);

  return (
    <main className="page-container flex min-h-svh flex-col items-start justify-center py-16">
      <h1 className="text-3xl font-semibold">{t('title')}</h1>
      <p className="mt-4 mb-8 text-muted-foreground">{t('description')}</p>
      <Button onClick={reset}>{t('retry')}</Button>
    </main>
  );
}
