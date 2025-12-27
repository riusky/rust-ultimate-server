<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="数据列表" description="这是一个数据列表示例，展示卡片式数据展示和列表操作">
    <div class="space-y-6">
      <!-- 筛选和搜索 -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3">
          <UiInput placeholder="搜索项目..." class="w-full sm:w-64" v-model="searchQuery" />
          <UiSelect v-model="categoryFilter">
            <UiSelectTrigger class="w-32">
              <UiSelectValue placeholder="分类筛选" />
            </UiSelectTrigger>
            <UiSelectContent>
              <UiSelectItem value="all">全部</UiSelectItem>
              <UiSelectItem value="design">设计</UiSelectItem>
              <UiSelectItem value="development">开发</UiSelectItem>
              <UiSelectItem value="marketing">营销</UiSelectItem>
              <UiSelectItem value="research">研究</UiSelectItem>
            </UiSelectContent>
          </UiSelect>
          <UiSelect v-model="statusFilter">
            <UiSelectTrigger class="w-32">
              <UiSelectValue placeholder="状态筛选" />
            </UiSelectTrigger>
            <UiSelectContent>
              <UiSelectItem value="all">全部</UiSelectItem>
              <UiSelectItem value="active">进行中</UiSelectItem>
              <UiSelectItem value="completed">已完成</UiSelectItem>
              <UiSelectItem value="pending">待处理</UiSelectItem>
            </UiSelectContent>
          </UiSelect>
        </div>
        <div class="flex gap-2">
          <UiButton variant="outline" @click="toggleViewMode">
            <component :is="viewMode === 'grid' ? List : Grid" class="w-4 h-4 mr-2" />
            {{ viewMode === 'grid' ? '列表视图' : '网格视图' }}
          </UiButton>
          <UiButton @click="showCreateDialog = true">
            <UiPlus class="w-4 h-4 mr-2" />
            新建项目
          </UiButton>
        </div>
      </div>

      <!-- 网格视图 -->
      <div v-if="viewMode === 'grid'" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <UiCard
          v-for="item in filteredItems"
          :key="item.id"
          class="hover:shadow-md transition-shadow cursor-pointer"
          @click="viewItem(item)"
        >
          <UiCardHeader class="pb-3">
            <div class="flex items-start justify-between">
              <div>
                <UiCardTitle class="text-base">{{ item.title }}</UiCardTitle>
                <UiCardDescription class="mt-1">{{ item.description }}</UiCardDescription>
              </div>
              <UiBadge :variant="getStatusVariant(item.status)" class="shrink-0">
                {{ getStatusText(item.status) }}
              </UiBadge>
            </div>
          </UiCardHeader>
          <UiCardContent class="pb-3">
            <div class="space-y-3">
              <!-- 进度条 -->
              <div class="space-y-1">
                <div class="flex justify-between text-sm">
                  <span>进度</span>
                  <span>{{ item.progress }}%</span>
                </div>
                <UiProgress :value="item.progress" class="w-full" />
              </div>

              <!-- 标签 -->
              <div class="flex flex-wrap gap-1">
                <UiBadge variant="outline" class="text-xs">
                  {{ item.category }}
                </UiBadge>
                <UiBadge variant="outline" class="text-xs">
                  {{ item.priority }}
                </UiBadge>
              </div>

              <!-- 成员 -->
              <div class="flex items-center justify-between text-sm text-muted-foreground">
                <div class="flex items-center gap-1">
                  <UiUsers class="w-3 h-3" />
                  <span>{{ item.members.length }} 人</span>
                </div>
                <div class="flex items-center gap-1">
                  <UiCalendar class="w-3 h-3" />
                  <span>{{ formatDate(item.dueDate) }}</span>
                </div>
              </div>
            </div>
          </UiCardContent>
          <UiCardFooter class="pt-0">
            <div class="flex justify-between w-full">
              <div class="flex -space-x-2">
                <div
                  v-for="(member, index) in item.members.slice(0, 3)"
                  :key="member.id"
                  class="w-6 h-6 rounded-full bg-primary/10 border-2 border-background flex items-center justify-center text-xs font-medium text-primary"
                  :style="{ zIndex: 10 - index }"
                >
                  {{ member.name.charAt(0).toUpperCase() }}
                </div>
                <div
                  v-if="item.members.length > 3"
                  class="w-6 h-6 rounded-full bg-muted border-2 border-background flex items-center justify-center text-xs text-muted-foreground"
                >
                  +{{ item.members.length - 3 }}
                </div>
              </div>
              <div class="flex gap-1">
                <UiButton variant="ghost" size="sm" @click.stop="editItem(item)">
                  <UiEdit class="w-3 h-3" />
                </UiButton>
                <UiButton
                  variant="ghost"
                  size="sm"
                  class="text-destructive hover:text-destructive"
                  @click.stop="deleteItem(item)"
                >
                  <UiTrash2 class="w-3 h-3" />
                </UiButton>
              </div>
            </div>
          </UiCardFooter>
        </UiCard>
      </div>

      <!-- 列表视图 -->
      <div v-else class="space-y-4">
        <UiCard>
          <UiCardContent class="p-0">
            <div class="divide-y">
              <div
                v-for="item in filteredItems"
                :key="item.id"
                class="p-4 hover:bg-muted/50 transition-colors cursor-pointer"
                @click="viewItem(item)"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-4 flex-1">
                    <div
                      class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center"
                    >
                      <component
                        :is="getCategoryIcon(item.category)"
                        class="w-5 h-5 text-primary"
                      />
                    </div>
                    <div class="flex-1 min-w-0">
                      <div class="flex items-center gap-2 mb-1">
                        <h3 class="font-medium text-sm truncate">{{ item.title }}</h3>
                        <UiBadge :variant="getStatusVariant(item.status)" class="shrink-0">
                          {{ getStatusText(item.status) }}
                        </UiBadge>
                      </div>
                      <p class="text-sm text-muted-foreground truncate">{{ item.description }}</p>
                      <div class="flex items-center gap-4 mt-2 text-xs text-muted-foreground">
                        <div class="flex items-center gap-1">
                          <UiCalendar class="w-3 h-3" />
                          <span>{{ formatDate(item.dueDate) }}</span>
                        </div>
                        <div class="flex items-center gap-1">
                          <UiUsers class="w-3 h-3" />
                          <span>{{ item.members.length }} 人</span>
                        </div>
                        <UiBadge variant="outline">{{ item.category }}</UiBadge>
                        <UiBadge variant="outline">{{ item.priority }}</UiBadge>
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2 ml-4">
                    <!-- 进度 -->
                    <div class="w-20 text-right">
                      <div class="text-sm font-medium">{{ item.progress }}%</div>
                      <UiProgress :value="item.progress" class="w-full mt-1" />
                    </div>
                    <!-- 操作按钮 -->
                    <div class="flex gap-1">
                      <UiButton variant="ghost" size="sm" @click.stop="editItem(item)">
                        <UiEdit class="w-3 h-3" />
                      </UiButton>
                      <UiButton
                        variant="ghost"
                        size="sm"
                        class="text-destructive hover:text-destructive"
                        @click.stop="deleteItem(item)"
                      >
                        <UiTrash2 class="w-3 h-3" />
                      </UiButton>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 空状态 -->
            <div
              v-if="filteredItems.length === 0"
              class="flex flex-col items-center justify-center py-12 text-center"
            >
              <UiFolderOpen class="w-12 h-12 text-muted-foreground mb-4" />
              <h3 class="text-lg font-medium">暂无项目</h3>
              <p class="text-sm text-muted-foreground mt-1">没有找到匹配的项目数据</p>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 分页控件 -->
      <div
        v-if="filteredItems.length > 0"
        class="flex flex-col sm:flex-row items-center justify-between gap-4"
      >
        <div class="text-sm text-muted-foreground">共 {{ filteredItems.length }} 个项目</div>
        <div class="flex items-center gap-2">
          <UiButton
            variant="outline"
            size="sm"
            :disabled="currentPage === 1"
            @click="currentPage--"
          >
            上一页
          </UiButton>
          <div class="text-sm text-muted-foreground">第 {{ currentPage }} 页</div>
          <UiButton
            variant="outline"
            size="sm"
            :disabled="currentPage * itemsPerPage >= filteredItems.length"
            @click="currentPage++"
          >
            下一页
          </UiButton>
        </div>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import { Grid, List, FolderOpen, Palette, Code, Megaphone, Search } from 'lucide-vue-next'

interface Member {
  id: number
  name: string
  avatar?: string
}

interface ProjectItem {
  id: number
  title: string
  description: string
  category: string
  status: 'active' | 'completed' | 'pending'
  priority: string
  progress: number
  dueDate: string
  members: Member[]
}

// 模拟数据
const mockItems: ProjectItem[] = [
  {
    id: 1,
    title: '用户界面设计',
    description: '设计新的用户界面，提升用户体验和视觉吸引力',
    category: 'design',
    status: 'active',
    priority: '高',
    progress: 75,
    dueDate: '2024-02-15T00:00:00Z',
    members: [
      { id: 1, name: '张三' },
      { id: 2, name: '李四' },
      { id: 3, name: '王五' },
    ],
  },
  {
    id: 2,
    title: '后端API开发',
    description: '开发用户管理模块的RESTful API接口',
    category: 'development',
    status: 'active',
    priority: '高',
    progress: 60,
    dueDate: '2024-02-20T00:00:00Z',
    members: [
      { id: 4, name: '赵六' },
      { id: 5, name: '钱七' },
    ],
  },
  {
    id: 3,
    title: '市场推广活动',
    description: '策划并执行春季产品推广活动',
    category: 'marketing',
    status: 'pending',
    priority: '中',
    progress: 30,
    dueDate: '2024-03-01T00:00:00Z',
    members: [
      { id: 6, name: '孙八' },
      { id: 7, name: '周九' },
      { id: 8, name: '吴十' },
    ],
  },
  {
    id: 4,
    title: '用户行为研究',
    description: '分析用户行为数据，优化产品功能',
    category: 'research',
    status: 'completed',
    priority: '低',
    progress: 100,
    dueDate: '2024-01-31T00:00:00Z',
    members: [{ id: 9, name: '郑一' }],
  },
  {
    id: 5,
    title: '移动端应用开发',
    description: '开发iOS和Android平台的移动应用',
    category: 'development',
    status: 'active',
    priority: '高',
    progress: 45,
    dueDate: '2024-03-15T00:00:00Z',
    members: [
      { id: 10, name: '王二' },
      { id: 11, name: '张三' },
    ],
  },
  {
    id: 6,
    title: '品牌视觉设计',
    description: '更新品牌视觉识别系统，包括Logo和色彩方案',
    category: 'design',
    status: 'pending',
    priority: '中',
    progress: 20,
    dueDate: '2024-02-28T00:00:00Z',
    members: [
      { id: 12, name: '李四' },
      { id: 13, name: '王五' },
    ],
  },
]

// 响应式数据
const items = ref<ProjectItem[]>([])
const searchQuery = ref('')
const categoryFilter = ref('all')
const statusFilter = ref('all')
const viewMode = ref<'grid' | 'list'>('grid')
const currentPage = ref(1)
const itemsPerPage = 6
const showCreateDialog = ref(false)

// 计算属性
const filteredItems = computed(() => {
  let filtered = items.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (item) =>
        item.title.toLowerCase().includes(query) || item.description.toLowerCase().includes(query),
    )
  }

  // 分类过滤
  if (categoryFilter.value !== 'all') {
    filtered = filtered.filter((item) => item.category === categoryFilter.value)
  }

  // 状态过滤
  if (statusFilter.value !== 'all') {
    filtered = filtered.filter((item) => item.status === statusFilter.value)
  }

  return filtered
})

// 方法
const getStatusVariant = (status: string) => {
  switch (status) {
    case 'active':
      return 'default'
    case 'completed':
      return 'secondary'
    case 'pending':
      return 'outline'
    default:
      return 'outline'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'active':
      return '进行中'
    case 'completed':
      return '已完成'
    case 'pending':
      return '待处理'
    default:
      return status
  }
}

const getCategoryIcon = (category: string) => {
  switch (category) {
    case 'design':
      return Palette
    case 'development':
      return Code
    case 'marketing':
      return Megaphone
    case 'research':
      return Search
    default:
      return FolderOpen
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('zh-CN')
}

const toggleViewMode = () => {
  viewMode.value = viewMode.value === 'grid' ? 'list' : 'grid'
}

const viewItem = (item: ProjectItem) => {
  console.log('查看项目:', item)
  // 这里可以打开项目详情页面
}

const editItem = (item: ProjectItem) => {
  console.log('编辑项目:', item)
  // 这里可以打开编辑对话框
}

const deleteItem = (item: ProjectItem) => {
  console.log('删除项目:', item)
  // 这里可以显示确认删除对话框
}

// 生命周期
onMounted(() => {
  // 模拟从API获取数据
  setTimeout(() => {
    items.value = mockItems
  }, 500)
})

// 监听搜索和筛选变化，重置页码
watch([searchQuery, categoryFilter, statusFilter], () => {
  currentPage.value = 1
})
</script>

<style scoped></style>
