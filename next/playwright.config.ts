import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  timeout: 45_000,
  retries: 0,
  workers: 1,
  outputDir: '.ailog/test-results',
  reporter: [['list'], ['html', { open: 'never', outputFolder: '.ailog/playwright-report' }]],
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    command: 'npm run start',
    url: 'http://localhost:3000',
    reuseExistingServer: false,
    timeout: 30_000,
  },
});
