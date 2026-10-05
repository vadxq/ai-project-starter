import type { JSX } from 'react';
import { Link, Route, Routes } from 'react-router-dom';
import { useAuth } from './auth/provider';
import { Preferences } from './components/preferences';
import { SignIn } from './components/sign-in';
import { Items } from './features/items';
import { useCopy } from './i18n';

export function Workspace(): JSX.Element {
  const { t } = useCopy();
  const { user, busy, signOut } = useAuth();
  return (
    <div className="shell">
      <header className="app-header">
        <a className="wordmark" href="/items">
          {t('app')}
          <span aria-hidden="true">.</span>
        </a>
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

function NotFound(): JSX.Element {
  const { t } = useCopy();
  return (
    <main className="shell">
      <h1>{t('notFound')}</h1>
      <Link to="/items">{t('back')}</Link>
    </main>
  );
}

export function App(): JSX.Element {
  return (
    <Routes>
      <Route path="/" element={<Workspace />} />
      <Route path="/items" element={<Workspace />} />
      <Route path="*" element={<NotFound />} />
    </Routes>
  );
}
