import type { App } from 'vue'

import { setupAutoAnimate } from './auto-animate'
import { setupDayjs } from './dayjs'
import { setupI18n } from './i18n'
import { setupNProgress } from './nprogress'
import { setupPinia } from './pinia'
import { setupTanstackVueQuery } from './tanstack-vue-query'
import { setupTSParticles } from './tsparticles'

export function setupPlugins(app: App) {
  setupDayjs()
  setupNProgress()
  setupAutoAnimate(app)
  setupTanstackVueQuery(app)
  setupPinia(app)
  setupI18n(app)
  setupTSParticles(app)
}
