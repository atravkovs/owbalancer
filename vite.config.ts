import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { ViteRsw } from 'vite-plugin-rsw';
import { URL, fileURLToPath } from 'url';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    ViteRsw(),
  ],
  resolve: {
    alias: [
      {
        find: '@',
        replacement: fileURLToPath(new URL('./src', import.meta.url)),
      },
      {
        find: '~bootstrap',
        replacement: fileURLToPath(new URL('./node_modules/bootstrap', import.meta.url)),
      },
    ],
  },
});
