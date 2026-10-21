'use client';

import { Button } from '~/components/ui/button';
import { ItemEditor } from './item-editor';

import { useState, type JSX } from 'react';
import type { Item, UpdateItem } from '~/lib/api/generated';
import { useCopy } from '~/lib/copy';

interface RowProps {
  item: Item;
  busy: boolean;
  update: (item: Item, patch: UpdateItem) => Promise<void>;
  remove: (item: Item) => Promise<void>;
}

export function ItemRow({ item, busy, update, remove }: RowProps): JSX.Element {
  const { t, locale } = useCopy();
  const [editing, setEditing] = useState<boolean>(false);
  const [confirming, setConfirming] = useState<boolean>(false);
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
          <ItemEditor
            item={item}
            busy={busy}
            update={update}
            close={(): void => setEditing(false)}
          />
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
              <Button
                variant="ghost"
                className="danger"
                disabled={busy}
                onClick={(): void => {
                  void remove(item).catch(() => undefined);
                }}
              >
                {t('delete')}
              </Button>
              <Button variant="ghost" disabled={busy} onClick={(): void => setConfirming(false)}>
                {t('cancel')}
              </Button>
            </>
          ) : (
            <>
              <Button
                variant="ghost"
                disabled={busy}
                onClick={(): void => {
                  setEditing(true);
                }}
              >
                {t('edit')}
              </Button>
              <Button variant="ghost" disabled={busy} onClick={(): void => setConfirming(true)}>
                {t('delete')}
              </Button>
            </>
          )}
        </div>
      )}
    </li>
  );
}
