<script setup lang="ts">
import { setLocale, currentLocale } from '@/plugins/i18n'
import { Languages, Check } from 'lucide-vue-next'
import { Icon } from '@iconify/vue'

const availableLocales = [
  { code: 'zh-CN', name: '中文', flag: 'flag:cn-4x3' },
  { code: 'en', name: 'English', flag: 'flag:us-4x3' },
  { code: 'de', name: 'Deutsch', flag: 'flag:de-4x3' },
  { code: 'es', name: 'Español', flag: 'flag:es-4x3' },
  { code: 'fr', name: 'Français', flag: 'flag:fr-4x3' },
  { code: 'ja', name: '日本語', flag: 'flag:jp-4x3' },
  { code: 'ko', name: '한국어', flag: 'flag:kr-4x3' },
  { code: 'ru', name: 'Русский', flag: 'flag:ru-4x3' },
]

const changeLanguage = (lang: string) => {
  setLocale(lang)
}
</script>

<template>
  <UiDropdownMenu>
    <UiDropdownMenuTrigger as-child>
      <UiButton variant="outline" size="icon" class="hover:!bg-accent hover:text-accent-foreground">
        <Languages class="h-[1.2rem] w-[1.2rem]" />
        <span class="sr-only">Toggle language</span>
      </UiButton>
    </UiDropdownMenuTrigger>
    <UiDropdownMenuContent align="end" class="w-[180px]">
      <UiDropdownMenuItem
        v-for="locale in availableLocales"
        :key="locale.code"
        @click="changeLanguage(locale.code)"
      >
        <span class="flex items-center justify-between w-full">
          <span class="flex items-center gap-2">
            <Icon :icon="locale.flag" class="h-4 w-4" />
            {{ locale.name }}
          </span>
          <Check v-if="currentLocale === locale.code" class="h-4 w-4 text-primary" />
        </span>
      </UiDropdownMenuItem>
    </UiDropdownMenuContent>
  </UiDropdownMenu>
</template>
