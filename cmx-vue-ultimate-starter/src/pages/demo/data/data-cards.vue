<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="数据卡片" description="这是一个数据卡片展示示例，支持多种卡片布局和交互">
    <div class="max-w-7xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">数据卡片</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          使用卡片形式展示数据和信息，支持多种布局和交互方式
        </p>
      </div>

      <!-- 统计卡片 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">统计卡片</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <UiCard v-for="stat in stats" :key="stat.title" class="relative overflow-hidden">
            <UiCardContent class="p-6">
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium text-muted-foreground">{{ stat.title }}</p>
                  <p class="text-2xl font-bold">{{ stat.value }}</p>
                  <div
                    class="flex items-center gap-1 text-xs"
                    :class="stat.trend > 0 ? 'text-green-600' : 'text-red-600'"
                  >
                    <UiTrendingUp v-if="stat.trend > 0" class="w-3 h-3" />
                    <UiTrendingDown v-else class="w-3 h-3" />
                    <span>{{ Math.abs(stat.trend) }}%</span>
                    <span class="text-muted-foreground">较上月</span>
                  </div>
                </div>
                <div class="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
                  <component :is="stat.icon" class="w-6 h-6 text-primary" />
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 信息卡片 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">信息卡片</h2>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <UiCard
            v-for="info in infoCards"
            :key="info.title"
            class="hover:shadow-md transition-shadow"
          >
            <UiCardHeader class="pb-3">
              <div class="flex items-center justify-between">
                <UiCardTitle class="text-lg">{{ info.title }}</UiCardTitle>
                <UiBadge :variant="info.status === 'active' ? 'default' : 'secondary'">
                  {{ info.status === 'active' ? '活跃' : '待处理' }}
                </UiBadge>
              </div>
              <UiCardDescription>{{ info.description }}</UiCardDescription>
            </UiCardHeader>
            <UiCardContent class="space-y-4">
              <div class="flex items-center justify-between text-sm">
                <span class="text-muted-foreground">负责人</span>
                <span class="font-medium">{{ info.owner }}</span>
              </div>
              <div class="flex items-center justify-between text-sm">
                <span class="text-muted-foreground">截止日期</span>
                <span class="font-medium">{{ info.dueDate }}</span>
              </div>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-muted-foreground">进度</span>
                  <span>{{ info.progress }}%</span>
                </div>
                <UiProgress :value="info.progress" class="w-full" />
              </div>
            </UiCardContent>
            <UiCardFooter class="pt-3">
              <div class="flex gap-2 w-full">
                <UiButton size="sm" class="flex-1">查看详情</UiButton>
                <UiButton variant="outline" size="sm">编辑</UiButton>
              </div>
            </UiCardFooter>
          </UiCard>
        </div>
      </div>

      <!-- 用户卡片 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">用户卡片</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          <UiCard
            v-for="user in users"
            :key="user.id"
            class="text-center hover:shadow-md transition-shadow"
          >
            <UiCardContent class="p-6">
              <div class="flex flex-col items-center space-y-4">
                <!-- 用户头像 -->
                <div
                  class="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center text-primary text-xl font-bold"
                >
                  {{ user.name.charAt(0).toUpperCase() }}
                </div>

                <!-- 用户信息 -->
                <div class="space-y-1">
                  <h3 class="font-semibold">{{ user.name }}</h3>
                  <p class="text-sm text-muted-foreground">{{ user.role }}</p>
                  <p class="text-xs text-muted-foreground">{{ user.email }}</p>
                </div>

                <!-- 状态标签 -->
                <UiBadge
                  :variant="
                    user.status === 'active'
                      ? 'default'
                      : user.status === 'inactive'
                        ? 'secondary'
                        : 'destructive'
                  "
                >
                  {{
                    user.status === 'active'
                      ? '活跃'
                      : user.status === 'inactive'
                        ? '非活跃'
                        : '禁用'
                  }}
                </UiBadge>

                <!-- 技能标签 -->
                <div class="flex flex-wrap gap-1 justify-center">
                  <UiBadge
                    v-for="skill in user.skills.slice(0, 3)"
                    :key="skill"
                    variant="outline"
                    class="text-xs"
                  >
                    {{ skill }}
                  </UiBadge>
                  <UiBadge v-if="user.skills.length > 3" variant="outline" class="text-xs">
                    +{{ user.skills.length - 3 }}
                  </UiBadge>
                </div>

                <!-- 操作按钮 -->
                <div class="flex gap-2 w-full">
                  <UiButton size="sm" class="flex-1">联系</UiButton>
                  <UiButton variant="outline" size="sm">
                    <UiUserPlus class="w-3 h-3" />
                  </UiButton>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 产品卡片 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">产品卡片</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <UiCard
            v-for="product in products"
            :key="product.id"
            class="overflow-hidden hover:shadow-lg transition-all"
          >
            <div class="aspect-video bg-muted relative">
              <div class="absolute inset-0 flex items-center justify-center text-muted-foreground">
                <UiImage class="w-12 h-12" />
              </div>
              <div class="absolute top-3 right-3">
                <UiBadge variant="secondary">{{ product.category }}</UiBadge>
              </div>
            </div>
            <UiCardHeader class="pb-3">
              <div class="flex items-start justify-between">
                <div>
                  <UiCardTitle class="text-lg">{{ product.name }}</UiCardTitle>
                  <UiCardDescription class="mt-1">{{ product.description }}</UiCardDescription>
                </div>
                <div class="text-right">
                  <p class="text-2xl font-bold text-primary">¥{{ product.price }}</p>
                  <p
                    class="text-xs text-muted-foreground line-through"
                    v-if="product.originalPrice"
                  >
                    ¥{{ product.originalPrice }}
                  </p>
                </div>
              </div>
            </UiCardHeader>
            <UiCardContent class="space-y-3">
              <div class="flex items-center justify-between text-sm">
                <span class="text-muted-foreground">库存</span>
                <span class="font-medium">{{ product.stock }} 件</span>
              </div>
              <div class="flex items-center justify-between text-sm">
                <span class="text-muted-foreground">评分</span>
                <div class="flex items-center gap-1">
                  <UiStar class="w-3 h-3 fill-yellow-400 text-yellow-400" />
                  <span class="font-medium">{{ product.rating }}</span>
                  <span class="text-muted-foreground">({{ product.reviews }})</span>
                </div>
              </div>
            </UiCardContent>
            <UiCardFooter class="pt-3">
              <div class="flex gap-2 w-full">
                <UiButton size="sm" class="flex-1">加入购物车</UiButton>
                <UiButton variant="outline" size="sm">
                  <UiHeart class="w-4 h-4" />
                </UiButton>
              </div>
            </UiCardFooter>
          </UiCard>
        </div>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import { Users, ShoppingCart, DollarSign, Activity } from 'lucide-vue-next'

// 统计数据
const stats = [
  {
    title: '总用户数',
    value: '12,458',
    trend: 12.5,
    icon: Users,
  },
  {
    title: '总销售额',
    value: '¥89,245',
    trend: 8.2,
    icon: DollarSign,
  },
  {
    title: '订单数量',
    value: '1,234',
    trend: -3.2,
    icon: ShoppingCart,
  },
  {
    title: '活跃用户',
    value: '8,932',
    trend: 15.7,
    icon: Activity,
  },
]

// 信息卡片数据
const infoCards = [
  {
    title: '项目管理系统',
    description: '开发新一代项目协作平台',
    status: 'active' as const,
    owner: '张三',
    dueDate: '2024-03-15',
    progress: 75,
  },
  {
    title: '移动端应用',
    description: 'iOS和Android平台应用开发',
    status: 'active' as const,
    owner: '李四',
    dueDate: '2024-04-20',
    progress: 45,
  },
  {
    title: '数据分析平台',
    description: '构建大数据分析和可视化平台',
    status: 'active' as const,
    owner: '王五',
    dueDate: '2024-05-10',
    progress: 30,
  },
  {
    title: 'API文档更新',
    description: '更新系统API接口文档',
    status: 'pending' as const,
    owner: '赵六',
    dueDate: '2024-02-28',
    progress: 90,
  },
]

// 用户数据
const users = [
  {
    id: 1,
    name: '张三',
    email: 'zhangsan@example.com',
    role: '前端工程师',
    status: 'active' as const,
    skills: ['Vue', 'React', 'TypeScript', 'JavaScript'],
  },
  {
    id: 2,
    name: '李四',
    email: 'lisi@example.com',
    role: '后端工程师',
    status: 'active' as const,
    skills: ['Node.js', 'Python', 'MySQL', 'Redis'],
  },
  {
    id: 3,
    name: '王五',
    email: 'wangwu@example.com',
    role: 'UI设计师',
    status: 'inactive' as const,
    skills: ['Figma', 'Sketch', 'Photoshop', 'Illustrator'],
  },
  {
    id: 4,
    name: '赵六',
    email: 'zhaoliu@example.com',
    role: '产品经理',
    status: 'active' as const,
    skills: ['产品设计', '用户研究', '数据分析', '项目管理'],
  },
  {
    id: 5,
    name: '钱七',
    email: 'qianqi@example.com',
    role: '全栈工程师',
    status: 'active' as const,
    skills: ['Vue', 'Node.js', 'MongoDB', 'Docker'],
  },
  {
    id: 6,
    name: '孙八',
    email: 'sunba@example.com',
    role: '测试工程师',
    status: 'banned' as const,
    skills: ['自动化测试', '性能测试', '安全测试'],
  },
  {
    id: 7,
    name: '周九',
    email: 'zhoujiu@example.com',
    role: '运维工程师',
    status: 'active' as const,
    skills: ['Linux', 'Docker', 'Kubernetes', 'AWS'],
  },
  {
    id: 8,
    name: '吴十',
    email: 'wushi@example.com',
    role: '数据科学家',
    status: 'inactive' as const,
    skills: ['Python', '机器学习', '数据分析', 'SQL'],
  },
]

// 产品数据
const products = [
  {
    id: 1,
    name: '智能手表',
    description: '多功能健康监测智能手表',
    category: '电子产品',
    price: 1299,
    originalPrice: 1599,
    stock: 45,
    rating: 4.8,
    reviews: 234,
  },
  {
    id: 2,
    name: '无线耳机',
    description: '降噪蓝牙无线耳机',
    category: '电子产品',
    price: 699,
    originalPrice: null,
    stock: 89,
    rating: 4.6,
    reviews: 156,
  },
  {
    id: 3,
    name: '笔记本电脑',
    description: '轻薄便携商务笔记本',
    category: '电子产品',
    price: 5999,
    originalPrice: 6999,
    stock: 12,
    rating: 4.9,
    reviews: 89,
  },
  {
    id: 4,
    name: '机械键盘',
    description: 'RGB背光机械键盘',
    category: '外设',
    price: 399,
    originalPrice: null,
    stock: 67,
    rating: 4.7,
    reviews: 123,
  },
  {
    id: 5,
    name: '显示器',
    description: '27英寸4K高清显示器',
    category: '外设',
    price: 1999,
    originalPrice: 2499,
    stock: 23,
    rating: 4.8,
    reviews: 78,
  },
  {
    id: 6,
    name: '游戏鼠标',
    description: '专业电竞游戏鼠标',
    category: '外设',
    price: 299,
    originalPrice: 399,
    stock: 56,
    rating: 4.5,
    reviews: 167,
  },
]
</script>

<style scoped></style>
