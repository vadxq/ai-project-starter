'use client';

import { useLocale, useTranslations } from 'next-intl';
import { usePathname, useRouter } from 'next/navigation';
import en from '~/locales/en.json';

export type Locale = 'en' | 'zh-CN';
type CopyKey = {
  [K in keyof typeof en]: (typeof en)[K] extends string ? K : never;
}[keyof typeof en];
type Copy = {
  t: (key: string, values?: Record<string, string | number>) => string;
  locale: Locale;
  setLocale: (locale: Locale) => void;
};

export function useCopy(): Copy {
  const translate = useTranslations();
  const locale: Locale = useLocale() === 'zh-CN' ? 'zh-CN' : 'en';
  const pathname = usePathname();
  const router = useRouter();
  const t = (key: string, values?: Record<string, string | number>): string =>
    translate(
      (key in en && typeof en[key as keyof typeof en] === 'string'
        ? key
        : 'unknown_error') as CopyKey,
      values,
    );
  const setLocale = (next: Locale): void => {
    document.documentElement.lang = next;
    router.replace(
      `/${next}${pathname.replace(/^\/(en|zh-CN)(?=\/|$)/, '')}${window.location.search}${window.location.hash}`,
    );
  };
  return { t, locale, setLocale };
}
