<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="任务管理" description="查看和管理您的任务列表" sticky>
    <div class="max-w-6xl mx-auto space-y-8">
      <!-- 任务统计 -->
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总任务数</UiCardTitle>
            <ListTodo class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">42</div>
            <p class="text-xs text-muted-foreground">+5 较上周</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">待处理</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">18</div>
            <p class="text-xs text-muted-foreground">需要关注</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">进行中</UiCardTitle>
            <Play class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">12</div>
            <p class="text-xs text-muted-foreground">活跃任务</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">已完成</UiCardTitle>
            <CheckCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">12</div>
            <p class="text-xs text-muted-foreground">29% 完成率</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 任务列表 -->
      <UiCard>
        <UiCardHeader>
          <div class="flex items-center justify-between">
            <div>
              <UiCardTitle>任务列表</UiCardTitle>
              <UiCardDescription> 管理您的所有任务和待办事项 </UiCardDescription>
            </div>
            <UiButton>
              <Plus class="w-4 h-4 mr-2" />
              新建任务
            </UiButton>
          </div>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <!-- 任务项 -->
            <div
              v-for="task in tasks"
              :key="task.id"
              class="flex items-center justify-between p-4 border rounded-lg"
            >
              <div class="flex items-center space-x-4">
                <UiCheckbox :checked="task.completed" />
                <div class="space-y-1">
                  <p
                    class="text-sm font-medium leading-none"
                    :class="{ 'line-through text-muted-foreground': task.completed }"
                  >
                    {{ task.title }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{ task.description }}
                  </p>
                </div>
              </div>
              <div class="flex items-center space-x-2">
                <UiBadge :variant="getStatusVariant(task.status)">
                  {{ task.status }}
                </UiBadge>
                <UiButton variant="ghost" size="sm">
                  <Edit class="w-4 h-4" />
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 快速操作 -->
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <UiCard class="cursor-pointer transition-all hover:shadow-lg">
          <UiCardContent class="p-6">
            <div class="flex flex-col items-center text-center space-y-4">
              <div class="p-3 rounded-full bg-primary/10">
                <Filter class="w-6 h-6 text-primary" />
              </div>
              <div>
                <h3 class="font-semibold mb-2">任务筛选</h3>
                <p class="text-sm text-muted-foreground">按状态、优先级筛选任务</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
        <UiCard class="cursor-pointer transition-all hover:shadow-lg">
          <UiCardContent class="p-6">
            <div class="flex flex-col items-center text-center space-y-4">
              <div class="p-3 rounded-full bg-green-500/10">
                <Calendar class="w-6 h-6 text-green-500" />
              </div>
              <div>
                <h3 class="font-semibold mb-2">日程视图</h3>
                <p class="text-sm text-muted-foreground">按时间查看任务安排</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
        <UiCard class="cursor-pointer transition-all hover:shadow-lg">
          <UiCardContent class="p-6">
            <div class="flex flex-col items-center text-center space-y-4">
              <div class="p-3 rounded-full bg-blue-500/10">
                <BarChart3 class="w-6 h-6 text-blue-500" />
              </div>
              <div>
                <h3 class="font-semibold mb-2">任务统计</h3>
                <p class="text-sm text-muted-foreground">查看任务完成情况统计</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>
    </div>
  </Page>
</template>

<script setup lang="ts">
import {
  BarChart3,
  Calendar,
  CheckCircle,
  Clock,
  Edit,
  Filter,
  ListTodo,
  Play,
  Plus,
} from 'lucide-vue-next'
import Page from '@/components/global-layout/basic-page.vue'

// 示例任务数据
const tasks = ref([
  {
    id: 1,
    title: '完成项目需求文档',
    description: '编写详细的项目需求规格说明书',
    status: '进行中',
    completed: false,
  },
  {
    id: 2,
    title: '修复登录页面bug',
    description: '解决用户登录时的验证码显示问题',
    status: '待处理',
    completed: false,
  },
  {
    id: 3,
    title: '优化数据库查询性能',
    description: '分析并优化慢查询语句',
    status: '已完成',
    completed: true,
  },
  {
    id: 4,
    title: '设计用户反馈系统',
    description: '创建用户反馈收集和处理流程',
    status: '进行中',
    completed: false,
  },
])

// 获取状态对应的样式变体
function getStatusVariant(status: string) {
  switch (status) {
    case '已完成':
      return 'default'
    case '进行中':
      return 'secondary'
    case '待处理':
      return 'destructive'
    default:
      return 'outline'
  }
}
</script>
