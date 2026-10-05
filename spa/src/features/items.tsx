import type { JSX } from 'react';
import { authManager } from '../auth/manager';
import { useCopy } from '../i18n';
import { ItemRow } from './item-row';
import { ItemComposer } from './item-composer';
import { PAGE_SIZE, useItems } from './use-items';

export function Items(): JSX.Element {
  const { t, locale } = useCopy();
  const { page, loading, fetching, saving, code, offset, setOffset, write, retry } = useItems();
  const total: number = page?.total ?? 0;
  return (
    <section className="workspace" aria-label={t('app')}>
      <ItemComposer
        saving={saving}
        save={async (title: string): Promise<void> => {
          await write({ kind: 'create', title });
          setOffset(0);
        }}
      />
      {code && (
        <div className="error-state" role="alert">
          <span>{t(code, { defaultValue: t('unknown_error') })}</span>
          <button
            onClick={
              code === 'unauthorized'
                ? (): void => {
                    authManager().clear();
                  }
                : retry
            }
          >
            {t(code === 'unauthorized' ? 'signIn' : 'retry')}
          </button>
        </div>
      )}
      <div className="list-heading">
        <span>{t('count', { count: total })}</span>
        {fetching && <span role="status">{t('loading')}</span>}
      </div>
      {loading ? (
        <p role="status" className="empty-state">
          {t('loading')}
        </p>
      ) : page?.items.length === 0 ? (
        <div className="empty-state">
          <h2>{t('empty')}</h2>
          <p>{t('emptyBody')}</p>
        </div>
      ) : (
        <ul className="item-list">
          {page?.items.map((item) => (
            <ItemRow
              key={item.id}
              item={item}
              busy={saving}
              update={async (item, patch): Promise<void> => {
                await write({ kind: 'update', item, patch });
              }}
              remove={async (item): Promise<void> => {
                await write({ kind: 'delete', item });
                if (page.items.length === 1 && offset > 0) setOffset(offset - PAGE_SIZE);
              }}
            />
          ))}
        </ul>
      )}
      {(total > PAGE_SIZE || offset > 0) && (
        <nav className="pagination" aria-label={t('app')}>
          <button
            disabled={offset === 0 || fetching}
            onClick={(): void => setOffset(Math.max(0, offset - PAGE_SIZE))}
          >
            {t('previous')}
          </button>
          <span>{new Intl.NumberFormat(locale).format(Math.floor(offset / PAGE_SIZE) + 1)}</span>
          <button
            disabled={offset + PAGE_SIZE >= total || fetching}
            onClick={(): void => setOffset(offset + PAGE_SIZE)}
          >
            {t('next')}
          </button>
        </nav>
      )}
    </section>
  );
}
