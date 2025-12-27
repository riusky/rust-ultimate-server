<template>
  <Page
    :title="transformI18n('system.operation-logs.title')"
    :description="transformI18n('system.operation-logs.description')"
    sticky
  >
    <!-- 操作日志统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总操作数</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">2,458</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">成功操作</UiCardTitle>
          <CheckCircle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">2,435</div>
          <p class="text-xs text-muted-foreground">正常执行</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">失败操作</UiCardTitle>
          <XCircle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">23</div>
          <p class="text-xs text-muted-foreground">执行失败</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃用户</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">42</div>
          <p class="text-xs text-muted-foreground">今日操作</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 操作日志列表 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>操作日志</UiCardTitle>
          <UiCardDescription>系统操作执行记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="log in operationLogs"
              :key="log.id"
              class="flex items-center justify-between p-4 rounded-lg border"
              :class="{
                'border-success/20 bg-success/10': log.status === 'success',
                'border-destructive/20 bg-destructive/10': log.status === 'failed',
                'border-warning/20 bg-warning/10': log.status === 'warning',
              }"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ log.operation }}</h3>
                    <UiBadge :variant="getStatusColor(log.status) as any">
                      {{ getStatusText(log.status) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ log.module }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground mb-2">
                    用户: {{ log.user }} • 执行时间: {{ log.executionTime }}ms
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{ log.description }}
                  </p>
                  <div class="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                    <span>时间: {{ log.timestamp }}</span>
                    <span>IP: {{ log.ip }}</span>
                  </div>
                </div>
              </div>
              <div class="flex flex-col space-y-2">
                <UiButton variant="outline" size="sm">
                  <ExternalLink class="h-4 w-4 mr-1" />
                  详情
                </UiButton>
                <UiButton variant="outline" size="sm">
                  <Download class="h-4 w-4 mr-1" />
                  重试
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 日志配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>日志配置</UiCardTitle>
          <UiCardDescription>操作日志收集和存储设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="retention-period">保留周期</UiLabel>
              <UiSelect v-model="logConfig.retentionPeriod">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="30">30天</UiSelectItem>
                  <UiSelectItem value="90">90天</UiSelectItem>
                  <UiSelectItem value="365">1年</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div>
              <UiLabel for="log-level">日志级别</UiLabel>
              <UiSelect v-model="logConfig.logLevel">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="info">信息</UiSelectItem>
                  <UiSelectItem value="debug">调试</UiSelectItem>
                  <UiSelectItem value="verbose">详细</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="user-ops" v-model="logConfig.userOperations" />
                <UiLabel for="user-ops">用户操作日志</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="system-ops" v-model="logConfig.systemOperations" />
                <UiLabel for="system-ops">系统操作日志</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="api-ops" v-model="logConfig.apiOperations" />
                <UiLabel for="api-ops">API 调用日志</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="performance" v-model="logConfig.performanceMetrics" />
                <UiLabel for="performance">性能指标</UiLabel>
              </div>
            </div>
            <div class="pt-4">
              <UiButton class="w-full" @click="exportLogs">
                <Download class="h-4 w-4 mr-2" />
                导出日志
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作统计 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>操作统计</UiCardTitle>
          <UiCardDescription>操作类型和执行结果统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-1">1,245</div>
              <p class="text-sm text-muted-foreground">用户操作</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-2">856</div>
              <p class="text-sm text-muted-foreground">系统操作</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-3">357</div>
              <p class="text-sm text-muted-foreground">API 调用</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-4">23</div>
              <p class="text-sm text-muted-foreground">失败操作</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline">
          <RefreshCw class="h-4 w-4 mr-2" />
          刷新日志
        </UiButton>
        <UiButton>
          <Save class="h-4 w-4 mr-2" />
          保存配置
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const operationLogs = ref([
  {
    id: 1,
    operation: '用户登录',
    status: 'success',
    module: '认证',
    user: 'admin',
    executionTime: 125,
    description: '管理员用户成功登录系统',
    timestamp: '2024-01-15 14:30:22',
    ip: '192.168.1.101',
  },
  {
    id: 2,
    operation: '数据备份',
    status: 'success',
    module: '备份',
    user: 'system',
    executionTime: 2450,
    description: '系统自动执行数据库备份',
    timestamp: '2024-01-15 14:28:15',
    ip: '192.168.1.100',
  },
  {
    id: 3,
    operation: '权限修改',
    status: 'warning',
    module: '权限',
    user: 'admin',
    executionTime: 89,
    description: '修改用户权限配置',
    timestamp: '2024-01-15 14:25:33',
    ip: '192.168.1.101',
  },
  {
    id: 4,
    operation: '文件上传',
    status: 'failed',
    module: '文件',
    user: 'user01',
    executionTime: 156,
    description: '文件上传失败，磁盘空间不足',
    timestamp: '2024-01-15 14:20:45',
    ip: '192.168.1.50',
  },
])

const logConfig = ref({
  retentionPeriod: '90',
  logLevel: 'info',
  userOperations: true,
  systemOperations: true,
  apiOperations: true,
  performanceMetrics: false,
})

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    success: 'success',
    failed: 'destructive',
    warning: 'warning',
  }
  return colors[status] || 'default'
}

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    success: '成功',
    failed: '失败',
    warning: '警告',
  }
  return texts[status] || status
}

const exportLogs = () => {
  console.log('导出操作日志')
}
</script>

<style></style>
