import { defineRouting } from 'next-intl/routing';
import { supportedLocales } from './locales';

export const routing = defineRouting({
  locales: supportedLocales,
  defaultLocale: 'en',
  localePrefix: 'always',
});

export type Locale = (typeof routing.locales)[number];

export const localeNames: Record<Locale, string> = { en: 'English', 'zh-CN': '简体中文' };
