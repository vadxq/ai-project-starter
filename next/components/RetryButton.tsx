'use client';

import type { ReactNode } from 'react';
import { Button } from '~/components/ui/button';

export function RetryButton({ label }: { label: string }): ReactNode {
  return <Button onClick={() => window.location.reload()}>{label}</Button>;
}
