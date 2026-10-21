'use client';

import { Button } from '~/components/ui/button';
import { Input } from '~/components/ui/input';

import { useState, type FormEvent, type JSX } from 'react';
import { useCopy } from '~/lib/copy';
import { titleValid } from './item-editor';

export function ItemComposer({
  saving,
  save,
}: {
  saving: boolean;
  save: (title: string) => Promise<void>;
}): JSX.Element {
  const { t } = useCopy();
  const [title, setTitle] = useState<string>('');
  const [invalid, setInvalid] = useState<boolean>(false);
  const submit = async (event: FormEvent<HTMLFormElement>): Promise<void> => {
    event.preventDefault();
    if (!titleValid(title)) {
      setInvalid(true);
      return;
    }
    try {
      await save(title);
      setTitle('');
    } catch {
      /* 由 useItems 展示 mutation 错误，保留本地输入。 */
    }
  };
  return (
    <>
      <form
        className="create-form"
        onSubmit={(event): void => {
          void submit(event);
        }}
      >
        <label className="sr-only" htmlFor="new-title">
          {t('title')}
        </label>
        <Input
          id="new-title"
          value={title}
          placeholder={t('placeholder')}
          disabled={saving}
          aria-invalid={invalid}
          aria-describedby={invalid ? 'new-error' : undefined}
          onChange={(event): void => {
            setTitle(event.target.value);
            setInvalid(false);
          }}
        />
        <Button className="primary" type="submit" disabled={saving}>
          {t('add')}
        </Button>
      </form>
      {invalid && (
        <p id="new-error" className="field-error" role="alert">
          {t('invalidTitle')}
        </p>
      )}
    </>
  );
}
