import tailwindcss from '@tailwindcss/vite'
import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import VueRouter from 'unplugin-vue-router/vite'
import Components from 'unplugin-vue-components/vite'
import { VantResolver } from 'unplugin-vue-components/resolvers'
import IconsResolver from 'unplugin-icons/resolver'
import { VueRouterAutoImports } from 'unplugin-vue-router'
import AutoImport from 'unplugin-auto-import/vite'
import Layouts from 'vite-plugin-vue-layouts'
import Markdown from 'unplugin-vue-markdown/vite'
import { visualizer } from 'rollup-plugin-visualizer'
import prism from 'markdown-it-prism'
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite'
const RouteGenerateExclude = [
  '**/ignored/**',
  '**/__*',
  '**/__**/*',
  '**/*.component.vue',
  '**/components/**',
  '**/layouts/**',
  '**/data/**',
  '**/types/**',
]
import { pathResolve } from './build/utils'
import yaml from '@rollup/plugin-yaml'

const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    yaml(),
    VueRouter({
      extensions: ['.page.vue', '.vue'],
      importMode: 'async',
      dts: 'src/types/typed-router.d.ts', // 类型提示文件
      logs: false,
      // getRouteName: getPascalCaseRouteName,
      experimental: {
        autoExportsDataLoaders: ['src/loaders/**/*', '@/loaders/**/*'],
      },
      // 对每个生成的路由进行修改
      extendRoute(route) {
        route.params.forEach((param) => {
          // transform kebab-case to camelCase
          param.paramName = param.paramName.replace(/-([a-z])/g, (g) => g[1].toUpperCase())
        })
      },
      beforeWriteFiles(root) {
        // root.insert('/from-root', join(__dirname, './src/pages/index.vue'))
      },
      exclude: RouteGenerateExclude,
    }),
    vue(),
    VueI18nPlugin({
      include: [pathResolve('./src/plugins/i18n/locales/**')],
    }),
    vueJsx(),
    vueDevTools(),
    tailwindcss(),
    visualizer({ gzipSize: true, brotliSize: true }),
    Markdown({
      markdownItOptions: {},
      headEnabled: true,
      markdownItUses: [prism],
    }),
    Layouts({
      layoutsDirs: 'src/layouts',
      pagesDirs: 'src/pages',
      defaultLayout: 'default',
    }),
    AutoImport({
      include: [
        /\.[tj]sx?$/, // .ts, .tsx, .js, .jsx
        /\.vue$/,
        /\.vue\?vue/, // .vue
        /\.vue\.[tj]sx?\?vue/, // .vue (vue-loader with experimentalInlineMatchResource enabled)
        /\.md$/, // .md
      ],
      imports: ['vue', 'vue-router', VueRouterAutoImports], // 自动加载 vue,vue-router api
      dirs: ['src/composables/**/*.ts', 'src/enum/**/*.ts', 'src/store/**/*.ts'], // 自动加载配置里的文件
      // Include auto-imported packages in Vite's `optimizeDeps` options
      // Recommend to enable
      viteOptimizeDeps: true,
      injectAtEnd: true,
      // Options for scanning directories for auto import
      defaultExportByFilename: true, // 包含文件夹名称，避免命名冲突
      dts: 'src/types/auto-import.d.ts', // 类型提示文件
    }),
    Components({
      dirs: ['src/components', 'src/components/global-layout'],
      extensions: ['vue', 'md', 'svg'],
      collapseSamePrefixes: true,
      // 启用目录名作为组件的命名空间。避免不同目录下同名组件的冲突。
      directoryAsNamespace: true,
      // 自动生成 TypeScript 类型声明文件。默认会在项目根目录生成 components.d.ts。提供组件导入的类型提示，避免 TS 报错。
      // dts: true,
      dts: 'src/types/auto-import-components.d.ts',
      // 指定全局组件的命名空间。例如 global::MyComponent 会从全局命名空间解析。适用于无需手动导入的全局组件。
      // globalNamespaces: ['global'],
      include: [/\.vue($|\?)/, /\.md($|\?)/],
      exclude: [/[\\/]node_modules[\\/]/, /[\\/]\.git[\\/]/, /[\\/]\.nuxt[\\/]/],
      // 自定义组件解析器，用于指定如何解析组件名到实际路径。
      resolvers: [
        // (name) => {
        //   if (name === 'MyCustom')
        //     return path.resolve(__dirname, 'src/CustomResolved.vue').replaceAll('\\', '/')
        // },
        VantResolver(),
        // 配合 unplugin-icons，将图标名解析为组件，并添加前缀 i（例如 <i-mdi-home />）。
        IconsResolver({
          componentPrefix: 'i',
        }),
      ],
      directives: true,
      // 调试模式下打印解析到的组件信息。开发时会在控制台输出组件名和路径的映射关系。帮助确认组件是否被正确识别。
      dumpComponentsInfo: true,
    }),
  ],
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
})
