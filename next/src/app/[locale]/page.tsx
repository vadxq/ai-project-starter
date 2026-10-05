import type { Metadata } from 'next';
import type { JSX } from 'react';
import { getTranslations } from 'next-intl/server';
import { notFound } from 'next/navigation';
import { hasLocale } from 'next-intl';
import { routing } from '../../intl/routing';
import { Preferences } from '../../components/preferences';
import { SignIn } from '../../components/sign-in';

interface PageProps {
  params: Promise<{ locale: string }>;
}

export async function generateMetadata({ params }: PageProps): Promise<Metadata> {
  const { locale } = await params;
  const t = await getTranslations({ locale });
  return { title: t('app'), description: t('welcomeBody') };
}

export default async function Home({ params }: PageProps): Promise<JSX.Element> {
  const { locale } = await params;
  if (!hasLocale(routing.locales, locale)) notFound();
  const t = await getTranslations({ locale });
  return (
    <div className="shell">
      <header className="app-header">
        <span className="wordmark">{t('app')}.</span>
        <Preferences />
      </header>
      <main className="welcome">
        <h1>{t('welcome')}</h1>
        <p>{t('welcomeBody')}</p>
        <SignIn />
      </main>
    </div>
  );
}
