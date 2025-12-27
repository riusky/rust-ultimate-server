<template>
  <Page
    :title="transformI18n('system.security-logs.title')"
    :description="transformI18n('system.security-logs.description')"
    sticky
  >
    <!-- 安全日志统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总安全事件</UiCardTitle>
          <Shield class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">156</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">高危事件</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">8</div>
          <p class="text-xs text-muted-foreground">需要立即处理</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">中危事件</UiCardTitle>
          <ShieldAlert class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-warning">23</div>
          <p class="text-xs text-muted-foreground">建议关注</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">低危事件</UiCardTitle>
          <Info class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-chart-3">125</div>
          <p class="text-xs text-muted-foreground">常规监控</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 安全日志列表 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>安全日志</UiCardTitle>
          <UiCardDescription>系统安全事件记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="log in securityLogs"
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
                    来源: {{ log.source }} • 类型: {{ log.type }} • IP: {{ log.ip }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{ log.description }}
                  </p>
                  <div class="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                    <span>时间: {{ log.timestamp }}</span>
                    <span>影响: {{ log.impact }}</span>
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
                  处理
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 安全配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>安全配置</UiCardTitle>
          <UiCardDescription>安全日志收集和告警设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="retention-period">保留周期</UiLabel>
              <UiSelect v-model="securityConfig.retentionPeriod">
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
              <UiLabel for="alert-level">告警级别</UiLabel>
              <UiSelect v-model="securityConfig.alertLevel">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="high">仅高危</UiSelectItem>
                  <UiSelectItem value="medium">中危及以上</UiSelectItem>
                  <UiSelectItem value="low">所有级别</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="login-audit" v-model="securityConfig.loginAudit" />
                <UiLabel for="login-audit">登录审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="access-audit" v-model="securityConfig.accessAudit" />
                <UiLabel for="access-audit">访问审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="file-audit" v-model="securityConfig.fileAudit" />
                <UiLabel for="file-audit">文件审计</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="network-audit" v-model="securityConfig.networkAudit" />
                <UiLabel for="network-audit">网络审计</UiLabel>
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

      <!-- 安全统计 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>安全统计</UiCardTitle>
          <UiCardDescription>安全事件分类统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-1">45</div>
              <p class="text-sm text-muted-foreground">认证事件</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-2">32</div>
              <p class="text-sm text-muted-foreground">访问事件</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-3">56</div>
              <p class="text-sm text-muted-foreground">网络事件</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-4">23</div>
              <p class="text-sm text-muted-foreground">系统事件</p>
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

const securityLogs = ref([
  {
    id: 1,
    title: '异常登录尝试',
    severity: 'high',
    category: '认证',
    source: '认证系统',
    type: '登录失败',
    ip: '203.0.113.45',
    description: '检测到来自未知IP的多次登录失败尝试',
    timestamp: '2024-01-15 14:30:22',
    impact: '认证系统',
  },
  {
    id: 2,
    title: '权限提升尝试',
    severity: 'high',
    category: '权限',
    source: '权限系统',
    type: '权限变更',
    ip: '192.168.1.50',
    description: '用户尝试提升权限到管理员级别',
    timestamp: '2024-01-15 14:28:15',
    impact: '权限系统',
  },
  {
    id: 3,
    title: '文件访问异常',
    severity: 'medium',
    category: '文件',
    source: '文件系统',
    type: '文件访问',
    ip: '192.168.1.51',
    description: '用户访问敏感系统配置文件',
    timestamp: '2024-01-15 14:25:33',
    impact: '文件系统',
  },
  {
    id: 4,
    title: '网络端口扫描',
    severity: 'medium',
    category: '网络',
    source: '网络监控',
    type: '端口扫描',
    ip: '198.51.100.23',
    description: '检测到来自外部IP的端口扫描活动',
    timestamp: '2024-01-15 14:20:45',
    impact: '网络服务',
  },
  {
    id: 5,
    title: '系统配置变更',
    severity: 'low',
    category: '系统',
    source: '系统配置',
    type: '配置修改',
    ip: '192.168.1.101',
    description: '管理员修改系统网络配置',
    timestamp: '2024-01-15 14:15:30',
    impact: '系统配置',
  },
])

const securityConfig = ref({
  retentionPeriod: '90',
  alertLevel: 'medium',
  loginAudit: true,
  accessAudit: true,
  fileAudit: true,
  networkAudit: true,
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
  console.log('导出安全日志')
}
</script>

<style></style>
