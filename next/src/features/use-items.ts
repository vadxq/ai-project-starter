'use client';

import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  createItem,
  deleteItem,
  getMe,
  listItems,
  updateItem,
  type Identity,
  type Item,
  type ItemPage,
  type UpdateItem,
} from '../api/generated';
import { apiOptions, errorCode } from '../api/client';
import { useAuth } from '../auth/provider';

export const PAGE_SIZE = 20;
type Write =
  | { kind: 'create'; title: string }
  | { kind: 'update'; item: Item; patch: UpdateItem }
  | { kind: 'delete'; item: Item };
interface ItemsState {
  page: ItemPage | undefined;
  loading: boolean;
  fetching: boolean;
  saving: boolean;
  code: string | null;
  offset: number;
  setOffset: (offset: number) => void;
  write: (command: Write) => Promise<void>;
  retry: () => void;
}

async function writeItem(command: Write): Promise<void> {
  if (command.kind === 'create') {
    await createItem({ ...apiOptions(), body: { title: command.title, completed: false } });
  } else if (command.kind === 'update') {
    await updateItem({ ...apiOptions(), path: { id: command.item.id }, body: command.patch });
  } else {
    await deleteItem({
      ...apiOptions(),
      path: { id: command.item.id },
      query: { version: command.item.version },
    });
  }
}

export function useItems(): ItemsState {
  const { user } = useAuth();
  const cache = useQueryClient();
  const [offset, setOffset] = useState<number>(0);
  // 后端 Identity UUID 是数据缓存的用户边界。
  const identity = useQuery({
    queryKey: ['me', user?.id],
    enabled: Boolean(user),
    retry: 1,
    queryFn: async ({ signal }): Promise<Identity> =>
      (await getMe({ ...apiOptions(), signal })).data,
  });
  const query = useQuery({
    queryKey: ['items', identity.data?.id, offset],
    enabled: Boolean(identity.data),
    retry: 1,
    queryFn: async ({ signal }): Promise<ItemPage> =>
      (await listItems({ ...apiOptions(), query: { limit: PAGE_SIZE, offset }, signal })).data,
  });
  const mutation = useMutation({
    mutationFn: writeItem,
    onSuccess: async (): Promise<void> => {
      await cache.invalidateQueries({ queryKey: ['items'] });
    },
  });
  const error: unknown = mutation.error ?? identity.error ?? query.error;
  return {
    page: query.data,
    loading: !identity.isError && query.isPending,
    fetching: identity.isFetching || query.isFetching,
    saving: mutation.isPending,
    code: error ? errorCode(error) : null,
    offset,
    setOffset,
    write: async (command: Write): Promise<void> => {
      await mutation.mutateAsync(command);
    },
    retry: (): void => {
      mutation.reset();
      if (identity.isError) void identity.refetch();
      else void query.refetch();
    },
  };
}
