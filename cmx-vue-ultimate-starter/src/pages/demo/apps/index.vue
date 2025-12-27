<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="应用中心" description="发现和管理您的应用集成" sticky>
    <div class="flex flex-col lg:flex-row gap-6">
      <!-- Sidebar Filters -->
      <div class="lg:w-80 flex-shrink-0">
        <UiCard class="sticky top-20">
          <UiCardHeader>
            <UiCardTitle class="text-lg">筛选应用</UiCardTitle>
          </UiCardHeader>
          <UiCardContent class="p-4">
            <AppFilters
              v-model:search-query="searchQuery"
              v-model:selected-category="selectedCategory"
              v-model:selected-pricing="selectedPricing"
              v-model:show-featured-only="showFeaturedOnly"
              v-model:show-connected-only="showConnectedOnly"
              v-model:min-rating="minRating"
              :categories="categories"
              :pricing-types="pricingTypes"
              :stats="stats"
              :filtered-apps="filteredApps"
              @clear-filters="clearFilters"
            />
          </UiCardContent>
        </UiCard>
      </div>

      <!-- Main Content -->
      <div class="flex-1">
        <!-- Header Stats -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <UiCard>
            <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
              <UiCardTitle class="text-sm font-medium">总应用数</UiCardTitle>
              <Boxes class="h-4 w-4 text-muted-foreground" />
            </UiCardHeader>
            <UiCardContent>
              <div class="text-2xl font-bold">{{ stats.total }}</div>
              <p class="text-xs text-muted-foreground">涵盖 {{ categories.length }} 个分类</p>
            </UiCardContent>
          </UiCard>

          <UiCard>
            <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
              <UiCardTitle class="text-sm font-medium">已连接</UiCardTitle>
              <Check class="h-4 w-4 text-muted-foreground" />
            </UiCardHeader>
            <UiCardContent>
              <div class="text-2xl font-bold">{{ stats.connected }}</div>
              <p class="text-xs text-muted-foreground">
                {{ Math.round((stats.connected / stats.total) * 100) }}% 的应用
              </p>
            </UiCardContent>
          </UiCard>

          <UiCard>
            <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
              <UiCardTitle class="text-sm font-medium">精选应用</UiCardTitle>
              <Star class="h-4 w-4 text-muted-foreground" />
            </UiCardHeader>
            <UiCardContent>
              <div class="text-2xl font-bold">{{ stats.featured }}</div>
              <p class="text-xs text-muted-foreground">推荐使用的应用</p>
            </UiCardContent>
          </UiCard>

          <UiCard>
            <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
              <UiCardTitle class="text-sm font-medium">平均评分</UiCardTitle>
              <Star class="h-4 w-4 text-muted-foreground" />
            </UiCardHeader>
            <UiCardContent>
              <div class="text-2xl font-bold">
                {{
                  (
                    filteredApps.reduce((acc, app) => acc + app.rating, 0) / filteredApps.length ||
                    0
                  ).toFixed(1)
                }}
              </div>
              <p class="text-xs text-muted-foreground">基于用户评价</p>
            </UiCardContent>
          </UiCard>
        </div>

        <!-- View Toggle -->
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center space-x-2">
            <UiButton
              :variant="viewMode === 'grid' ? 'default' : 'outline'"
              size="sm"
              @click="viewMode = 'grid'"
            >
              <LayoutGrid class="w-4 h-4 mr-2" />
              网格视图
            </UiButton>
            <UiButton
              :variant="viewMode === 'list' ? 'default' : 'outline'"
              size="sm"
              @click="viewMode = 'list'"
            >
              <List class="w-4 h-4 mr-2" />
              列表视图
            </UiButton>
          </div>

          <div class="flex items-center space-x-2">
            <UiSelect v-model:model-value="sortBy">
              <UiSelectTrigger class="w-32">
                <UiSelectValue placeholder="排序方式" />
              </UiSelectTrigger>
              <UiSelectContent>
                <UiSelectItem value="name">名称</UiSelectItem>
                <UiSelectItem value="rating">评分</UiSelectItem>
                <UiSelectItem value="reviews">评价数</UiSelectItem>
                <UiSelectItem value="lastUpdated">更新时间</UiSelectItem>
              </UiSelectContent>
            </UiSelect>

            <UiSelect v-model:model-value="sortOrder">
              <UiSelectTrigger class="w-20">
                <UiSelectValue placeholder="顺序" />
              </UiSelectTrigger>
              <UiSelectContent>
                <UiSelectItem value="asc">升序</UiSelectItem>
                <UiSelectItem value="desc">降序</UiSelectItem>
              </UiSelectContent>
            </UiSelect>
          </div>
        </div>

        <!-- Apps Grid/List -->
        <div v-if="sortedApps.length > 0">
          <!-- Grid View -->
          <div
            v-if="viewMode === 'grid'"
            class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6"
          >
            <AppCard
              v-for="app in sortedApps"
              :key="app.id"
              :app="app"
              @toggle-connection="toggleAppConnection"
              @view-details="openAppDetail"
            />
          </div>

          <!-- List View -->
          <div v-else class="space-y-4">
            <div
              v-for="app in sortedApps"
              :key="app.id"
              class="p-4 border rounded-lg hover:shadow-md transition-shadow"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4 flex-1">
                  <div class="flex items-center justify-center p-3 rounded-xl size-12 bg-muted">
                    <component :is="app.logo" class="w-6 h-6" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center space-x-2 mb-1">
                      <h3 class="font-semibold text-lg">{{ app.name }}</h3>
                      <UiBadge
                        v-if="app.featured"
                        variant="secondary"
                        class="bg-gradient-to-r from-primary to-purple-600 text-white border-0"
                      >
                        精选
                      </UiBadge>
                      <UiBadge :variant="app.connected ? 'default' : 'outline'" class="text-xs">
                        {{ app.connected ? '已连接' : '未连接' }}
                      </UiBadge>
                    </div>
                    <p class="text-muted-foreground text-sm line-clamp-2">
                      {{ app.desc }}
                    </p>
                    <div class="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                      <span class="flex items-center space-x-1">
                        <Star class="w-3 h-3 text-yellow-500 fill-current" />
                        <span>{{ app.rating }}</span>
                        <span>({{ formatNumber(app.reviews) }})</span>
                      </span>
                      <span>{{ app.developer }}</span>
                      <span>{{ getCategoryLabel(app.category) }}</span>
                    </div>
                  </div>
                </div>
                <div class="flex items-center space-x-2 ml-4">
                  <UiButton
                    :variant="app.connected ? 'secondary' : 'default'"
                    size="sm"
                    @click="toggleAppConnection(app.id)"
                  >
                    <component :is="app.connected ? Check : Plug" class="w-4 h-4 mr-1.5" />
                    {{ app.connected ? '已连接' : '连接' }}
                  </UiButton>
                  <UiButton variant="outline" size="sm" @click="openAppDetail(app.id)">
                    详情
                  </UiButton>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-else class="text-center py-12">
          <Boxes class="w-16 h-16 text-muted-foreground mx-auto mb-4" />
          <h3 class="text-lg font-semibold mb-2">未找到应用</h3>
          <p class="text-muted-foreground mb-4">尝试调整筛选条件或搜索关键词</p>
          <UiButton @click="clearFilters"> 清除筛选条件 </UiButton>
        </div>
      </div>
    </div>

    <!-- App Detail Modal -->
    <AppDetail
      v-if="selectedApp"
      :app="selectedApp"
      :open="detailOpen"
      @update:open="detailOpen = $event"
      @toggle-connection="toggleAppConnection"
    />
  </Page>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Boxes, Check, LayoutGrid, List, Plug, Star } from 'lucide-vue-next'
import type { IApp } from './type'

import Page from '@/components/global-layout/basic-page.vue'
import AppCard from './components/app-card.vue'
import AppDetail from './components/app-detail.vue'
import AppFilters from './components/app-filters.vue'
import { useApps } from './composables/useApps'

// Use apps composable
const {
  searchQuery,
  selectedCategory,
  selectedPricing,
  showFeaturedOnly,
  showConnectedOnly,
  minRating,
  filteredApps,
  categories,
  pricingTypes,
  stats,
  toggleAppConnection,
  getAppById,
  clearFilters,
} = useApps()

// Local state
const viewMode = ref<'grid' | 'list'>('grid')
const sortBy = ref<'name' | 'rating' | 'reviews' | 'lastUpdated'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')
const detailOpen = ref(false)
const selectedApp = ref<IApp | null>(null)

// Computed properties
const sortedApps = computed(() => {
  const apps = [...filteredApps.value]

  return apps.sort((a, b) => {
    let aValue: string | number | Date
    let bValue: string | number | Date

    switch (sortBy.value) {
      case 'name':
        aValue = a.name
        bValue = b.name
        break
      case 'rating':
        aValue = a.rating
        bValue = b.rating
        break
      case 'reviews':
        aValue = a.reviews
        bValue = b.reviews
        break
      case 'lastUpdated':
        aValue = new Date(a.lastUpdated)
        bValue = new Date(b.lastUpdated)
        break
      default:
        aValue = a.name
        bValue = b.name
    }

    if (sortOrder.value === 'asc') {
      return aValue < bValue ? -1 : aValue > bValue ? 1 : 0
    } else {
      return aValue > bValue ? -1 : aValue < bValue ? 1 : 0
    }
  })
})

// Methods
const openAppDetail = (appId: string) => {
  const app = getAppById(appId)
  if (app) {
    selectedApp.value = app
    detailOpen.value = true
  }
}

const formatNumber = (num: number): string => {
  if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'k'
  }
  return num.toString()
}

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

// Note: Connected only filter is handled in the composable
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 2;
}
</style>
