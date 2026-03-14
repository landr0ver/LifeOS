import { writable } from 'svelte/store';
import { api } from '$lib/services/api';

function createAuthStore() {
  const token = typeof localStorage !== 'undefined' ? localStorage.getItem('lifeos_token') : null;
  const { subscribe, set } = writable<string | null>(token);

  return {
    subscribe,
    async login(password: string) {
      const res = await api.post<{ token: string }>('/auth/login', { password });
      localStorage.setItem('lifeos_token', res.token);
      set(res.token);
    },
    logout() {
      api.post('/auth/logout').catch(() => {});
      localStorage.removeItem('lifeos_token');
      set(null);
    },
    isLoggedIn() {
      return typeof localStorage !== 'undefined' && !!localStorage.getItem('lifeos_token');
    },
  };
}

export const auth = createAuthStore();
