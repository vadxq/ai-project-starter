import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  outputDir: './.ailog/test-results',
  fullyParallel: false,
  workers: 1,
  use: { baseURL: 'http://localhost:3000', trace: 'retain-on-failure' },
  webServer: {
    command: 'npm run start',
    url: 'http://localhost:3000/en',
    reuseExistingServer: false,
    timeout: 60_000,
  },
});
