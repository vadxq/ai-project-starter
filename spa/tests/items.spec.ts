import { randomUUID } from 'node:crypto';
import { expect, test, type Page } from '@playwright/test';
test('pagination, request failure and retry preserve the draft', async ({ page }) => {
  test.setTimeout(120_000);
  await signIn(page);
  const prefix: string = `Paging ${randomUUID()}`;
  for (let index: number = 0; index < 21; index += 1) {
    await page.getByRole('textbox', { name: 'Item title', exact: true }).fill(`${prefix} ${index}`);
    await page.getByRole('button', { name: 'Add item' }).click();
    await expect(page.getByRole('textbox', { name: 'Item title', exact: true })).toHaveValue('');
  }
  await page.getByRole('button', { name: 'Next', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Previous', exact: true })).toBeEnabled();
  await page.getByRole('button', { name: 'Previous', exact: true }).click();
  const draft: string = `${prefix} draft`;
  await page.getByRole('textbox', { name: 'Item title', exact: true }).fill(draft);
  // 只中断传输，不伪造 API 响应；恢复后仍使用真实 Rust / PostgreSQL。
  await page.route('**/api/v1/items', (route) => route.abort('connectionfailed'));
  await page.getByRole('button', { name: 'Add item' }).click();
  await expect(page.getByRole('main').getByRole('alert')).toContainText('Could not connect');
  await expect(page.getByRole('textbox', { name: 'Item title', exact: true })).toHaveValue(draft);
  await page.unroute('**/api/v1/items');
  await page.getByRole('button', { name: 'Try again', exact: true }).click();
  await expect(page.getByRole('main').getByRole('alert')).toHaveCount(0);
  await page.getByRole('textbox', { name: 'Item title', exact: true }).fill('');
  for (let index: number = 0; index < 21; index += 1) {
    const row = page.getByRole('listitem').filter({ hasText: prefix }).first();
    await expect(row).toBeVisible();
    const title: string = (await row.getByRole('checkbox').getAttribute('aria-label')) ?? '';
    await row.getByRole('button', { name: 'Delete', exact: true }).click();
    await row.getByRole('button', { name: 'Delete', exact: true }).click();
    await expect(page.getByRole('checkbox', { name: title, exact: true })).toHaveCount(0);
  }
});
async function signIn(page: Page): Promise<void> {
  await page.goto('/');
  await page.getByRole('combobox', { name: 'Language' }).selectOption('en');
  await page.locator('#username').fill('alice');
  await page.locator('#password').fill('starter-password');
  await page.getByRole('button', { name: 'Sign in', exact: true }).click();
  await expect(page).toHaveURL(/\/items$/, { timeout: 15_000 });
  await expect(page.getByRole('button', { name: 'Sign out' })).toBeVisible();
}

test('real password login, CRUD, locales, themes, responsive layout, reload and logout', async ({
  page,
}, testInfo) => {
  const title: string = `Browser ${randomUUID()}`;
  await signIn(page);
  await page.getByRole('textbox', { name: 'Item title', exact: true }).fill(title);
  await page.getByRole('button', { name: 'Add item' }).click();
  const row = page
    .getByRole('listitem')
    .filter({ has: page.getByRole('checkbox', { name: new RegExp(title) }) });
  await expect(row).toBeVisible();
  await row.getByRole('button', { name: 'Edit', exact: true }).click();
  await row.getByRole('textbox', { name: 'Item title' }).fill(`${title} updated`);
  await row.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(row).toContainText(`${title} updated`);
  await row.getByRole('checkbox').click();
  await expect(row.getByRole('checkbox')).toBeChecked();
  await page.getByRole('combobox', { name: 'Appearance' }).selectOption('dark');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await page.getByRole('combobox', { name: 'Language' }).selectOption('zh-CN');
  await expect(page.getByRole('button', { name: '添加事项' })).toBeVisible();
  for (const width of [360, 768, 1440]) {
    await page.setViewportSize({ width, height: 900 });
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth),
    ).toBe(true);
    await page.screenshot({ path: testInfo.outputPath(`items-${width}.png`), fullPage: true });
  }
  await page.reload();
  await expect(page.getByRole('button', { name: '退出登录' })).toBeVisible();
  await expect(page.getByRole('listitem').filter({ hasText: title })).toBeVisible();
  const saved = page.getByRole('listitem').filter({ hasText: title });
  await saved.getByRole('button', { name: '删除', exact: true }).click();
  await saved.getByRole('button', { name: '删除', exact: true }).click();
  await expect(saved).toHaveCount(0);
  const stored: string = await page.evaluate(() => JSON.stringify(localStorage));
  expect(stored).not.toContain('access_token');
  await page.getByRole('button', { name: '退出登录' }).click();
  await expect(page.getByRole('button', { name: '登录', exact: true })).toBeVisible();
});

test('empty title is rejected before mutation and login UI is keyboard reachable', async ({
  page,
}) => {
  await signIn(page);
  await page.getByRole('button', { name: 'Add item' }).click();
  await expect(page.getByRole('alert').filter({ hasText: 'title' })).toContainText('title');
  await page.getByRole('textbox', { name: 'Item title', exact: true }).focus();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('button', { name: 'Add item' })).toBeFocused();
});
