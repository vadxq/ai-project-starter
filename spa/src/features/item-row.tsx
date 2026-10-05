import { useState, type FormEvent, type JSX } from 'react';
import type { Item, UpdateItem } from '../api/generated';
import { useCopy } from '../i18n';

export function titleValid(value: string): boolean {
  const title: string = value.replace(/^[ \t\r\n]+|[ \t\r\n]+$/g, '');
  const bytes: number = new TextEncoder().encode(title).length;
  return bytes > 0 && bytes <= 240;
}

interface RowProps {
  item: Item;
  busy: boolean;
  update: (item: Item, patch: UpdateItem) => Promise<void>;
  remove: (item: Item) => Promise<void>;
}

export function ItemRow({ item, busy, update, remove }: RowProps): JSX.Element {
  const { t, locale } = useCopy();
  const [editing, setEditing] = useState<boolean>(false);
  const [title, setTitle] = useState<string>(item.title);
  const [invalid, setInvalid] = useState<boolean>(false);
  const [confirming, setConfirming] = useState<boolean>(false);
  const submit = async (event: FormEvent<HTMLFormElement>): Promise<void> => {
    event.preventDefault();
    if (!titleValid(title)) {
      setInvalid(true);
      return;
    }
    try {
      await update(item, { version: item.version, title });
      setEditing(false);
    } catch {
      /* 错误由列表统一呈现，保留编辑输入。 */
    }
  };
  const date: string = new Intl.DateTimeFormat(locale, { dateStyle: 'medium' }).format(
    new Date(item.updatedAt),
  );
  return (
    <li className={`item-row ${item.completed ? 'is-complete' : ''}`}>
      <input
        type="checkbox"
        checked={item.completed}
        disabled={busy}
        aria-label={`${t(item.completed ? 'reopen' : 'complete')}: ${item.title}`}
        onChange={(): void => {
          void update(item, { version: item.version, completed: !item.completed }).catch(
            () => undefined,
          );
        }}
      />
      <div className="item-content">
        {editing ? (
          <form
            className="edit-form"
            onSubmit={(event): void => {
              void submit(event);
            }}
          >
            <label className="sr-only" htmlFor={`title-${item.id}`}>
              {t('title')}
            </label>
            <input
              id={`title-${item.id}`}
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
              <button type="submit" disabled={busy}>
                {t('save')}
              </button>
              <button type="button" disabled={busy} onClick={(): void => setEditing(false)}>
                {t('cancel')}
              </button>
            </div>
          </form>
        ) : (
          <>
            <p className="item-title">{item.title}</p>
            <span className="item-date">{t('updated', { date })}</span>
          </>
        )}
      </div>
      {!editing && (
        <div className="row-actions">
          {confirming ? (
            <>
              <span>{t('confirmDelete')}</span>
              <button
                className="danger"
                disabled={busy}
                onClick={(): void => {
                  void remove(item).catch(() => undefined);
                }}
              >
                {t('delete')}
              </button>
              <button disabled={busy} onClick={(): void => setConfirming(false)}>
                {t('cancel')}
              </button>
            </>
          ) : (
            <>
              <button
                disabled={busy}
                onClick={(): void => {
                  setTitle(item.title);
                  setEditing(true);
                }}
              >
                {t('edit')}
              </button>
              <button disabled={busy} onClick={(): void => setConfirming(true)}>
                {t('delete')}
              </button>
            </>
          )}
        </div>
      )}
    </li>
  );
}
