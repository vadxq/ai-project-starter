import createMiddleware from 'next-intl/middleware';
import { routing } from '~/lib/i18n/routing';

export default createMiddleware(routing);

export const config = {
  // 框架和静态资源绕过语言重定向。
  matcher: ['/((?!api|_next|_vercel|.*\\..*).*)'],
};
