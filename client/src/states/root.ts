import {defineStore} from 'pinia';

export const rootStore = defineStore({
  id: 'root',
  state: () => ({
    baseUrl: import.meta.env.VITE_API_BASE_URL,
  })
});