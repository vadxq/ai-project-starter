import type { ReactNode } from 'react';
import '~/styles/globals.css';
import Link from 'next/link';
import en from '~/locales/en.json';
import zh from '~/locales/zh-CN.json';

export default function GlobalNotFound(): ReactNode {
  return (
    <html lang="en">
      <body style={{ fontFamily: 'sans-serif', padding: '10vh 8vw' }}>
        <main>
          <h1>404</h1>
          <p>
            {en.notFound.title} <span lang="zh-CN">{zh.notFound.title}</span>
          </p>
          <Link href="/en">{en.notFound.home}</Link>
          {' · '}
          <Link href="/zh-CN" lang="zh-CN">
            {zh.notFound.home}
          </Link>
        </main>
      </body>
    </html>
  );
}
