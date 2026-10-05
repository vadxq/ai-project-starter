import { useEffect, useState, type JSX } from 'react';
import { useCopy, type Locale } from '../i18n';

type Theme = 'system' | 'light' | 'dark';

function savedTheme(): Theme {
  if (typeof window === 'undefined') return 'system';
  const value: string | null = localStorage.getItem('theme');
  return value === 'light' || value === 'dark' ? value : 'system';
}

export function Preferences(): JSX.Element {
  const { t, locale, setLocale } = useCopy();
  const [theme, setTheme] = useState<Theme>(savedTheme);
  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    localStorage.setItem('theme', theme);
  }, [theme]);
  return (
    <div className="preferences">
      <label>
        <span className="sr-only">{t('language')}</span>
        <select
          aria-label={t('language')}
          value={locale}
          onChange={(event): void => setLocale(event.target.value as Locale)}
        >
          <option value="en">English</option>
          <option value="zh-CN">简体中文</option>
        </select>
      </label>
      <label>
        <span className="sr-only">{t('theme')}</span>
        <select
          aria-label={t('theme')}
          value={theme}
          onChange={(event): void => setTheme(event.target.value as Theme)}
        >
          <option value="system">{t('system')}</option>
          <option value="light">{t('light')}</option>
          <option value="dark">{t('dark')}</option>
        </select>
      </label>
    </div>
  );
}
