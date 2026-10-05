import type { NextConfig } from 'next';
import createNextIntlPlugin from 'next-intl/plugin';

const config: NextConfig = { output: 'standalone', poweredByHeader: false };
const withNextIntl = createNextIntlPlugin('./src/intl/request.ts');
export default withNextIntl(config);
