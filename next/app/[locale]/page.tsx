import type { ReactNode } from 'react';
import { getTranslations } from 'next-intl/server';
import { Header } from '~/components/layout/Header';
import { SignIn } from '~/components/auth/SignIn';

export default async function Home(): Promise<ReactNode> {
  const t = await getTranslations();
  return (
    <>
      <Header />
      <main id="main" className="page-container py-16 sm:py-24">
        <section className="enter max-w-sm">
          <h1 className="mb-8 text-3xl font-semibold tracking-tight">{t('signIn')}</h1>
          <SignIn />
        </section>
      </main>
    </>
  );
}
