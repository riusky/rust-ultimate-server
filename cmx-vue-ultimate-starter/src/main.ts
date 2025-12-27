import { createApp } from 'vue'

import App from './App.vue'
import { setupPlugins } from './plugins'
import router from './router'
import '@/assets/index.css'
import '@/assets/scrollbar.css'
import '@/assets/themes.css'
// import '@/assets/chart-theme.css'
import 'vue-sonner/style.css' // vue sonner style
import '@/assets/themes-combined.css'

import '@/utils/env'

import { useColorMode } from '@vueuse/core'

// 初始化主题（默认跟随系统偏好，可手动设置 'light' 或 'dark'）
useColorMode({
  initialValue: 'dark', // 默认主题（可选 'auto' | 'light' | 'dark'）
  storageKey: 'vueuse-color-scheme', // localStorage 的 key（可选）
})

function bootstrap() {
  const app = createApp(App)

  setupPlugins(app)

  app.use(router)
  app.mount('#app')
}

bootstrap()
