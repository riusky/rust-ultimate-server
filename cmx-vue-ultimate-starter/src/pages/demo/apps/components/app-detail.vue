<template>
  <UiDialog v-model:open="open">
    <UiDialogContent class="max-w-4xl max-h-[90vh] overflow-hidden">
      <UiDialogHeader>
        <UiDialogTitle class="flex items-center space-x-3">
          <div
            class="flex items-center justify-center p-3 rounded-xl size-14 bg-gradient-to-br from-muted to-muted/80"
          >
            <component :is="app.logo" class="w-7 h-7 text-foreground" />
          </div>
          <div>
            <h2 class="text-2xl font-bold">{{ app.name }}</h2>
            <p class="text-muted-foreground">{{ app.developer }}</p>
          </div>
        </UiDialogTitle>
        <UiDialogDescription>
          {{ app.desc }}
        </UiDialogDescription>
      </UiDialogHeader>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 max-h-[60vh] overflow-y-auto">
        <!-- Main Content -->
        <div class="lg:col-span-2 space-y-6">
          <!-- Screenshots -->
          <div v-if="app.screenshots && app.screenshots.length > 0">
            <h3 class="text-lg font-semibold mb-3">应用截图</h3>
            <div class="grid grid-cols-2 gap-3">
              <div
                v-for="(screenshot, index) in app.screenshots"
                :key="index"
                class="aspect-video bg-muted rounded-lg overflow-hidden cursor-pointer"
                @click="selectedImage = screenshot"
              >
                <img
                  :src="screenshot"
                  :alt="`${app.name} screenshot ${index + 1}`"
                  class="w-full h-full object-cover hover:scale-105 transition-transform duration-300"
                />
              </div>
            </div>
          </div>

          <!-- Description -->
          <div>
            <h3 class="text-lg font-semibold mb-3">详细描述</h3>
            <p class="text-muted-foreground leading-relaxed">
              {{ app.desc }}
            </p>
          </div>

          <!-- Features -->
          <div>
            <h3 class="text-lg font-semibold mb-3">主要功能</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              <div
                v-for="(permission, index) in app.permissions"
                :key="index"
                class="flex items-center space-x-2 text-sm"
              >
                <Check class="w-4 h-4 text-green-500" />
                <span>{{ permission }}</span>
              </div>
            </div>
          </div>

          <!-- Requirements -->
          <div v-if="app.requirements">
            <h3 class="text-lg font-semibold mb-3">系统要求</h3>
            <div class="space-y-2 text-sm">
              <div v-if="app.requirements.minVersion" class="flex justify-between py-2 border-b">
                <span class="text-muted-foreground">最低版本</span>
                <span class="font-medium">{{ app.requirements.minVersion }}</span>
              </div>
              <div v-if="app.requirements.storage" class="flex justify-between py-2 border-b">
                <span class="text-muted-foreground">存储空间</span>
                <span class="font-medium">{{ app.requirements.storage }}</span>
              </div>
              <div v-if="app.requirements.memory" class="flex justify-between py-2 border-b">
                <span class="text-muted-foreground">内存要求</span>
                <span class="font-medium">{{ app.requirements.memory }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Sidebar -->
        <div class="space-y-6">
          <!-- Connection Status -->
          <UiCard>
            <UiCardHeader>
              <UiCardTitle class="text-lg">连接状态</UiCardTitle>
            </UiCardHeader>
            <UiCardContent class="space-y-4">
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">状态</span>
                <UiBadge :variant="app.connected ? 'default' : 'secondary'" class="capitalize">
                  {{ app.connected ? '已连接' : '未连接' }}
                </UiBadge>
              </div>
              <UiButton
                :variant="app.connected ? 'outline' : 'default'"
                class="w-full"
                @click="$emit('toggle-connection', app.id)"
              >
                <component :is="app.connected ? X : Plug" class="w-4 h-4 mr-2" />
                {{ app.connected ? '断开连接' : '立即连接' }}
              </UiButton>
            </UiCardContent>
          </UiCard>

          <!-- App Info -->
          <UiCard>
            <UiCardHeader>
              <UiCardTitle class="text-lg">应用信息</UiCardTitle>
            </UiCardHeader>
            <UiCardContent class="space-y-3">
              <div class="flex justify-between">
                <span class="text-sm text-muted-foreground">分类</span>
                <UiBadge variant="outline" class="capitalize">
                  {{ getCategoryLabel(app.category) }}
                </UiBadge>
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-muted-foreground">版本</span>
                <span class="text-sm font-medium">v{{ app.version }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-muted-foreground">更新日期</span>
                <span class="text-sm font-medium">{{ formatDate(app.lastUpdated) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-muted-foreground">状态</span>
                <UiBadge
                  :variant="
                    app.status === 'active'
                      ? 'default'
                      : app.status === 'maintenance'
                        ? 'secondary'
                        : 'destructive'
                  "
                  class="capitalize"
                >
                  {{ getStatusLabel(app.status) }}
                </UiBadge>
              </div>
            </UiCardContent>
          </UiCard>

          <!-- Pricing -->
          <UiCard v-if="app.pricing">
            <UiCardHeader>
              <UiCardTitle class="text-lg">价格方案</UiCardTitle>
            </UiCardHeader>
            <UiCardContent>
              <div class="text-center space-y-2">
                <div class="text-2xl font-bold text-primary">
                  {{ getPricingDisplay(app.pricing) }}
                </div>
                <p class="text-sm text-muted-foreground">
                  {{ getPricingDescription(app.pricing) }}
                </p>
                <UiButton
                  v-if="app.website"
                  variant="outline"
                  size="sm"
                  class="w-full"
                  @click="openWebsite"
                >
                  <ExternalLink class="w-4 h-4 mr-2" />
                  访问官网
                </UiButton>
              </div>
            </UiCardContent>
          </UiCard>

          <!-- Rating -->
          <UiCard>
            <UiCardHeader>
              <UiCardTitle class="text-lg">用户评价</UiCardTitle>
            </UiCardHeader>
            <UiCardContent>
              <div class="text-center space-y-2">
                <div class="flex items-center justify-center space-x-1">
                  <Star class="w-5 h-5 text-yellow-500 fill-current" />
                  <span class="text-2xl font-bold">{{ app.rating }}</span>
                  <span class="text-muted-foreground">/ 5.0</span>
                </div>
                <p class="text-sm text-muted-foreground">
                  基于 {{ formatNumber(app.reviews) }} 条评价
                </p>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <UiDialogFooter>
        <UiButton variant="outline" @click="open = false">关闭</UiButton>
        <UiButton @click="$emit('toggle-connection', app.id)">
          {{ app.connected ? '断开连接' : '立即连接' }}
        </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>

  <!-- Image Viewer -->
  <UiDialog v-model:open="imageViewerOpen">
    <UiDialogContent class="max-w-4xl p-0">
      <img :src="selectedImage" alt="应用截图" class="w-full h-auto max-h-[80vh] object-contain" />
    </UiDialogContent>
  </UiDialog>
</template>

<script setup lang="ts">
import { Check, ExternalLink, Plug, Star, X } from 'lucide-vue-next'
import type { IApp } from '../type'

interface Props {
  app: IApp
  open: boolean
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'toggle-connection', appId: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const open = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value),
})

const imageViewerOpen = ref(false)
const selectedImage = ref('')

// Helper functions
const formatNumber = (num: number): string => {
  if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'k'
  }
  return num.toString()
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
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

const getStatusLabel = (status: string): string => {
  const labels: Record<string, string> = {
    active: '活跃',
    inactive: '未激活',
    maintenance: '维护中',
    beta: '测试版',
  }
  return labels[status] || status
}

const getPricingDisplay = (
  pricing: { type?: string; price?: number; currency?: string; period?: string } | undefined,
): string => {
  if (!pricing) return '免费'

  switch (pricing.type) {
    case 'free':
      return '免费'
    case 'freemium':
      return pricing.price ? `$${pricing.price}/月` : '免费增值'
    case 'paid':
      return pricing.price ? `$${pricing.price}` : '付费'
    case 'subscription':
      return pricing.price ? `$${pricing.price}/月` : '订阅'
    default:
      return '免费'
  }
}

const getPricingDescription = (
  pricing: { type?: string; price?: number; currency?: string; period?: string } | undefined,
): string => {
  if (!pricing) return '完全免费使用'

  switch (pricing.type) {
    case 'free':
      return '完全免费使用'
    case 'freemium':
      return '基础功能免费，高级功能付费'
    case 'paid':
      return '一次性购买'
    case 'subscription':
      return '按月订阅'
    default:
      return '完全免费使用'
  }
}

const openWebsite = () => {
  if (props.app.website) {
    window.open(props.app.website, '_blank')
  }
}

// Watch for image selection
watch(selectedImage, (newImage) => {
  imageViewerOpen.value = !!newImage
})
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 3;
}
</style>
