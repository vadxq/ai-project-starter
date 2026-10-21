'use client';

import { useState, type FormEvent, type ReactNode } from 'react';
import { Button } from '~/components/ui/button';
import { Input } from '~/components/ui/input';
import { useCopy } from '~/lib/copy';
import type { Item, UpdateItem } from '~/lib/api/generated';

export function titleValid(value: string): boolean {
  const title: string = value.replace(/^[ \t\r\n]+|[ \t\r\n]+$/g, '');
  const bytes: number = new TextEncoder().encode(title).length;
  return bytes > 0 && bytes <= 240;
}

interface EditorProps {
  item: Item;
  busy: boolean;
  update: (item: Item, patch: UpdateItem) => Promise<void>;
  close: () => void;
}

export function ItemEditor({ item, busy, update, close }: EditorProps): ReactNode {
  const { t } = useCopy();
  const [title, setTitle] = useState<string>(item.title);
  const [invalid, setInvalid] = useState<boolean>(false);
  const submit = async (event: FormEvent<HTMLFormElement>): Promise<void> => {
    event.preventDefault();
    if (!titleValid(title)) {
      setInvalid(true);
      return;
    }
    try {
      await update(item, { version: item.version, title });
      close();
    } catch {
      // 列表呈现 mutation 错误，编辑器保留输入以便重试。
    }
  };
  return (
    <form
      className="edit-form"
      onSubmit={(event): void => {
        void submit(event);
      }}
    >
      <label className="sr-only" htmlFor={'title-' + item.id}>
        {t('title')}
      </label>
      <Input
        id={'title-' + item.id}
        value={title}
        autoFocus
        disabled={busy}
        aria-invalid={invalid}
        onChange={(event): void => {
          setTitle(event.target.value);
          setInvalid(false);
        }}
      />
      {invalid && (
        <p className="field-error" role="alert">
          {t('invalidTitle')}
        </p>
      )}
      <div className="row-actions">
        <Button type="submit" disabled={busy}>
          {t('save')}
        </Button>
        <Button variant="ghost" type="button" disabled={busy} onClick={close}>
          {t('cancel')}
        </Button>
      </div>
    </form>
  );
}
