import { expect, test } from '@playwright/test';

test('invalid credentials stay in the app and a revoked session clears on reload', async ({
  page,
  request,
}, testInfo) => {
  await page.goto('/');
  await page.getByRole('combobox', { name: 'Language' }).selectOption('en');
  await page.getByLabel('Username', { exact: true }).fill('alice');
  await page.getByLabel('Password', { exact: true }).fill('wrong-password');
  await page.getByRole('button', { name: 'Sign in', exact: true }).click();
  await expect(page.getByRole('main').getByRole('alert')).toContainText(
    'Incorrect username or password',
    { timeout: 15_000 },
  );
  expect(new URL(page.url()).port).toBe('3000');
  for (const width of [360, 768, 1440]) {
    await page.setViewportSize({ width, height: 900 });
    await expect(page.getByLabel('Username', { exact: true })).toBeVisible();
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth),
    ).toBe(true);
    await page.screenshot({ path: testInfo.outputPath(`login-${width}.png`), fullPage: true });
  }
  await page.getByLabel('Password', { exact: true }).fill('starter-password');
  await page.getByRole('button', { name: 'Sign in', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Sign out' })).toBeVisible();
  const refresh: string = await page.evaluate(
    () => sessionStorage.getItem('starter.refresh') ?? '',
  );
  expect(refresh).toMatch(/^[a-f0-9]{64}$/);
  const logout = await request.post('http://localhost:8080/api/v1/auth/logout', {
    data: { refreshToken: refresh },
  });
  expect(logout.status()).toBe(204);
  await page.reload();
  await expect(page.getByRole('button', { name: 'Sign in', exact: true })).toBeVisible();
  await expect(page.getByRole('textbox', { name: 'Item title', exact: true })).toHaveCount(0);
  expect(await page.evaluate(() => sessionStorage.getItem('starter.refresh'))).toBeNull();
  expect(await page.evaluate(() => JSON.stringify(localStorage))).not.toContain('accessToken');
});
