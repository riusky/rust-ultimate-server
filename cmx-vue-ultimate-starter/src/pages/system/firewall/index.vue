<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const firewallStatus = ref({
  enabled: true,
  defaultPolicy: 'deny',
  activeRules: 24,
  blockedConnections: 156,
  allowedConnections: 2450,
})

const firewallRules = ref([
  {
    id: 1,
    name: 'SSH Access',
    protocol: 'TCP',
    port: '22',
    source: '192.168.1.0/24',
    destination: 'any',
    action: 'allow',
    enabled: true,
    description: '内部网络 SSH 访问',
  },
  {
    id: 2,
    name: 'HTTP Access',
    protocol: 'TCP',
    port: '80',
    source: '0.0.0.0/0',
    destination: 'any',
    action: 'allow',
    enabled: true,
    description: 'Web 服务器 HTTP 访问',
  },
  {
    id: 3,
    name: 'HTTPS Access',
    protocol: 'TCP',
    port: '443',
    source: '0.0.0.0/0',
    destination: 'any',
    action: 'allow',
    enabled: true,
    description: 'Web 服务器 HTTPS 访问',
  },
  {
    id: 4,
    name: 'MySQL Block',
    protocol: 'TCP',
    port: '3306',
    source: '0.0.0.0/0',
    destination: 'any',
    action: 'deny',
    enabled: true,
    description: '阻止外部 MySQL 访问',
  },
  {
    id: 5,
    name: 'ICMP Allow',
    protocol: 'ICMP',
    port: 'any',
    source: '192.168.1.0/24',
    destination: 'any',
    action: 'allow',
    enabled: true,
    description: '内部网络 Ping 测试',
  },
])

const securityGroups = ref([
  {
    id: 1,
    name: 'Web Servers',
    description: 'Web 服务器安全组',
    rules: 8,
    instances: 3,
  },
  {
    id: 2,
    name: 'Database Servers',
    description: '数据库服务器安全组',
    rules: 12,
    instances: 2,
  },
  {
    id: 3,
    name: 'Application Servers',
    description: '应用服务器安全组',
    rules: 6,
    instances: 4,
  },
])

const firewallLogs = ref([
  {
    id: 1,
    timestamp: '2024-01-15 14:30:22',
    source: '203.0.113.45',
    destination: '192.168.1.101',
    protocol: 'TCP',
    port: '3306',
    action: 'blocked',
    reason: '规则拒绝',
  },
  {
    id: 2,
    timestamp: '2024-01-15 14:28:15',
    source: '192.168.1.50',
    destination: '192.168.1.101',
    protocol: 'TCP',
    port: '22',
    action: 'allowed',
    reason: '规则允许',
  },
  {
    id: 3,
    timestamp: '2024-01-15 14:25:33',
    source: '198.51.100.23',
    destination: '192.168.1.101',
    protocol: 'TCP',
    port: '80',
    action: 'allowed',
    reason: '规则允许',
  },
])

const toggleFirewall = () => {
  firewallStatus.value.enabled = !firewallStatus.value.enabled
  console.log('防火墙状态:', firewallStatus.value.enabled ? '启用' : '禁用')
}

const addRule = () => {
  console.log('添加防火墙规则')
}

const editRule = (ruleId: number) => {
  console.log('编辑防火墙规则:', ruleId)
}

const deleteRule = (ruleId: number) => {
  console.log('删除防火墙规则:', ruleId)
}

const getActionColor = (action: string) => {
  return action === 'allow' ? 'success' : 'destructive'
}

const getActionText = (action: string) => {
  return action === 'allow' ? '允许' : '拒绝'
}

const getLogActionColor = (action: string) => {
  return action === 'allowed' ? 'success' : 'destructive'
}

const getLogActionText = (action: string) => {
  return action === 'allowed' ? '允许' : '阻止'
}
</script>

<template>
  <Page
    :title="transformI18n('system.firewall.title')"
    :description="transformI18n('system.firewall.description')"
    sticky
  >
    <!-- 防火墙状态概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">防火墙状态</UiCardTitle>
          <Shield class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div
            class="text-2xl font-bold"
            :class="firewallStatus.enabled ? 'text-success' : 'text-destructive'"
          >
            {{ firewallStatus.enabled ? '启用' : '禁用' }}
          </div>
          <p class="text-xs text-muted-foreground">
            默认策略: {{ firewallStatus.defaultPolicy === 'deny' ? '拒绝' : '允许' }}
          </p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃规则</UiCardTitle>
          <List class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ firewallStatus.activeRules }}</div>
          <p class="text-xs text-muted-foreground">总规则数</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">允许连接</UiCardTitle>
          <CheckCircle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">{{ firewallStatus.allowedConnections }}</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">阻止连接</UiCardTitle>
          <XCircle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">
            {{ firewallStatus.blockedConnections }}
          </div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 防火墙规则 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>防火墙规则</UiCardTitle>
          <UiCardDescription>网络访问控制规则配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="rule in firewallRules"
              :key="rule.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ rule.name }}</h3>
                    <UiBadge :variant="getActionColor(rule.action) as any">
                      {{ getActionText(rule.action) }}
                    </UiBadge>
                    <UiBadge v-if="!rule.enabled" variant="outline"> 禁用 </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ rule.description }}</p>
                  <p class="text-xs text-muted-foreground">
                    {{ rule.protocol }}:{{ rule.port }} • 来源: {{ rule.source }} • 目标:
                    {{ rule.destination }}
                  </p>
                </div>
              </div>
              <div class="flex items-center space-x-2">
                <UiSwitch v-model="rule.enabled" />
                <UiButton variant="outline" size="sm" @click="editRule(rule.id)">
                  <Edit class="h-4 w-4" />
                </UiButton>
                <UiButton variant="outline" size="sm" @click="deleteRule(rule.id)">
                  <Trash2 class="h-4 w-4" />
                </UiButton>
              </div>
            </div>
          </div>
          <div class="mt-4">
            <UiButton @click="addRule">
              <Plus class="h-4 w-4 mr-2" />
              添加规则
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 安全组 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>安全组</UiCardTitle>
          <UiCardDescription>服务器安全组配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div v-for="group in securityGroups" :key="group.id" class="p-3 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <h3 class="font-semibold">{{ group.name }}</h3>
                <UiBadge variant="outline">{{ group.rules }} 规则</UiBadge>
              </div>
              <p class="text-sm text-muted-foreground mb-2">{{ group.description }}</p>
              <div class="flex justify-between text-xs text-muted-foreground">
                <span>{{ group.instances }} 个实例</span>
                <UiButton variant="ghost" size="sm">
                  <Settings class="h-3 w-3 mr-1" />
                  配置
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 防火墙日志 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>防火墙日志</UiCardTitle>
          <UiCardDescription>最近的防火墙活动记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="log in firewallLogs"
              :key="log.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <UiBadge :variant="getLogActionColor(log.action) as any">
                      {{ getLogActionText(log.action) }}
                    </UiBadge>
                    <span class="text-sm font-medium"
                      >{{ log.source }} → {{ log.destination }}</span
                    >
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ log.protocol }}:{{ log.port }} • {{ log.timestamp }}
                  </p>
                  <p class="text-xs text-muted-foreground">原因: {{ log.reason }}</p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="toggleFirewall">
          <Power class="h-4 w-4 mr-2" />
          {{ firewallStatus.enabled ? '禁用防火墙' : '启用防火墙' }}
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
