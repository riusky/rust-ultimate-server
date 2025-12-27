<template>
  <Page title="系统管理" description="管理系统配置、监控和安全设置" sticky>
    <div class="space-y-6">
      <!-- 系统概览 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>系统概览</UiCardTitle>
          <UiCardDescription>系统整体运行状态和关键指标</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <Server class="w-8 h-8 text-chart-3 mx-auto mb-2" />
              <p class="text-2xl font-bold">12</p>
              <p class="text-sm text-muted-foreground">服务器</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Users class="w-8 h-8 text-chart-2 mx-auto mb-2" />
              <p class="text-2xl font-bold">150</p>
              <p class="text-sm text-muted-foreground">用户</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <AlertTriangle class="w-8 h-8 text-chart-4 mx-auto mb-2" />
              <p class="text-2xl font-bold">23</p>
              <p class="text-sm text-muted-foreground">未处理告警</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Activity class="w-8 h-8 text-chart-5 mx-auto mb-2" />
              <p class="text-2xl font-bold">98.5%</p>
              <p class="text-sm text-muted-foreground">系统可用性</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 功能模块 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- 系统概览 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">系统概览</UiCardTitle>
            <Monitor class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">仪表板</div>
            <p class="text-xs text-muted-foreground">查看系统整体运行状态</p>
          </UiCardContent>
        </UiCard>

        <!-- 监控中心 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">监控中心</UiCardTitle>
            <Activity class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">系统监控</div>
            <p class="text-xs text-muted-foreground">实时监控系统性能指标</p>
          </UiCardContent>
        </UiCard>

        <!-- 服务器管理 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">服务器管理</UiCardTitle>
            <Server class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">服务器</div>
            <p class="text-xs text-muted-foreground">管理服务器和运行状态</p>
          </UiCardContent>
        </UiCard>

        <!-- 用户权限 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">用户权限</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">用户管理</div>
            <p class="text-xs text-muted-foreground">管理用户账户和权限</p>
          </UiCardContent>
        </UiCard>

        <!-- 安全管理 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">安全管理</UiCardTitle>
            <Shield class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">安全策略</div>
            <p class="text-xs text-muted-foreground">配置系统安全设置</p>
          </UiCardContent>
        </UiCard>

        <!-- 告警管理 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">告警管理</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">告警中心</div>
            <p class="text-xs text-muted-foreground">管理系统告警和通知</p>
          </UiCardContent>
        </UiCard>

        <!-- 系统设置 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">系统设置</UiCardTitle>
            <Settings class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">配置管理</div>
            <p class="text-xs text-muted-foreground">系统参数和全局设置</p>
          </UiCardContent>
        </UiCard>

        <!-- 数据库管理 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">数据库管理</UiCardTitle>
            <Database class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">数据库</div>
            <p class="text-xs text-muted-foreground">数据库配置和监控</p>
          </UiCardContent>
        </UiCard>

        <!-- 运维工具 -->
        <UiCard class="cursor-pointer hover:shadow-md transition-shadow">
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">运维工具</UiCardTitle>
            <Wrench class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">运维中心</div>
            <p class="text-xs text-muted-foreground">系统运维和维护工具</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 快速操作 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>快速操作</UiCardTitle>
          <UiCardDescription>常用系统管理操作</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="flex flex-wrap gap-4">
            <UiButton variant="outline">
              <UserPlus class="w-4 h-4 mr-2" />
              添加用户
            </UiButton>
            <UiButton variant="outline">
              <Server class="w-4 h-4 mr-2" />
              添加服务器
            </UiButton>
            <UiButton variant="outline">
              <Shield class="w-4 h-4 mr-2" />
              安全检查
            </UiButton>
            <UiButton variant="outline">
              <Download class="w-4 h-4 mr-2" />
              系统备份
            </UiButton>
            <UiButton variant="outline">
              <RefreshCw class="w-4 h-4 mr-2" />
              重启服务
            </UiButton>
            <UiButton variant="outline">
              <FileText class="w-4 h-4 mr-2" />
              查看日志
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import {
  Server,
  Users,
  AlertTriangle,
  Activity,
  Monitor,
  Shield,
  Settings,
  Database,
  Wrench,
  UserPlus,
  Download,
  RefreshCw,
  FileText,
} from 'lucide-vue-next'
</script>

<style scoped></style>
