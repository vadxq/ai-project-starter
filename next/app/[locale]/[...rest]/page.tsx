import { notFound } from 'next/navigation';

// 匹配语言前缀下的未知路径，让当前语言的 not-found 页面处理。
export default function MissingPage(): never {
  notFound();
}
