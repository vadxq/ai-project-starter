import { expect, test } from '@playwright/test';

test('public locale content and login form are server-rendered', async ({ request }) => {
  const publicPage = await request.get('/zh-CN');
  expect(publicPage.status()).toBe(200);
  const html: string = await publicPage.text();
  expect(html).toContain('登录');
  expect(html).toContain('lang="zh-CN"');
  expect(html).toContain('<title>事项</title>');
  expect(html).toMatch(/autocomplete="username"/i);
  expect(html).toMatch(/autocomplete="current-password"/i);
});
