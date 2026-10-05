'use client';

import type { JSX } from 'react';
import Link from 'next/link';
import { useAuth } from '../auth/provider';
import { useCopy } from '../i18n';
import { Preferences } from '../components/preferences';
import { SignIn } from '../components/sign-in';
import { Items } from './items';

export function Workspace(): JSX.Element {
  const { t, locale } = useCopy();
  const { user, busy, signOut } = useAuth();
  return (
    <div className="shell">
      <header className="app-header">
        <Link className="wordmark" href={`/${locale}`}>
          {t('app')}.
        </Link>
        <Preferences />
      </header>
      <main>
        <div className="workspace-heading">
          <h1>{t('app')}</h1>
          {user && (
            <div className="account">
              <span>{user.username}</span>
              <button
                disabled={busy}
                onClick={(): void => {
                  void signOut();
                }}
              >
                {t('signOut')}
              </button>
            </div>
          )}
        </div>
        {user ? (
          <Items />
        ) : (
          <section className="welcome">
            <h2>{t('welcome')}</h2>
            <p>{t('welcomeBody')}</p>
            <SignIn />
          </section>
        )}
      </main>
    </div>
  );
}
