// src/plugins/i18n/index.ts
import { createI18n, type I18n } from 'vue-i18n'
import { ref, watch, type Ref } from 'vue'

// 1. 定义语言文件加载函数
const loadLocaleMessages = () => {
  const locales = import.meta.glob('./locales/*.y(a)?ml', { eager: true })
  const messages: Record<string, any> = {}

  Object.keys(locales).forEach(key => {
    const matched = key.match(/([A-Za-z0-9-_]+)\./i)
    if (matched && matched[1]) {
      const locale = matched[1]
      messages[locale] = (locales[key] as any).default
    }
  })

  return messages
}

// 监听 localStorage 变化以实现多标签页同步
window.addEventListener('storage', (event) => {
  if (event.key === 'locale' && event.newValue) {
    currentLocale.value = event.newValue
    if ('value' in i18n.global.locale) {
      i18n.global.locale.value = event.newValue
    }
  }
})

// 确保在页面加载时也监听 visibilitychange
document.addEventListener('visibilitychange', () => {
  if (!document.hidden) {
    const savedLocale = localStorage.getItem('locale')
    if (savedLocale && savedLocale !== currentLocale.value) {
      currentLocale.value = savedLocale
    }
  }
})

// 2. 语言状态管理
const getDefaultLocale = (): string => {
  return localStorage.getItem('locale') || 'en'
}

// const currentLocale = ref(getDefaultLocale())

export const setLocale = (newLocale: string) => {
  currentLocale.value = newLocale
  localStorage.setItem('locale', newLocale)

  // 手动触发 storage 事件以确保当前页也能响应
  window.dispatchEvent(
    new StorageEvent('storage', {
      key: 'locale',
      newValue: newLocale,
      oldValue: currentLocale.value,
      storageArea: localStorage
    })
  )
}
// 在导出部分添加
export const currentLocale = ref(getDefaultLocale())
export const getLocale = (): string => currentLocale.value

// 3. 创建 i18n 实例（带完整类型声明）
interface CustomI18nGlobal {
  t: (key: string, params?: Record<string, any>) => string
  locale: Ref<string>
  getLocaleMessage: (locale: string) => Record<string, any>
}

export const i18n: I18n & { global: CustomI18nGlobal } = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'en',
  messages: loadLocaleMessages()
}) as any

// 4. 扁平化键名工具函数
const flattenMessages = (messages: any, prefix = ''): Record<string, string> => {
  let result: Record<string, string> = {}

  for (const key in messages) {
    const value = messages[key]
    const fullKey = prefix ? `${prefix}.${key}` : key

    if (typeof value === 'object' && value !== null) {
      result = { ...result, ...flattenMessages(value, fullKey) }
    } else {
      result[fullKey] = value as string
    }
  }

  return result
}

// 5. 获取扁平化键名集合
const getFlatI18nKeys = (locale = 'zh-CN') => {
  const messages = i18n.global.getLocaleMessage(locale)
  return new Set(Object.keys(flattenMessages(messages)))
}

// 6. 核心转换函数
export const transformI18n = (message: any = '', params?: Record<string, any>): string => {
  if (!message) return ''

  // 处理对象格式的多语言内容 {zh: '', en: ''}
  if (typeof message === 'object' && message !== null) {
    return message[getLocale()] || ''
  }

  // 检查是否是国际化键名
  const flatKeys = getFlatI18nKeys()
  const isI18nKey = flatKeys.has(message.toString())

  if (isI18nKey) {
    return i18n.global.t(message.toString(), params)
  }

  // 处理带参数的纯文本
  if (params) {
    return Object.keys(params).reduce((result, key) => {
      return result.replace(new RegExp(`\\{${key}\\}`, 'g'), params[key])
    }, message.toString())
  }

  return message.toString()
}

// 7. 安装函数（最终修正版）
export const setupI18n = (app: any) => {
  app.use(i18n)

  // 响应式更新语言（完全类型安全）
  watch(currentLocale, (newLocale) => {
    if ('value' in i18n.global.locale) {
      // 处理 Ref 类型
      i18n.global.locale.value = newLocale
    } else {
      // 处理字符串类型（虽然legacy: false下不会出现）
      (i18n.global as any).locale = newLocale
    }
  })
}

// 8. 为 i18n Ally 插件提供支持
export const $t = (key: string) => key
