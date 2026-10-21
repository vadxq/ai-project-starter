import { createStore, type Mutate, type StoreApi } from 'zustand/vanilla';
import { createJSONStorage, persist } from 'zustand/middleware';

export type PreferencesState = { compact: boolean };
export type PreferencesStore = PreferencesState & {
  setCompact: (compact: boolean) => void;
  reset: () => void;
};

export type PreferencesStoreApi = Mutate<
  StoreApi<PreferencesStore>,
  [['zustand/persist', PreferencesState]]
>;

export function createPreferencesStore(initialState: PreferencesState): PreferencesStoreApi {
  // SSR 不读取 localStorage；客户端挂载后恢复用户偏好。
  return createStore<PreferencesStore>()(
    persist(
      (set) => ({
        ...initialState,
        setCompact: (compact: boolean): void => {
          set({ compact });
        },
        reset: (): void => {
          set(initialState);
        },
      }),
      {
        name: 'starter-preferences',
        version: 1,
        storage: createJSONStorage<PreferencesState>(() => localStorage),
        partialize: (state) => ({ compact: state.compact }),
        // 只恢复经过验证的偏好，避免旧数据覆盖 store 的 action。
        merge: (persisted: unknown, current: PreferencesStore): PreferencesStore => ({
          ...current,
          compact:
            typeof persisted === 'object' &&
            persisted !== null &&
            'compact' in persisted &&
            typeof persisted.compact === 'boolean'
              ? persisted.compact
              : current.compact,
        }),
        skipHydration: true,
      },
    ),
  );
}
