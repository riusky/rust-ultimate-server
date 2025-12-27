<template>
  <div class="space-y-6">
    <!-- Search -->
    <div>
      <label class="text-sm font-medium mb-2 block">搜索应用</label>
      <UiInput
        v-model:model-value="searchQuery"
        placeholder="搜索应用名称、描述、开发者..."
        class="w-full"
      >
        <template #prefix>
          <Search class="w-4 h-4 text-muted-foreground" />
        </template>
      </UiInput>
    </div>

    <!-- Categories -->
    <div>
      <label class="text-sm font-medium mb-3 block">应用分类</label>
      <div class="space-y-2">
        <UiButton
          v-for="category in categories"
          :key="category"
          :variant="selectedCategory === category ? 'default' : 'outline'"
          size="sm"
          class="w-full justify-start"
          @click="selectedCategory = category"
        >
          <div class="flex items-center justify-between w-full">
            <span>{{ getCategoryLabel(category) }}</span>
            <UiBadge variant="secondary" class="text-xs">
              {{ stats.byCategory[category] || 0 }}
            </UiBadge>
          </div>
        </UiButton>

        <UiButton
          :variant="selectedCategory === 'all' ? 'default' : 'outline'"
          size="sm"
          class="w-full justify-start"
          @click="selectedCategory = 'all'"
        >
          <div class="flex items-center justify-between w-full">
            <span>全部应用</span>
            <UiBadge variant="secondary" class="text-xs">
              {{ stats.total }}
            </UiBadge>
          </div>
        </UiButton>
      </div>
    </div>

    <!-- Pricing -->
    <div>
      <label class="text-sm font-medium mb-3 block">价格类型</label>
      <div class="space-y-2">
        <UiButton
          v-for="type in pricingTypes"
          :key="type"
          :variant="selectedPricing === type ? 'default' : 'outline'"
          size="sm"
          class="w-full justify-start capitalize"
          @click="selectedPricing = type"
        >
          {{ getPricingLabel(type) }}
        </UiButton>

        <UiButton
          :variant="selectedPricing === 'all' ? 'default' : 'outline'"
          size="sm"
          class="w-full justify-start"
          @click="selectedPricing = 'all'"
        >
          全部类型
        </UiButton>
      </div>
    </div>

    <!-- Rating -->
    <div>
      <label class="text-sm font-medium mb-3 block">最低评分</label>
      <div class="space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">{{ minRating }}+ 星</span>
          <UiBadge variant="secondary" class="text-xs">
            {{ getAppsCountByRating(minRating) }}
          </UiBadge>
        </div>
        <UiSlider v-model:model-value="minRating" :max="5" :step="0.5" class="w-full" />
        <div class="flex justify-between text-xs text-muted-foreground">
          <span>0</span>
          <span>5</span>
        </div>
      </div>
    </div>

    <!-- Additional Filters -->
    <div class="space-y-4">
      <div class="flex items-center space-x-2">
        <UiCheckbox v-model:checked="showFeaturedOnly" id="featured" />
        <label for="featured" class="text-sm font-medium leading-none"> 仅显示精选应用 </label>
        <UiBadge v-if="showFeaturedOnly" variant="secondary" class="text-xs">
          {{ stats.featured }}
        </UiBadge>
      </div>

      <div class="flex items-center space-x-2">
        <UiCheckbox v-model:checked="showConnectedOnly" id="connected" />
        <label for="connected" class="text-sm font-medium leading-none"> 仅显示已连接 </label>
        <UiBadge v-if="showConnectedOnly" variant="secondary" class="text-xs">
          {{ stats.connected }}
        </UiBadge>
      </div>
    </div>

    <!-- Active Filters -->
    <div v-if="hasActiveFilters" class="pt-4 border-t">
      <div class="flex items-center justify-between mb-2">
        <span class="text-sm font-medium">当前筛选</span>
        <UiButton
          variant="ghost"
          size="sm"
          @click="clearFilters"
          class="h-auto p-0 text-xs text-muted-foreground hover:text-foreground"
        >
          清除全部
        </UiButton>
      </div>
      <div class="flex flex-wrap gap-1">
        <UiBadge v-if="selectedCategory !== 'all'" variant="secondary" class="text-xs">
          {{ getCategoryLabel(selectedCategory) }}
          <button @click="selectedCategory = 'all'" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>

        <UiBadge v-if="selectedPricing !== 'all'" variant="secondary" class="text-xs capitalize">
          {{ getPricingLabel(selectedPricing) }}
          <button @click="selectedPricing = 'all'" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>

        <UiBadge v-if="minRating > 0" variant="secondary" class="text-xs">
          {{ minRating }}+ 星
          <button @click="minRating = 0" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>

        <UiBadge v-if="showFeaturedOnly" variant="secondary" class="text-xs">
          精选应用
          <button @click="showFeaturedOnly = false" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>

        <UiBadge v-if="showConnectedOnly" variant="secondary" class="text-xs">
          已连接
          <button @click="showConnectedOnly = false" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>

        <UiBadge v-if="searchQuery" variant="secondary" class="text-xs">
          搜索: {{ searchQuery }}
          <button @click="searchQuery = ''" class="ml-1 hover:text-destructive">
            <X class="w-3 h-3" />
          </button>
        </UiBadge>
      </div>
    </div>

    <!-- Results Count -->
    <div class="pt-4 border-t">
      <div class="text-center">
        <p class="text-sm text-muted-foreground">
          找到 <span class="font-semibold text-foreground">{{ filteredApps.length }}</span> 个应用
        </p>
        <p class="text-xs text-muted-foreground mt-1">
          共 {{ stats.total }} 个应用，{{ stats.connected }} 个已连接
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Search, X } from 'lucide-vue-next'
import type { AppCategory } from '../type'

interface Props {
  searchQuery: string
  selectedCategory: AppCategory | 'all'
  selectedPricing: string
  showFeaturedOnly: boolean
  showConnectedOnly: boolean
  minRating: number
  categories: AppCategory[]
  pricingTypes: string[]
  stats: {
    total: number
    connected: number
    byCategory: Record<string, number>
    featured: number
  }
  filteredApps: {
    id: string
    name: string
    rating: number
    connected: boolean
  }[]
}

interface Emits {
  (e: 'update:searchQuery', value: string): void
  (e: 'update:selectedCategory', value: AppCategory | 'all'): void
  (e: 'update:selectedPricing', value: string): void
  (e: 'update:showFeaturedOnly', value: boolean): void
  (e: 'update:showConnectedOnly', value: boolean): void
  (e: 'update:minRating', value: number): void
  (e: 'clear-filters'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Computed properties
const searchQuery = computed({
  get: () => props.searchQuery,
  set: (value) => emit('update:searchQuery', value),
})

const selectedCategory = computed({
  get: () => props.selectedCategory,
  set: (value) => emit('update:selectedCategory', value),
})

const selectedPricing = computed({
  get: () => props.selectedPricing,
  set: (value) => emit('update:selectedPricing', value),
})

const showFeaturedOnly = computed({
  get: () => props.showFeaturedOnly,
  set: (value) => emit('update:showFeaturedOnly', value),
})

const showConnectedOnly = computed({
  get: () => props.showConnectedOnly,
  set: (value) => emit('update:showConnectedOnly', value),
})

const minRating = computed({
  get: () => props.minRating,
  set: (value) => emit('update:minRating', value),
})

const hasActiveFilters = computed(() => {
  return (
    props.selectedCategory !== 'all' ||
    props.selectedPricing !== 'all' ||
    props.minRating > 0 ||
    props.showFeaturedOnly ||
    props.showConnectedOnly ||
    !!props.searchQuery
  )
})

// Helper functions
const getCategoryLabel = (category: string): string => {
  const labels: Record<string, string> = {
    communication: '通讯',
    productivity: '生产力',
    development: '开发',
    design: '设计',
    finance: '金融',
    entertainment: '娱乐',
    utilities: '工具',
    security: '安全',
  }
  return labels[category] || category
}

const getPricingLabel = (type: string): string => {
  const labels: Record<string, string> = {
    free: '免费',
    freemium: '免费增值',
    paid: '付费',
    subscription: '订阅',
  }
  return labels[type] || type
}

const getAppsCountByRating = (rating: number): number => {
  return props.filteredApps.filter((app) => app.rating >= rating).length
}

const clearFilters = () => {
  emit('clear-filters')
}
</script>

<style scoped></style>
