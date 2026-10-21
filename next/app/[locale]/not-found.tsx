import type { ReactNode } from 'react';
import { getTranslations } from 'next-intl/server';
import { Link } from '~/lib/i18n/navigation';
import { Button } from '~/components/ui/button';

export default async function NotFound(): Promise<ReactNode> {
  const t = await getTranslations('notFound');
  return (
    <main className="page-container flex min-h-svh flex-col items-start justify-center py-16">
      <span className="mb-5 text-sm text-primary">404</span>
      <h1 className="text-3xl font-semibold">{t('title')}</h1>
      <p className="mt-4 mb-8 text-muted-foreground">{t('description')}</p>
      <Button asChild>
        <Link href="/">{t('home')}</Link>
      </Button>
    </main>
  );
}
