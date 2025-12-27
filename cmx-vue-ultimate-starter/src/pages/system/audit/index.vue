<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const auditLogs = ref([
  {
    id: 1,
    title: '用户登录成功',
    severity: 'low',
    category: '认证',
    user: 'admin',
    action: '登录',
    ip: '192.168.1.101',
    description: '管理员用户成功登录系统',
    timestamp: '2024-01-15 14:30:22',
    resource: '认证系统',
  },
  {
    id: 2,
    title: '权限变更',
    severity: 'medium',
    category: '权限',
    user: 'user01',
    action: '权限修改',
    ip: '192.168.1.50',
    description: '用户权限被管理员修改',
    timestamp: '2024-01-15 14:28:15',
    resource: '权限系统',
  },
  {
    id: 3,
    title: '异常登录尝试',
    severity: 'high',
    category: '安全',
    user: 'unknown',
    action: '登录失败',
    ip: '203.0.113.45',
    description: '检测到多次登录失败尝试',
    timestamp: '2024-01-15 14:25:33',
    resource: '认证系统',
  },
  {
    id: 4,
    title: '系统配置变更',
    severity: 'medium',
    category: '系统',
    user: 'admin',
    action: '配置修改',
    ip: '192.168.1.101',
    description: '系统网络配置被修改',
    timestamp: '2024-01-15 14:20:45',
    resource: '系统配置',
  },
])

const auditConfig = ref({
  retentionPeriod: '90',
  logLevel: 'standard',
  userAudit: true,
  systemAudit: true,
  networkAudit: true,
  fileAudit: false,
})

const getSeverityColor = (severity: string) => {
  const colors: Record<string, string> = {
    high: 'destructive',
    medium: 'warning',
    low: 'chart-3',
  }
  return colors[severity] || 'default'
}

const getSeverityText = (severity: string) => {
  const texts: Record<string, string> = {
    high: '高危',
    medium: '中危',
    low: '低危',
  }
  return texts[severity] || severity
}

const exportLogs = () => {
  console.log('导出审计日志')
}
</script>

<template>
  <Page
    :title="transformI18n('system.audit.title')"
    :description="transformI18n('system.audit.description')"
    sticky
  >
    <!-- 审计日志统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总审计事件</UiCardTitle>
          <FileText class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">1,245</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">异常事件</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">23</div>
          <p class="text-xs text-muted-foreground">需要关注</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">用户操作</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">856</div>
          <p class="text-xs text-muted-foreground">用户活动</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">系统操作</UiCardTitle>
          <Server class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">389</div>
          <p class="text-xs text-muted-foreground">系统活动</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 审计日志列表 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>审计日志</UiCardTitle>
          <UiCardDescription>系统审计事件记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="log in auditLogs"
              :key="log.id"
              class="flex items-center justify-between p-4 rounded-lg border"
              :class="{
                'border-destructive/20 bg-destructive/10': log.severity === 'high',
                'border-warning/20 bg-warning/10': log.severity === 'medium',
                'border-chart-3/20 bg-chart-3/10': log.severity === 'low',
              }"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ log.title }}</h3>
                    <UiBadge :variant="getSeverityColor(log.severity) as any">
                      {{ getSeverityText(log.severity) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ log.category }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground mb-2">
                    用户: {{ log.user }} • 操作: {{ log.action }} • IP: {{ log.ip }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{ log.description }}
                  </p>
                  <div class="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                    <span>时间: {{ log.timestamp }}</span>
                    <span>资源: {{ log.resource }}</span>
                  </div>
                </div>
              </div>
              <div class="flex flex-col space-y-2">
                <UiButton variant="outline" size="sm">
                  <ExternalLink class="h-4 w-4 mr-1" />
                  详情
                </UiButton>
                <UiButton variant="outline" size="sm">
                  <Flag class="h-4 w-4 mr-1" />
                  标记
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 审计配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>审计配置</UiCardTitle>
          <UiCardDescription>审计日志收集和存储设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="retention-period">保留周期</UiLabel>
              <UiSelect v-model="auditConfig.retentionPeriod">
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
              <UiSelect v-model="auditConfig.logLevel">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="minimal">最小</UiSelectItem>
                  <UiSelectItem value="standard">标准</UiSelectItem>
                  <UiSelectItem value="verbose">详细</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="user-audit" v-model="auditConfig.userAudit" />
                <UiLabel for="user-audit">用户行为审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="system-audit" v-model="auditConfig.systemAudit" />
                <UiLabel for="system-audit">系统操作审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="network-audit" v-model="auditConfig.networkAudit" />
                <UiLabel for="network-audit">网络访问审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="file-audit" v-model="auditConfig.fileAudit" />
                <UiLabel for="file-audit">文件访问审计</UiLabel>
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

      <!-- 审计统计 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>审计统计</UiCardTitle>
          <UiCardDescription>审计事件分类统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-1">856</div>
              <p class="text-sm text-muted-foreground">用户操作</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-2">389</div>
              <p class="text-sm text-muted-foreground">系统操作</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-3">156</div>
              <p class="text-sm text-muted-foreground">网络访问</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-4">23</div>
              <p class="text-sm text-muted-foreground">异常事件</p>
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

<style></style>
