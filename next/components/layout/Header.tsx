import type { ReactNode } from 'react';
import { getTranslations } from 'next-intl/server';
import { LocaleSwitcher } from '~/components/LocaleSwitcher';
import { ThemeToggle } from '~/components/ThemeToggle';
import { Link } from '~/lib/i18n/navigation';

export async function Header(): Promise<ReactNode> {
  const t = await getTranslations();
  return (
    <header className="border-b border-border">
      <a className="skip-link" href="#main">
        {t('navigation.skip')}
      </a>
      <div className="page-container flex min-h-20 items-center justify-between gap-3">
        <Link href="/" className="flex min-w-0 items-center gap-3 font-semibold tracking-tight">
          <span aria-hidden className="brand-mark">
            <span />
            <span />
            <span />
            <span />
          </span>
          <span>{t('app')}</span>
        </Link>
        <nav aria-label={t('navigation.label')} className="flex shrink-0 items-center gap-2">
          <LocaleSwitcher />
          <ThemeToggle />
        </nav>
      </div>
    </header>
  );
}
