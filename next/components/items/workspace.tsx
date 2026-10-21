'use client';

import type { ReactNode } from 'react';
import { Button } from '~/components/ui/button';
import { useAuth } from '~/lib/auth/provider';
import { useCopy } from '~/lib/copy';
import { usePreferences } from '~/components/providers/PreferencesProvider';
import { SignIn } from '~/components/auth/SignIn';
import { Items } from './items';

export function Workspace(): ReactNode {
  const { t } = useCopy();
  const { user, busy, signOut } = useAuth();
  const compact = usePreferences((state) => state.compact);
  const setCompact = usePreferences((state) => state.setCompact);
  return (
    <main id="main" className="page-container workspace-main" data-compact={compact}>
      <div className="workspace-heading">
        <h1>{t(user ? 'app' : 'signIn')}</h1>
        {user && (
          <div className="account">
            <span>{user.username}</span>
            <Button
              variant="ghost"
              disabled={busy}
              onClick={(): void => {
                void signOut();
              }}
            >
              {t('signOut')}
            </Button>
          </div>
        )}
      </div>
      {busy && !user ? (
        <p role="status">{t('loading')}</p>
      ) : user ? (
        <>
          <div className="density-option">
            <Button
              variant="ghost"
              size="sm"
              aria-pressed={compact}
              onClick={(): void => setCompact(!compact)}
            >
              {t('compact')}
            </Button>
          </div>
          <Items />
        </>
      ) : (
        <div className="max-w-sm">
          <SignIn />
        </div>
      )}
    </main>
  );
}
