<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="古诗词欣赏" description="探索中华古典诗词之美" sticky>
    <div class="max-w-4xl mx-auto space-y-8">
      <!-- 搜索和筛选区域 -->
      <div class="flex flex-col gap-4 sm:flex-row justify-center">
        <div class="flex-1 max-w-md">
          <UiInput v-model="searchQuery" placeholder="搜索诗词、作者或诗句..." class="w-full">
            <template #prefix>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="size-4 text-muted-foreground"
              >
                <circle cx="11" cy="11" r="8" />
                <path d="m21 21-4.3-4.3" />
              </svg>
            </template>
          </UiInput>
        </div>
        <UiSelect v-model="selectedDynasty">
          <UiSelectTrigger class="w-[180px]">
            <UiSelectValue placeholder="选择朝代" />
          </UiSelectTrigger>
          <UiSelectContent>
            <UiSelectItem v-for="dynasty in dynasties" :key="dynasty.value" :value="dynasty.value">
              {{ dynasty.name }}
            </UiSelectItem>
          </UiSelectContent>
        </UiSelect>
      </div>

      <!-- 诗词列表 -->
      <PoetryList :search-query="searchQuery" :selected-dynasty="selectedDynasty" />
      <PoetryList2 />
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import PoetryList from './components/poetry-list.vue'

const searchQuery = ref('')
const selectedDynasty = ref('all')

const dynasties = [
  { name: '全部', value: 'all' },
  { name: '唐代', value: 'tang' },
  { name: '宋代', value: 'song' },
  { name: '元代', value: 'yuan' },
  { name: '明代', value: 'ming' },
  { name: '清代', value: 'qing' },
]
</script>

<style scoped></style>
