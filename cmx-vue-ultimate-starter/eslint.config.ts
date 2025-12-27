import { globalIgnores } from 'eslint/config'
import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript'
import pluginVue from 'eslint-plugin-vue'
import pluginVitest from '@vitest/eslint-plugin'
import pluginPlaywright from 'eslint-plugin-playwright'
import pluginOxlint from 'eslint-plugin-oxlint'
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'

export default defineConfigWithVueTs(
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}'],
    // 👇 添加 rules 配置，关闭 vue/multi-word-component-names
    rules: {
      'vue/multi-word-component-names': [
        'error',
        {
          ignores: [
            'index',
            'default',
            'auth',
            'otp',
            'tables',
            'examples',
            'icons',
            '401',
            '403',
            '404',
            '500',
            '503',
            'advanced',
            'basic',
            'upload',
            'validation',
            'contact',
            'faq',
            'tutorials',
            'account',
            'appearance',
            'display',
            'notifications',
            'copy',
            'dashboard',
            'analytics',
            'apps',
            'ai-talk',
            'billing',
            'data',
            'errors',
            'forms',
            'help',
            'poetry',
            'settings',
            'sva-components',
            'tasks',
            'users',
            'dev',
          ], // 忽略所有单单词组件名
        },
      ],
    },
  },

  globalIgnores(['**/dist/**', '**/dist-ssr/**', '**/coverage/**']),

  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,

  {
    ...pluginVitest.configs.recommended,
    files: ['src/**/__tests__/*'],
  },

  {
    ...pluginPlaywright.configs['flat/recommended'],
    files: ['e2e/**/*.{test,spec}.{js,ts,jsx,tsx}'],
  },
  ...pluginOxlint.configs['flat/recommended'],
  skipFormatting,
)
