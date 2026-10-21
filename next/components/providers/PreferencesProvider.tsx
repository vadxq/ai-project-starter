'use client';

import { createContext, useContext, useEffect, useState, type ReactNode } from 'react';
import { useStore } from 'zustand';
import { createPreferencesStore, type PreferencesStore } from '~/stores/preferences';

type PreferencesStoreApi = ReturnType<typeof createPreferencesStore>;
const PreferencesContext = createContext<PreferencesStoreApi | null>(null);

export function PreferencesProvider({ children }: { children: ReactNode }): ReactNode {
  // 实例属于当前组件树，服务端请求之间不共享状态。
  const [store] = useState<PreferencesStoreApi>(() => createPreferencesStore({ compact: false }));

  useEffect(() => {
    void store.persist.rehydrate();
  }, [store]);

  return <PreferencesContext.Provider value={store}>{children}</PreferencesContext.Provider>;
}

export function usePreferences<T>(selector: (state: PreferencesStore) => T): T {
  const store = useContext(PreferencesContext);
  if (store === null) throw new Error('usePreferences requires PreferencesProvider');
  return useStore(store, selector);
}
