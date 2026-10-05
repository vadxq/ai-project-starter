import { useState, type FormEvent, type JSX } from 'react';
import { useCopy } from '../i18n';
import { titleValid } from './item-row';

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
        <input
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
        <button className="primary" type="submit" disabled={saving}>
          {t('add')}
        </button>
      </form>
      {invalid && (
        <p id="new-error" className="field-error" role="alert">
          {t('invalidTitle')}
        </p>
      )}
    </>
  );
}
