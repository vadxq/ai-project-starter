import { readFile } from 'node:fs/promises';
import { parse } from '@formatjs/icu-messageformat-parser';

const locales = ['en', 'zh-CN'];

function flattenMessages(messages, prefix) {
  return Object.fromEntries(
    Object.entries(messages).flatMap(([key, value]) => {
      const path = prefix ? `${prefix}.${key}` : key;
      if (typeof value === 'string') return [[path, value]];
      if (value === null || typeof value !== 'object' || Array.isArray(value)) {
        throw new TypeError(`Invalid translation value: ${path}`);
      }
      return Object.entries(flattenMessages(value, path));
    }),
  );
}

function getArguments(elements) {
  return elements.flatMap((element) => {
    const argument =
      'value' in element && element.type !== 0 && element.type !== 7 ? [element.value] : [];
    const children = 'children' in element ? getArguments(element.children) : [];
    const options =
      'options' in element
        ? Object.values(element.options).flatMap((option) => getArguments(option.value))
        : [];
    return [...argument, ...children, ...options];
  });
}

const translations = await Promise.all(
  locales.map(async (locale) => {
    const content = await readFile(new URL(`../locales/${locale}.json`, import.meta.url), 'utf8');
    return flattenMessages(JSON.parse(content), '');
  }),
);

const referenceKeys = Object.keys(translations[0]).sort();
translations.forEach((messages, index) => {
  if (JSON.stringify(Object.keys(messages).sort()) !== JSON.stringify(referenceKeys)) {
    throw new Error(`Translation keys differ: ${locales[index]}`);
  }

  referenceKeys.forEach((key) => {
    const expected = [...new Set(getArguments(parse(translations[0][key])))].sort();
    const actual = [...new Set(getArguments(parse(messages[key])))].sort();
    if (JSON.stringify(expected) !== JSON.stringify(actual)) {
      throw new Error(`Translation arguments differ: ${locales[index]}:${key}`);
    }
  });
});

console.info({ event: 'i18n_check_passed', locales, keys: referenceKeys.length });
