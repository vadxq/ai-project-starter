import type { JSX, ReactNode } from 'react';
import { hasLocale, NextIntlClientProvider } from 'next-intl';
import { notFound } from 'next/navigation';
import { routing } from '../../intl/routing';
import en from '../../locales/en.json';
import zh from '../../locales/zh-CN.json';

export default async function LocaleLayout({
  children,
  params,
}: {
  children: ReactNode;
  params: Promise<{ locale: string }>;
}): Promise<JSX.Element> {
  const { locale } = await params;
  if (!hasLocale(routing.locales, locale)) notFound();
  return (
    <NextIntlClientProvider locale={locale} messages={locale === 'zh-CN' ? zh : en}>
      {children}
    </NextIntlClientProvider>
  );
}
