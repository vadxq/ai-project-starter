'use client';

import { useLocale, useTranslations } from 'next-intl';
import { usePathname, useRouter } from 'next/navigation';
import en from './locales/en.json';

export type Locale = 'en' | 'zh-CN';
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
    translate(key in en ? key : 'unknown_error', values);
  const setLocale = (next: Locale): void => {
    document.documentElement.lang = next;
    router.replace(`/${next}${pathname.replace(/^\/(en|zh-CN)(?=\/|$)/, '')}`);
  };
  return { t, locale, setLocale };
}
