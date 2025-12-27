// src/plugins/tsparticles.ts
import type { App } from 'vue'
import Particles from '@tsparticles/vue3'
import { loadSlim } from '@tsparticles/slim' // 轻量版引擎
import type { Engine } from '@tsparticles/engine' // 或相应的类型导出
// 如需完整功能可改用：import { loadFull } from 'tsparticles'

const initParticlesEngine = async (engine: Engine) => {
  await loadSlim(engine)
  // 如需更多特效（如感染、箭头形状等）可改用：await loadFull(engine)
}

export function setupTSParticles(app: App) {
  app.use(Particles, {
    init: initParticlesEngine,
  })
}

// 默认导出配置函数
export default setupTSParticles
