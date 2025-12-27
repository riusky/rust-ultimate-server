<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="使用教程" description="系统使用教程和操作指南" sticky>
    <div class="max-w-4xl mx-auto space-y-8">
      <!-- 教程分类 -->
      <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        <UiCard
          v-for="tutorial in tutorials"
          :key="tutorial.title"
          class="cursor-pointer transition-all hover:shadow-lg"
        >
          <UiCardContent class="p-6">
            <div class="flex flex-col items-center text-center space-y-4">
              <div class="p-3 rounded-full bg-primary/10">
                <component :is="tutorial.icon" class="w-6 h-6 text-primary" />
              </div>
              <div>
                <h3 class="font-semibold mb-2">{{ tutorial.title }}</h3>
                <p class="text-sm text-muted-foreground">
                  {{ tutorial.description }}
                </p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 快速入门 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>快速入门</UiCardTitle>
          <UiCardDescription>系统基础操作指南</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="space-y-6">
          <div class="space-y-4">
            <div>
              <h3 class="font-semibold mb-2">1. 登录系统</h3>
              <p class="text-sm text-muted-foreground">
                使用您的账号密码登录系统，首次登录请使用管理员提供的初始密码。
              </p>
            </div>
            <div>
              <h3 class="font-semibold mb-2">2. 导航菜单</h3>
              <p class="text-sm text-muted-foreground">
                左侧导航菜单包含所有功能模块，点击相应菜单项进入对应页面。
              </p>
            </div>
            <div>
              <h3 class="font-semibold mb-2">3. 团队切换</h3>
              <p class="text-sm text-muted-foreground">
                点击顶部团队切换器可以切换不同的工作空间，每个团队有独立的配置和数据。
              </p>
            </div>
            <div>
              <h3 class="font-semibold mb-2">4. 个人设置</h3>
              <p class="text-sm text-muted-foreground">
                在设置页面可以修改个人信息、密码、通知偏好等配置。
              </p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 功能详解 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>功能详解</UiCardTitle>
          <UiCardDescription>各功能模块详细说明</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="space-y-6">
          <div class="space-y-4">
            <div>
              <h3 class="font-semibold mb-2">用户管理</h3>
              <p class="text-sm text-muted-foreground mb-2">
                管理系统的用户账号，包括创建用户、分配角色、设置权限等操作。
              </p>
              <ul class="text-sm text-muted-foreground space-y-1 ml-4">
                <li>• 点击"创建用户"按钮添加新用户</li>
                <li>• 使用搜索框快速查找用户</li>
                <li>• 点击操作菜单进行编辑或删除</li>
              </ul>
            </div>
            <div>
              <h3 class="font-semibold mb-2">任务管理</h3>
              <p class="text-sm text-muted-foreground mb-2">
                创建和管理工作任务，跟踪任务进度和状态。
              </p>
              <ul class="text-sm text-muted-foreground space-y-1 ml-4">
                <li>• 支持任务分类和优先级设置</li>
                <li>• 可设置截止日期和提醒</li>
                <li>• 支持任务分配和协作</li>
              </ul>
            </div>
            <div>
              <h3 class="font-semibold mb-2">数据管理</h3>
              <p class="text-sm text-muted-foreground mb-2">
                管理和展示各类数据，支持表格、列表、卡片等多种展示方式。
              </p>
              <ul class="text-sm text-muted-foreground space-y-1 ml-4">
                <li>• 支持数据搜索和筛选</li>
                <li>• 可自定义列显示和排序</li>
                <li>• 支持数据导出功能</li>
              </ul>
            </div>
            <div>
              <h3 class="font-semibold mb-2">文件管理</h3>
              <p class="text-sm text-muted-foreground mb-2">
                上传和管理文件，支持多种文件格式和批量操作。
              </p>
              <ul class="text-sm text-muted-foreground space-y-1 ml-4">
                <li>• 支持拖拽上传</li>
                <li>• 可设置文件分类和标签</li>
                <li>• 支持文件预览和下载</li>
              </ul>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 常见问题 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>常见问题</UiCardTitle>
          <UiCardDescription>用户常见问题解答</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="space-y-4">
          <div class="space-y-3">
            <div>
              <h4 class="font-medium">如何重置密码？</h4>
              <p class="text-sm text-muted-foreground">
                在登录页面点击"忘记密码"，按照提示输入邮箱地址，系统会发送重置链接到您的邮箱。
              </p>
            </div>
            <div>
              <h4 class="font-medium">如何导出数据？</h4>
              <p class="text-sm text-muted-foreground">
                在数据表格页面，点击右上角的"导出数据"按钮，选择导出格式即可下载数据文件。
              </p>
            </div>
            <div>
              <h4 class="font-medium">如何设置通知？</h4>
              <p class="text-sm text-muted-foreground">
                在设置页面的"通知设置"选项中，可以配置邮件、短信和推送通知的接收方式。
              </p>
            </div>
            <div>
              <h4 class="font-medium">系统支持哪些浏览器？</h4>
              <p class="text-sm text-muted-foreground">
                推荐使用 Chrome、Firefox、Safari、Edge 等现代浏览器的最新版本。
              </p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script setup lang="ts">
import { BookOpen, Users, FileText, Settings, HelpCircle, Download } from 'lucide-vue-next'
import Page from '@/components/global-layout/basic-page.vue'

// 教程分类
const tutorials = [
  {
    title: '快速入门',
    description: '系统基础操作和导航',
    icon: BookOpen,
  },
  {
    title: '用户管理',
    description: '用户账号和权限管理',
    icon: Users,
  },
  {
    title: '数据管理',
    description: '数据展示和操作指南',
    icon: FileText,
  },
  {
    title: '系统设置',
    description: '个性化配置和偏好设置',
    icon: Settings,
  },
  {
    title: '常见问题',
    description: '用户常见问题解答',
    icon: HelpCircle,
  },
  {
    title: '高级功能',
    description: '系统高级功能使用',
    icon: Download,
  },
]
</script>
