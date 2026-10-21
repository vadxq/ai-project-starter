import type { MetadataRoute } from 'next';
import { routing } from '~/lib/i18n/routing';
import { site } from '~/lib/site';

export default function sitemap(): MetadataRoute.Sitemap {
  const languages: Record<string, string> = Object.fromEntries(
    routing.locales.map((locale) => [locale, new URL(`/${locale}`, site.url).href]),
  );
  return routing.locales.map((locale) => ({
    url: new URL(`/${locale}`, site.url).href,
    alternates: { languages },
  }));
}
