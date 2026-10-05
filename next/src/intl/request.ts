import { getRequestConfig } from 'next-intl/server';
import { hasLocale } from 'next-intl';
import { routing } from './routing';
import en from '../locales/en.json';
import zh from '../locales/zh-CN.json';

export default getRequestConfig(async ({ requestLocale }) => {
  const requested: string | undefined = await requestLocale;
  const locale = hasLocale(routing.locales, requested) ? requested : routing.defaultLocale;
  return { locale, messages: locale === 'zh-CN' ? zh : en };
});
