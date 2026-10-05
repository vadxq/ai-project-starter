'use client';

import { useState, type JSX, type FormEvent } from 'react';
import Link from 'next/link';
import { useAuth } from '../auth/provider';
import { useCopy } from '../i18n';

export function SignIn(): JSX.Element {
  const { user, busy, error, signIn } = useAuth();
  const { t, locale } = useCopy();
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  if (user) return <Link href={`/${locale}/items`}>{t('back')}</Link>;
  const submit = async (event: FormEvent<HTMLFormElement>): Promise<void> => {
    event.preventDefault();
    if (busy) return;
    await signIn({ username, password });
    setPassword('');
  };
  return (
    <form
      className="sign-in-form"
      onSubmit={(event: FormEvent<HTMLFormElement>): void => {
        void submit(event);
      }}
    >
      <label htmlFor="username">{t('username')}</label>
      <input
        id="username"
        name="username"
        autoComplete="username"
        maxLength={64}
        required
        value={username}
        onChange={(event): void => {
          setUsername(event.target.value);
        }}
        disabled={busy}
        autoCapitalize="none"
      />
      <label htmlFor="password">{t('password')}</label>
      <input
        id="password"
        name="password"
        type="password"
        autoComplete="current-password"
        maxLength={128}
        required
        value={password}
        onChange={(event): void => {
          setPassword(event.target.value);
        }}
        disabled={busy}
      />
      {error && (
        <p className="error-state" role="alert">
          {t(error)}
        </p>
      )}
      <button className="primary" type="submit" disabled={busy}>
        {t(busy ? 'loading' : 'signIn')}
      </button>
    </form>
  );
}
