<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="使用教程" description="通过详细的教程学习如何使用系统的各项功能">
    <div class="max-w-6xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">使用教程</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          从基础操作到高级功能，逐步学习如何充分利用系统
        </p>
      </div>

      <!-- 教程分类 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">教程分类</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <UiCard
            v-for="category in tutorialCategories"
            :key="category.id"
            class="cursor-pointer hover:shadow-md transition-shadow"
            @click="selectedCategory = category.id"
          >
            <UiCardContent class="p-6">
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
                  <component :is="category.icon" class="w-6 h-6 text-primary" />
                </div>
                <div>
                  <h3 class="font-semibold">{{ category.title }}</h3>
                  <p class="text-sm text-muted-foreground mt-1">{{ category.description }}</p>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 教程列表 -->
      <div class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold">教程列表</h2>
          <div class="flex items-center gap-2">
            <UiSelect v-model="difficultyFilter">
              <UiSelectTrigger class="w-32">
                <UiSelectValue placeholder="难度筛选" />
              </UiSelectTrigger>
              <UiSelectContent>
                <UiSelectItem value="all">全部难度</UiSelectItem>
                <UiSelectItem value="beginner">初级</UiSelectItem>
                <UiSelectItem value="intermediate">中级</UiSelectItem>
                <UiSelectItem value="advanced">高级</UiSelectItem>
              </UiSelectContent>
            </UiSelect>
          </div>
        </div>

        <div class="space-y-4">
          <UiCard
            v-for="tutorial in filteredTutorials"
            :key="tutorial.id"
            class="hover:shadow-md transition-shadow"
          >
            <UiCardContent class="p-6">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <h3 class="text-lg font-semibold">{{ tutorial.title }}</h3>
                    <UiBadge :variant="getDifficultyVariant(tutorial.difficulty)">
                      {{ getDifficultyText(tutorial.difficulty) }}
                    </UiBadge>
                    <UiBadge variant="outline">
                      {{ tutorial.duration }}
                    </UiBadge>
                  </div>
                  <p class="text-muted-foreground mb-4">{{ tutorial.description }}</p>

                  <!-- 教程步骤预览 -->
                  <div class="space-y-2">
                    <div
                      v-for="(step, index) in tutorial.steps.slice(0, 3)"
                      :key="index"
                      class="flex items-center gap-3 text-sm"
                    >
                      <div
                        class="w-6 h-6 rounded-full bg-muted flex items-center justify-center text-xs font-medium"
                      >
                        {{ index + 1 }}
                      </div>
                      <span>{{ step }}</span>
                    </div>
                    <div
                      v-if="tutorial.steps.length > 3"
                      class="text-sm text-muted-foreground pl-9"
                    >
                      还有 {{ tutorial.steps.length - 3 }} 个步骤...
                    </div>
                  </div>
                </div>

                <div class="flex flex-col gap-2 ml-4">
                  <UiButton @click="startTutorial(tutorial)">
                    <UiPlay class="w-4 h-4 mr-2" />
                    开始学习
                  </UiButton>
                  <UiButton variant="outline" @click="viewTutorialDetails(tutorial)">
                    查看详情
                  </UiButton>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>

        <!-- 空状态 -->
        <div
          v-if="filteredTutorials.length === 0"
          class="flex flex-col items-center justify-center py-12 text-center border-2 border-dashed rounded-lg"
        >
          <div class="w-16 h-16 mb-4 rounded-full bg-muted flex items-center justify-center">
            <UiBookOpen class="w-8 h-8 text-muted-foreground" />
          </div>
          <h3 class="text-lg font-medium mb-2">没有找到教程</h3>
          <p class="text-muted-foreground">尝试调整筛选条件</p>
        </div>
      </div>

      <!-- 学习进度 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>学习进度</UiCardTitle>
          <UiCardDescription> 跟踪您的学习进度，继续完成未完成的教程 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="progress in learningProgress"
              :key="progress.tutorialId"
              class="flex items-center justify-between p-4 border rounded-lg"
            >
              <div class="flex items-center gap-4 flex-1">
                <div class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                  <UiBookOpen class="w-5 h-5 text-primary" />
                </div>
                <div class="flex-1">
                  <h4 class="font-medium">{{ getTutorialTitle(progress.tutorialId) }}</h4>
                  <div class="flex items-center gap-2 mt-1">
                    <UiProgress :value="progress.progress" class="w-32" />
                    <span class="text-sm text-muted-foreground">{{ progress.progress }}%</span>
                  </div>
                </div>
              </div>
              <UiButton variant="outline" @click="continueTutorial(progress.tutorialId)">
                继续学习
              </UiButton>
            </div>

            <div
              v-if="learningProgress.length === 0"
              class="text-center py-8 text-muted-foreground"
            >
              <UiBookOpen class="w-12 h-12 mx-auto mb-3 opacity-50" />
              <p>还没有开始任何教程</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import { BookOpen, User, CreditCard, BarChart3, Shield, MessageCircle } from 'lucide-vue-next'

// 教程分类
const tutorialCategories = [
  {
    id: 'getting-started',
    title: '入门指南',
    description: '新用户快速上手',
    icon: BookOpen,
  },
  {
    id: 'account',
    title: '账户管理',
    description: '账户设置和安全管理',
    icon: User,
  },
  {
    id: 'billing',
    title: '账单支付',
    description: '订阅和支付管理',
    icon: CreditCard,
  },
  {
    id: 'analytics',
    title: '数据分析',
    description: '报表和数据分析',
    icon: BarChart3,
  },
  {
    id: 'security',
    title: '安全设置',
    description: '隐私和安全配置',
    icon: Shield,
  },
  {
    id: 'communication',
    title: '沟通协作',
    description: '团队沟通和协作',
    icon: MessageCircle,
  },
]

// 教程数据
const tutorials = [
  {
    id: 1,
    category: 'getting-started',
    title: '系统快速入门',
    description: '学习系统的基本操作和核心功能，快速上手使用',
    difficulty: 'beginner',
    duration: '15分钟',
    steps: [
      '注册并登录系统',
      '完成个人资料设置',
      '了解仪表板布局',
      '创建第一个项目',
      '邀请团队成员',
      '设置通知偏好',
    ],
  },
  {
    id: 2,
    category: 'account',
    title: '账户安全设置',
    description: '保护您的账户安全，设置双重验证和安全选项',
    difficulty: 'beginner',
    duration: '10分钟',
    steps: [
      '修改账户密码',
      '设置安全问题',
      '启用双重验证',
      '配置登录通知',
      '查看登录历史',
      '管理设备授权',
    ],
  },
  {
    id: 3,
    category: 'billing',
    title: '订阅管理指南',
    description: '了解如何管理您的订阅计划和支付方式',
    difficulty: 'intermediate',
    duration: '20分钟',
    steps: [
      '查看当前订阅计划',
      '升级或降级计划',
      '管理支付方式',
      '查看账单历史',
      '下载发票',
      '设置自动续费',
    ],
  },
  {
    id: 4,
    category: 'analytics',
    title: '数据报表分析',
    description: '学习如何创建和解读数据报表，获取业务洞察',
    difficulty: 'advanced',
    duration: '30分钟',
    steps: [
      '创建自定义报表',
      '设置数据筛选条件',
      '配置图表类型',
      '设置自动更新',
      '导出报表数据',
      '分享报表给团队',
    ],
  },
  {
    id: 5,
    category: 'security',
    title: '高级安全配置',
    description: '配置高级安全选项，保护敏感数据',
    difficulty: 'advanced',
    duration: '25分钟',
    steps: [
      '设置数据加密',
      '配置访问权限',
      '设置数据保留策略',
      '配置审计日志',
      '设置合规性要求',
      '定期安全检查',
    ],
  },
  {
    id: 6,
    category: 'communication',
    title: '团队协作指南',
    description: '学习如何使用系统的团队协作功能',
    difficulty: 'intermediate',
    duration: '18分钟',
    steps: [
      '创建团队空间',
      '邀请团队成员',
      '设置团队权限',
      '创建协作任务',
      '使用评论功能',
      '设置团队通知',
    ],
  },
]

// 学习进度数据
const learningProgress = [
  {
    tutorialId: 1,
    progress: 100,
    lastAccessed: '2024-01-20',
  },
  {
    tutorialId: 2,
    progress: 75,
    lastAccessed: '2024-01-21',
  },
  {
    tutorialId: 3,
    progress: 50,
    lastAccessed: '2024-01-19',
  },
]

// 响应式数据
const selectedCategory = ref('all')
const difficultyFilter = ref('all')

// 计算属性
const filteredTutorials = computed(() => {
  let filtered = tutorials

  // 分类过滤
  if (selectedCategory.value !== 'all') {
    filtered = filtered.filter((tutorial) => tutorial.category === selectedCategory.value)
  }

  // 难度过滤
  if (difficultyFilter.value !== 'all') {
    filtered = filtered.filter((tutorial) => tutorial.difficulty === difficultyFilter.value)
  }

  return filtered
})

// 方法
const getDifficultyVariant = (difficulty: string) => {
  switch (difficulty) {
    case 'beginner':
      return 'default'
    case 'intermediate':
      return 'secondary'
    case 'advanced':
      return 'destructive'
    default:
      return 'outline'
  }
}

const getDifficultyText = (difficulty: string) => {
  switch (difficulty) {
    case 'beginner':
      return '初级'
    case 'intermediate':
      return '中级'
    case 'advanced':
      return '高级'
    default:
      return difficulty
  }
}

const getTutorialTitle = (tutorialId: number) => {
  const tutorial = tutorials.find((t) => t.id === tutorialId)
  return tutorial ? tutorial.title : '未知教程'
}

const startTutorial = (tutorial: { title: string }) => {
  console.log('开始教程:', tutorial.title)
  // 这里可以实现开始教程的功能
}

const viewTutorialDetails = (tutorial: { title: string }) => {
  console.log('查看教程详情:', tutorial.title)
  // 这里可以实现查看教程详情的功能
}

const continueTutorial = (tutorialId: number) => {
  console.log('继续教程:', tutorialId)
  // 这里可以实现继续教程的功能
}
</script>

<style scoped></style>
