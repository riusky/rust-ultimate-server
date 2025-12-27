<template>
  <Page
    :title="transformI18n('system.security-audit.title')"
    :description="transformI18n('system.security-audit.description')"
    sticky
  >
    <!-- 审计统计概览 -->
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
          <UiCardTitle class="text-sm font-medium">合规检查</UiCardTitle>
          <CheckCircle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">89%</div>
          <p class="text-xs text-muted-foreground">合规率</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">审计覆盖率</UiCardTitle>
          <Shield class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-chart-1">95%</div>
          <p class="text-xs text-muted-foreground">系统覆盖</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 审计事件 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>审计事件</UiCardTitle>
          <UiCardDescription>最近的系统安全审计记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="event in auditEvents"
              :key="event.id"
              class="flex items-center justify-between p-4 rounded-lg border"
              :class="{
                'border-destructive/20 bg-destructive/10': event.severity === 'high',
                'border-warning/20 bg-warning/10': event.severity === 'medium',
                'border-chart-3/20 bg-chart-3/10': event.severity === 'low',
              }"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ event.title }}</h3>
                    <UiBadge :variant="getSeverityColor(event.severity) as any">
                      {{ getSeverityText(event.severity) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground mb-2">
                    用户: {{ event.user }} • 操作: {{ event.action }} • IP: {{ event.ip }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{ event.description }}
                  </p>
                  <div class="flex items-center space-x-4 mt-2 text-xs text-muted-foreground">
                    <span>时间: {{ event.timestamp }}</span>
                    <span>资源: {{ event.resource }}</span>
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
          <UiCardDescription>安全审计策略和设置</UiCardDescription>
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
              <UiButton class="w-full" @click="generateReport">
                <FileText class="h-4 w-4 mr-2" />
                生成报告
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 合规检查 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>合规检查</UiCardTitle>
          <UiCardDescription>安全合规性检查结果</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="check in complianceChecks"
              :key="check.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ check.name }}</h3>
                    <UiBadge :variant="getComplianceColor(check.status) as any">
                      {{ getComplianceText(check.status) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ check.description }}
                  </p>
                  <p class="text-xs text-muted-foreground">
                    标准: {{ check.standard }} • 上次检查: {{ check.lastCheck }}
                  </p>
                </div>
              </div>
              <div class="text-right">
                <div class="text-lg font-bold">{{ check.score }}%</div>
                <p class="text-sm text-muted-foreground">合规分数</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline">
          <Download class="h-4 w-4 mr-2" />
          导出审计
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

const auditEvents = ref([
  {
    id: 1,
    title: '异常登录尝试',
    severity: 'high',
    user: 'admin',
    action: '登录失败',
    ip: '192.168.1.100',
    description: '检测到多次登录失败尝试',
    timestamp: '2024-01-15 14:30:22',
    resource: '认证系统',
  },
  {
    id: 2,
    title: '权限变更',
    severity: 'medium',
    user: 'user01',
    action: '权限修改',
    ip: '192.168.1.50',
    description: '用户权限被管理员修改',
    timestamp: '2024-01-15 14:28:15',
    resource: '权限系统',
  },
  {
    id: 3,
    title: '文件访问',
    severity: 'low',
    user: 'user02',
    action: '文件读取',
    ip: '192.168.1.51',
    description: '用户访问敏感配置文件',
    timestamp: '2024-01-15 14:25:33',
    resource: '文件系统',
  },
  {
    id: 4,
    title: '系统配置变更',
    severity: 'medium',
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

const complianceChecks = ref([
  {
    id: 1,
    name: '密码策略',
    status: 'compliant',
    description: '密码复杂度要求和过期策略',
    standard: 'ISO 27001',
    lastCheck: '2024-01-15',
    score: 95,
  },
  {
    id: 2,
    name: '访问控制',
    status: 'compliant',
    description: '用户权限和访问控制机制',
    standard: 'NIST SP 800-53',
    lastCheck: '2024-01-15',
    score: 92,
  },
  {
    id: 3,
    name: '日志审计',
    status: 'non-compliant',
    description: '系统日志记录和审计跟踪',
    standard: 'PCI DSS',
    lastCheck: '2024-01-14',
    score: 78,
  },
  {
    id: 4,
    name: '数据加密',
    status: 'compliant',
    description: '数据传输和存储加密',
    standard: 'GDPR',
    lastCheck: '2024-01-14',
    score: 88,
  },
])

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

const getComplianceColor = (status: string) => {
  const colors: Record<string, string> = {
    compliant: 'success',
    'non-compliant': 'destructive',
    'partial-compliant': 'warning',
  }
  return colors[status] || 'default'
}

const getComplianceText = (status: string) => {
  const texts: Record<string, string> = {
    compliant: '合规',
    'non-compliant': '不合规',
    'partial-compliant': '部分合规',
  }
  return texts[status] || status
}

const generateReport = () => {
  console.log('生成审计报告')
}
</script>

<style></style>
