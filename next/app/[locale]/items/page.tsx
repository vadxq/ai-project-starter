import type { ReactNode } from 'react';
import { Header } from '~/components/layout/Header';
import { Workspace } from '~/components/items/workspace';

export default function ItemsPage(): ReactNode {
  return (
    <>
      <Header />
      <Workspace />
    </>
  );
}
