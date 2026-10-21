import type { Metadata, Viewport } from 'next';
import type { ReactNode } from 'react';
import { NextIntlClientProvider, hasLocale } from 'next-intl';
import { getTranslations } from 'next-intl/server';
import { notFound } from 'next/navigation';
import { AppProviders } from '~/components/providers/AppProviders';
import { routing } from '~/lib/i18n/routing';
import { site } from '~/lib/site';
import '~/styles/globals.css';

type LayoutProps = Readonly<{ children: ReactNode; params: Promise<{ locale: string }> }>;

export async function generateMetadata({ params }: LayoutProps): Promise<Metadata> {
  const { locale } = await params;
  if (!hasLocale(routing.locales, locale)) notFound();
  const t = await getTranslations({ locale, namespace: 'metadata' });

  return {
    metadataBase: site.url,
    title: { default: t('title'), template: `%s | ${t('title')}` },
    description: t('description'),
    alternates: {
      canonical: `/${locale}`,
      languages: { en: '/en', 'zh-CN': '/zh-CN', 'x-default': '/en' },
    },
    icons: { icon: '/icons/favicon.svg', apple: '/icons/apple-touch-icon.png' },
  };
}

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  themeColor: [
    { media: '(prefers-color-scheme: light)', color: '#f8f9f5' },
    { media: '(prefers-color-scheme: dark)', color: '#191d18' },
  ],
};

export function generateStaticParams(): { locale: string }[] {
  return routing.locales.map((locale) => ({ locale }));
}

export default async function LocaleLayout({ children, params }: LayoutProps): Promise<ReactNode> {
  const { locale } = await params;
  if (!hasLocale(routing.locales, locale)) notFound();

  return (
    <html lang={locale} suppressHydrationWarning>
      <body>
        <NextIntlClientProvider>
          <AppProviders>{children}</AppProviders>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
