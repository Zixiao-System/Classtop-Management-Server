import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue({
    template: {
      compilerOptions: {
        // 将所有 mdui- 开头的标签视为自定义元素
        isCustomElement: (tag) => tag.startsWith('mdui-')
      }
    }
  })],
  build: {
    outDir: '../static',
    emptyOutDir: true,
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8765',
        changeOrigin: true,
      }
    }
  }
})