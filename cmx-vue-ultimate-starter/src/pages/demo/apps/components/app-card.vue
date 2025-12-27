<template>
  <UiCard
    class="group relative overflow-hidden transition-all duration-300 hover:shadow-lg hover:scale-[1.02]"
  >
    <!-- Featured Badge -->
    <div v-if="app.featured" class="absolute top-3 right-3 z-10">
      <UiBadge
        variant="secondary"
        class="bg-gradient-to-r from-primary to-purple-600 text-white border-0"
      >
        <Star class="w-3 h-3 mr-1" />
        精选
      </UiBadge>
    </div>

    <!-- Status Indicator -->
    <div class="absolute top-3 left-3 z-10">
      <div
        class="w-2 h-2 rounded-full"
        :class="{
          'bg-green-500': app.status === 'active',
          'bg-yellow-500': app.status === 'maintenance',
          'bg-red-500': app.status === 'inactive',
          'bg-blue-500': app.status === 'beta',
        }"
      />
    </div>

    <UiCardHeader class="pb-3">
      <div class="flex items-start justify-between">
        <!-- App Logo and Basic Info -->
        <div class="flex items-center space-x-3">
          <div class="relative">
            <div
              class="flex items-center justify-center p-2 rounded-xl size-12 bg-gradient-to-br from-muted to-muted/80 shadow-sm"
            >
              <component :is="app.logo" class="w-6 h-6 text-foreground" />
            </div>
            <!-- Connection Status -->
            <div
              class="absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-background"
              :class="app.connected ? 'bg-green-500' : 'bg-muted-foreground/30'"
            />
          </div>

          <div class="flex-1 min-w-0">
            <UiCardTitle class="text-base font-semibold truncate">
              {{ app.name }}
            </UiCardTitle>
            <UiCardDescription class="text-xs mt-1">
              {{ app.developer }}
            </UiCardDescription>
          </div>
        </div>

        <!-- Rating -->
        <div class="flex items-center space-x-1 text-xs text-muted-foreground">
          <Star class="w-3 h-3 text-yellow-500 fill-current" />
          <span class="font-medium">{{ app.rating }}</span>
          <span>({{ formatNumber(app.reviews) }})</span>
        </div>
      </div>
    </UiCardHeader>

    <UiCardContent class="pt-0">
      <!-- Description -->
      <p class="text-sm text-muted-foreground line-clamp-2 mb-3">
        {{ app.desc }}
      </p>

      <!-- Tags -->
      <div class="flex flex-wrap gap-1 mb-3">
        <UiBadge
          v-for="tag in app.tags.slice(0, 2)"
          :key="tag"
          variant="outline"
          class="text-xs px-2 py-0 h-5"
        >
          {{ tag }}
        </UiBadge>
        <UiBadge v-if="app.tags.length > 2" variant="outline" class="text-xs px-2 py-0 h-5">
          +{{ app.tags.length - 2 }}
        </UiBadge>
      </div>

      <!-- Pricing & Version -->
      <div class="flex items-center justify-between text-xs text-muted-foreground mb-3">
        <div class="flex items-center space-x-2">
          <UiBadge
            variant="secondary"
            class="capitalize"
            :class="{
              'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300':
                app.pricing?.type === 'free',
              'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300':
                app.pricing?.type === 'freemium',
              'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300':
                app.pricing?.type === 'paid',
              'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300':
                app.pricing?.type === 'subscription',
            }"
          >
            {{ getPricingLabel(app.pricing?.type) }}
          </UiBadge>
          <span>v{{ app.version }}</span>
        </div>
        <span>{{ formatDate(app.lastUpdated) }}</span>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center justify-between">
        <UiButton
          :variant="app.connected ? 'secondary' : 'default'"
          size="sm"
          class="flex-1 mr-2"
          @click="$emit('toggle-connection', app.id)"
        >
          <component :is="app.connected ? Check : Plug" class="w-4 h-4 mr-1.5" />
          {{ app.connected ? '已连接' : '连接' }}
        </UiButton>

        <UiButton variant="ghost" size="sm" class="px-2" @click="$emit('view-details', app.id)">
          <MoreHorizontal class="w-4 h-4" />
        </UiButton>
      </div>
    </UiCardContent>
  </UiCard>
</template>

<script setup lang="ts">
import { Check, MoreHorizontal, Plug, Star } from 'lucide-vue-next'
import type { IApp } from '../type'

defineProps<{
  app: IApp
}>()

defineEmits<{
  'toggle-connection': [appId: string]
  'view-details': [appId: string]
}>()

// Helper functions
const formatNumber = (num: number): string => {
  if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'k'
  }
  return num.toString()
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  const now = new Date()
  const diffTime = Math.abs(now.getTime() - date.getTime())
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))

  if (diffDays === 1) return '今天'
  if (diffDays <= 7) return `${diffDays}天前`
  if (diffDays <= 30) return `${Math.ceil(diffDays / 7)}周前`
  return `${Math.ceil(diffDays / 30)}月前`
}

const getPricingLabel = (type?: string): string => {
  switch (type) {
    case 'free':
      return '免费'
    case 'freemium':
      return '免费增值'
    case 'paid':
      return '付费'
    case 'subscription':
      return '订阅'
    default:
      return '免费'
  }
}
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
