import i18next, { type TFunction } from 'i18next';
import { initReactI18next, useTranslation } from 'react-i18next';
import en from './locales/en.json';
import zh from './locales/zh-CN.json';

export type Locale = 'en' | 'zh-CN';

export function initialLocale(): Locale {
  const stored: string | null = localStorage.getItem('locale');
  if (stored === 'en' || stored === 'zh-CN') return stored;
  return navigator.language.toLowerCase().startsWith('zh') ? 'zh-CN' : 'en';
}

export async function initializeLocale(): Promise<void> {
  await i18next.use(initReactI18next).init({
    resources: { en: { translation: en }, 'zh-CN': { translation: zh } },
    lng: initialLocale(),
    fallbackLng: 'en',
    interpolation: { escapeValue: false },
  });
  document.documentElement.lang = i18next.language;
}

export function useCopy(): { t: TFunction; locale: Locale; setLocale: (locale: Locale) => void } {
  const { t, i18n } = useTranslation();
  const setLocale = (locale: Locale): void => {
    localStorage.setItem('locale', locale);
    document.documentElement.lang = locale;
    void i18n.changeLanguage(locale);
  };
  return { t, locale: i18n.language === 'zh-CN' ? 'zh-CN' : 'en', setLocale };
}
