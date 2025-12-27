<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="图标库" description="浏览和使用系统中的所有图标">
    <div class="max-w-7xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">图标库</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          浏览系统中可用的所有图标，支持搜索和分类查看
        </p>
      </div>

      <!-- 搜索和筛选 -->
      <div class="flex flex-col sm:flex-row gap-4 items-center justify-between">
        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3 flex-1 max-w-2xl">
          <div class="relative flex-1">
            <UiInput placeholder="搜索图标..." class="pl-9" v-model="searchQuery" />
            <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
              <UiSearch class="w-4 h-4 text-muted-foreground" />
            </div>
          </div>
          <UiSelect v-model="categoryFilter">
            <UiSelectTrigger class="w-32">
              <UiSelectValue placeholder="分类筛选" />
            </UiSelectTrigger>
            <UiSelectContent>
              <UiSelectItem value="all">全部</UiSelectItem>
              <UiSelectItem value="general">通用</UiSelectItem>
              <UiSelectItem value="navigation">导航</UiSelectItem>
              <UiSelectItem value="actions">操作</UiSelectItem>
              <UiSelectItem value="files">文件</UiSelectItem>
              <UiSelectItem value="communication">沟通</UiSelectItem>
              <UiSelectItem value="media">媒体</UiSelectItem>
            </UiSelectContent>
          </UiSelect>
        </div>
        <div class="flex items-center gap-2">
          <UiButton variant="outline" @click="toggleViewMode">
            <component :is="viewMode === 'grid' ? List : Grid" class="w-4 h-4 mr-2" />
            {{ viewMode === 'grid' ? '列表视图' : '网格视图' }}
          </UiButton>
        </div>
      </div>

      <!-- 图标展示区域 -->
      <div class="space-y-6">
        <!-- 网格视图 -->
        <div
          v-if="viewMode === 'grid'"
          class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4"
        >
          <div
            v-for="icon in filteredIcons"
            :key="icon.name"
            class="border rounded-lg p-4 text-center cursor-pointer hover:bg-muted/50 transition-colors group"
            @click="copyIconName(icon.name)"
          >
            <div class="flex flex-col items-center space-y-3">
              <div
                class="w-8 h-8 flex items-center justify-center text-muted-foreground group-hover:text-primary transition-colors"
              >
                <component :is="icon.component" class="w-6 h-6" />
              </div>
              <div class="text-xs text-muted-foreground font-mono truncate w-full">
                {{ icon.name }}
              </div>
            </div>
          </div>
        </div>

        <!-- 列表视图 -->
        <div v-else class="space-y-2">
          <UiCard>
            <UiCardContent class="p-0">
              <div class="divide-y">
                <div
                  v-for="icon in filteredIcons"
                  :key="icon.name"
                  class="p-4 hover:bg-muted/50 transition-colors cursor-pointer group"
                  @click="copyIconName(icon.name)"
                >
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                      <div
                        class="w-8 h-8 flex items-center justify-center text-muted-foreground group-hover:text-primary transition-colors"
                      >
                        <component :is="icon.component" class="w-5 h-5" />
                      </div>
                      <div>
                        <div class="font-medium text-sm">{{ icon.name }}</div>
                        <div class="text-xs text-muted-foreground">{{ icon.category }}</div>
                      </div>
                    </div>
                    <div class="text-xs text-muted-foreground font-mono">
                      {{ icon.name }}
                    </div>
                  </div>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>

        <!-- 空状态 -->
        <div
          v-if="filteredIcons.length === 0"
          class="flex flex-col items-center justify-center py-12 text-center border-2 border-dashed rounded-lg"
        >
          <div class="w-16 h-16 mb-4 rounded-full bg-muted flex items-center justify-center">
            <UiSearch class="w-8 h-8 text-muted-foreground" />
          </div>
          <h3 class="text-lg font-medium mb-2">没有找到图标</h3>
          <p class="text-muted-foreground">尝试调整搜索条件</p>
        </div>
      </div>

      <!-- 图标详情 -->
      <UiCard v-if="selectedIcon">
        <UiCardHeader>
          <UiCardTitle>图标详情</UiCardTitle>
          <UiCardDescription> 当前选中的图标信息和代码示例 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <div class="flex items-center gap-4">
                <div class="w-16 h-16 rounded-lg bg-primary/10 flex items-center justify-center">
                  <component :is="selectedIcon.component" class="w-8 h-8 text-primary" />
                </div>
                <div>
                  <h3 class="text-lg font-semibold">{{ selectedIcon.name }}</h3>
                  <p class="text-sm text-muted-foreground">{{ selectedIcon.category }}</p>
                </div>
              </div>
              <div class="space-y-2">
                <h4 class="font-medium">使用说明</h4>
                <p class="text-sm text-muted-foreground">
                  {{ selectedIcon.description }}
                </p>
              </div>
            </div>
            <div class="space-y-4">
              <h4 class="font-medium">代码示例</h4>
              <div class="bg-muted rounded-lg p-4">
                <pre
                  class="text-sm font-mono whitespace-pre-wrap"
                ><code>&lt;{{ selectedIcon.name }} class="w-4 h-4" /&gt;</code></pre>
              </div>
              <UiButton @click="copyCodeExample(selectedIcon.name)" class="w-full">
                <UiCopy class="w-4 h-4 mr-2" />
                复制代码
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 复制成功提示 -->
      <UiAlert
        v-if="copySuccess"
        variant="default"
        class="fixed bottom-4 right-4 w-80 animate-in slide-in-from-bottom"
      >
        <UiCheckCircle class="w-4 h-4" />
        <UiAlertTitle>复制成功</UiAlertTitle>
        <UiAlertDescription> 图标名称已复制到剪贴板 </UiAlertDescription>
      </UiAlert>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import {
  Grid,
  List,
  // 通用图标
  Home,
  Settings,
  User,
  Users,
  Mail,
  Phone,
  MessageCircle,
  Bell,
  Star,
  Heart,
  // 导航图标
  ChevronLeft,
  ChevronRight,
  ChevronUp,
  ChevronDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,
  Menu,
  X,
  // 操作图标
  Plus,
  Minus,
  Edit,
  Trash2,
  Download,
  Upload,
  Copy as CopyIcon,
  Share,
  // 文件图标
  File,
  Folder,
  Image,
  Video,
  Music,
  // 媒体图标
  Play,
  Pause,
  Volume2,
  VolumeX,
} from 'lucide-vue-next'

// 图标数据
const icons = [
  // 通用图标
  { name: 'Home', component: Home, category: 'general', description: '主页、首页相关功能' },
  { name: 'Settings', component: Settings, category: 'general', description: '设置、配置相关功能' },
  { name: 'User', component: User, category: 'general', description: '用户、个人资料相关' },
  { name: 'Users', component: Users, category: 'general', description: '用户组、团队相关' },
  { name: 'Mail', component: Mail, category: 'general', description: '邮件、消息相关' },
  { name: 'Phone', component: Phone, category: 'general', description: '电话、联系相关' },
  {
    name: 'MessageCircle',
    component: MessageCircle,
    category: 'general',
    description: '聊天、对话相关',
  },
  { name: 'Bell', component: Bell, category: 'general', description: '通知、提醒相关' },
  { name: 'Star', component: Star, category: 'general', description: '收藏、评分相关' },
  { name: 'Heart', component: Heart, category: 'general', description: '喜欢、收藏相关' },

  // 导航图标
  {
    name: 'ChevronLeft',
    component: ChevronLeft,
    category: 'navigation',
    description: '向左展开、返回',
  },
  {
    name: 'ChevronRight',
    component: ChevronRight,
    category: 'navigation',
    description: '向右展开、前进',
  },
  { name: 'ChevronUp', component: ChevronUp, category: 'navigation', description: '向上展开' },
  { name: 'ChevronDown', component: ChevronDown, category: 'navigation', description: '向下展开' },
  { name: 'ArrowLeft', component: ArrowLeft, category: 'navigation', description: '向左箭头' },
  { name: 'ArrowRight', component: ArrowRight, category: 'navigation', description: '向右箭头' },
  { name: 'ArrowUp', component: ArrowUp, category: 'navigation', description: '向上箭头' },
  { name: 'ArrowDown', component: ArrowDown, category: 'navigation', description: '向下箭头' },
  { name: 'Menu', component: Menu, category: 'navigation', description: '菜单、导航' },
  { name: 'X', component: X, category: 'navigation', description: '关闭、取消' },

  // 操作图标
  { name: 'Plus', component: Plus, category: 'actions', description: '添加、新建' },
  { name: 'Minus', component: Minus, category: 'actions', description: '删除、减少' },
  { name: 'Edit', component: Edit, category: 'actions', description: '编辑、修改' },
  { name: 'Trash2', component: Trash2, category: 'actions', description: '删除、移除' },
  { name: 'Download', component: Download, category: 'actions', description: '下载、导出' },
  { name: 'Upload', component: Upload, category: 'actions', description: '上传、导入' },
  { name: 'Copy', component: CopyIcon, category: 'actions', description: '复制、克隆' },
  { name: 'Share', component: Share, category: 'actions', description: '分享、发送' },

  // 文件图标
  { name: 'File', component: File, category: 'files', description: '文件、文档' },
  { name: 'Folder', component: Folder, category: 'files', description: '文件夹、目录' },
  { name: 'Image', component: Image, category: 'files', description: '图片、照片' },
  { name: 'Video', component: Video, category: 'files', description: '视频、影片' },
  { name: 'Music', component: Music, category: 'files', description: '音乐、音频' },

  // 媒体图标
  { name: 'Play', component: Play, category: 'media', description: '播放、开始' },
  { name: 'Pause', component: Pause, category: 'media', description: '暂停、等待' },

  { name: 'Volume2', component: Volume2, category: 'media', description: '音量、声音' },
  { name: 'VolumeX', component: VolumeX, category: 'media', description: '静音、无声' },
]

// 响应式数据
const searchQuery = ref('')
const categoryFilter = ref('all')
const viewMode = ref<'grid' | 'list'>('grid')
const selectedIcon = ref<(typeof icons)[0] | null>(null)
const copySuccess = ref(false)

// 计算属性
const filteredIcons = computed(() => {
  let filtered = icons

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (icon) =>
        icon.name.toLowerCase().includes(query) ||
        icon.category.toLowerCase().includes(query) ||
        icon.description.toLowerCase().includes(query),
    )
  }

  // 分类过滤
  if (categoryFilter.value !== 'all') {
    filtered = filtered.filter((icon) => icon.category === categoryFilter.value)
  }

  return filtered
})

// 方法
const toggleViewMode = () => {
  viewMode.value = viewMode.value === 'grid' ? 'list' : 'grid'
}

const copyIconName = async (iconName: string) => {
  try {
    await navigator.clipboard.writeText(iconName)
    copySuccess.value = true
    const foundIcon = icons.find((icon) => icon.name === iconName)
    if (foundIcon) {
      selectedIcon.value = foundIcon
    }

    // 3秒后隐藏成功提示
    setTimeout(() => {
      copySuccess.value = false
    }, 3000)
  } catch (err) {
    console.error('复制失败:', err)
  }
}

const copyCodeExample = async (iconName: string) => {
  const code = `<${iconName} class="w-4 h-4" />`
  try {
    await navigator.clipboard.writeText(code)
    copySuccess.value = true

    // 3秒后隐藏成功提示
    setTimeout(() => {
      copySuccess.value = false
    }, 3000)
  } catch (err) {
    console.error('复制失败:', err)
  }
}
</script>

<style scoped></style>
