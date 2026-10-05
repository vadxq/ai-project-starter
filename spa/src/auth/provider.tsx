import { createContext, useContext, useEffect, useState, type ReactNode, type JSX } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import type { Identity, LoginRequest } from '../api/generated/types.gen';
import { authManager, authErrorCode } from './manager';

interface AuthState {
  user: Identity | null;
  busy: boolean;
  error: string | null;
  signIn: (input: LoginRequest) => Promise<void>;
  signOut: () => Promise<void>;
}

const AuthContext = createContext<AuthState | null>(null);

export function AuthProvider({ children }: { children: ReactNode }): JSX.Element {
  const [user, setUser] = useState<Identity | null>(null);
  const [busy, setBusy] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  useEffect(() => {
    const manager = authManager();
    let active: boolean = true;
    const unsubscribe = manager.subscribe((next: Identity | null): void => {
      if (active) {
        setUser(next);
        if (!next) queryClient.clear();
      }
    });
    void manager
      .restore()
      .then((next: Identity | null): void => {
        if (active) setUser(next);
      })
      .catch((cause: unknown): void => {
        if (active) setError(authErrorCode(cause));
      })
      .finally((): void => {
        if (active) setBusy(false);
      });
    return (): void => {
      active = false;
      unsubscribe();
    };
  }, [queryClient]);

  const signIn = async (input: LoginRequest): Promise<void> => {
    setBusy(true);
    setError(null);
    queryClient.clear();
    try {
      await authManager().signIn(input);
      navigate('/items', { replace: true });
    } catch (cause: unknown) {
      setError(authErrorCode(cause));
    } finally {
      setBusy(false);
    }
  };
  const signOut = async (): Promise<void> => {
    setBusy(true);
    setError(null);
    queryClient.clear();
    try {
      await authManager().signOut();
    } catch (cause: unknown) {
      setError(authErrorCode(cause));
    } finally {
      setBusy(false);
    }
  };
  return (
    <AuthContext.Provider value={{ user, busy, error, signIn, signOut }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthState {
  const context = useContext(AuthContext);
  if (!context) throw new Error('AuthProvider is required');
  return context;
}
