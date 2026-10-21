'use client';

import { useLocale, useTranslations, hasLocale } from 'next-intl';
import { useTransition, type ChangeEvent, type ReactNode } from 'react';
import { localeNames, routing } from '~/lib/i18n/routing';
import { usePathname, useRouter } from '~/lib/i18n/navigation';

export function LocaleSwitcher(): ReactNode {
  const locale = useLocale();
  const t = useTranslations('navigation');
  const pathname = usePathname();
  const router = useRouter();
  const [pending, startTransition] = useTransition();

  function changeLocale(event: ChangeEvent<HTMLSelectElement>): void {
    const nextLocale = event.target.value;
    if (!hasLocale(routing.locales, nextLocale)) return;
    // 语言切换保留路由、搜索参数和 hash。
    const href = pathname + window.location.search + window.location.hash;
    startTransition(() => router.replace(href, { locale: nextLocale, scroll: false }));
  }

  return (
    <select
      aria-label={t('language')}
      value={locale}
      disabled={pending}
      onChange={changeLocale}
      className="h-9 max-w-24 rounded-md border border-input bg-background px-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:opacity-50"
    >
      {routing.locales.map((value) => (
        <option key={value} value={value}>
          {localeNames[value]}
        </option>
      ))}
    </select>
  );
}
