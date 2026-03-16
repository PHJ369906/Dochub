import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { copyFileSync, mkdirSync } from 'fs'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// 构建时自动拷贝 pdf.worker.min.mjs 到 public 目录
const copyPdfWorker = () => ({
  name: 'copy-pdf-worker',
  buildStart() {
    try {
      mkdirSync(resolve(__dirname, 'public'), { recursive: true })
      copyFileSync(
        resolve(__dirname, 'node_modules/pdfjs-dist/build/pdf.worker.min.mjs'),
        resolve(__dirname, 'public/pdf.worker.min.mjs')
      )
    } catch (e) {
      console.warn('copy pdf.worker failed:', e)
    }
  },
  configureServer() {
    // dev 模式同样拷贝
    try {
      mkdirSync(resolve(__dirname, 'public'), { recursive: true })
      copyFileSync(
        resolve(__dirname, 'node_modules/pdfjs-dist/build/pdf.worker.min.mjs'),
        resolve(__dirname, 'public/pdf.worker.min.mjs')
      )
    } catch (e) {
      console.warn('copy pdf.worker failed:', e)
    }
  }
})

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    copyPdfWorker(),
    vue(),
    AutoImport({
      resolvers: [ElementPlusResolver()],
      imports: ['vue', 'vue-router', 'pinia'],
      dts: true,
    }),
    Components({
      resolvers: [ElementPlusResolver()],
      dts: true,
    }),
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
  // Tauri 使用固定端口以便开发时连接
  server: {
    port: 3000,
    strictPort: true,
  },
  // 清除 env 前缀以便 Tauri 环境变量生效
  envPrefix: ['VITE_', 'TAURI_'],
})
