import { cp, mkdir } from 'node:fs/promises';

// Next standalone 不自动复制浏览器静态资源；产物在本工程内准备，可直接复制运行。
await mkdir('.next/standalone/.next', { recursive: true });
await cp('.next/static', '.next/standalone/.next/static', { recursive: true });
