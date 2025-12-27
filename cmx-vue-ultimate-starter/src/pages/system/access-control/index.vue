<template>
  <Page
    :title="transformI18n('system.access-control.title')"
    :description="transformI18n('system.access-control.description')"
    sticky
  >
    <!-- 访问控制概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总用户数</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">156</div>
          <p class="text-xs text-muted-foreground">注册用户</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃用户</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">42</div>
          <p class="text-xs text-muted-foreground">今日在线</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">角色数量</UiCardTitle>
          <Shield class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">8</div>
          <p class="text-xs text-muted-foreground">权限角色</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">访问规则</UiCardTitle>
          <List class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">24</div>
          <p class="text-xs text-muted-foreground">控制规则</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 访问规则 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>访问规则</UiCardTitle>
          <UiCardDescription>系统访问控制规则配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="rule in accessRules"
              :key="rule.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ rule.name }}</h3>
                    <UiBadge :variant="getRuleTypeColor(rule.type) as any">
                      {{ getRuleTypeText(rule.type) }}
                    </UiBadge>
                    <UiBadge v-if="!rule.enabled" variant="outline"> 禁用 </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ rule.description }}</p>
                  <p class="text-xs text-muted-foreground">
                    条件: {{ rule.condition }} • 动作: {{ getActionText(rule.action) }}
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

      <!-- 访问策略 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>访问策略</UiCardTitle>
          <UiCardDescription>全局访问控制策略</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="default-policy">默认策略</UiLabel>
              <UiSelect v-model="accessPolicy.defaultPolicy">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="allow">允许</UiSelectItem>
                  <UiSelectItem value="deny">拒绝</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div>
              <UiLabel for="session-timeout">会话超时 (分钟)</UiLabel>
              <UiInput id="session-timeout" v-model="accessPolicy.sessionTimeout" type="number" />
            </div>
            <div>
              <UiLabel for="max-login-attempts">最大登录尝试次数</UiLabel>
              <UiInput
                id="max-login-attempts"
                v-model="accessPolicy.maxLoginAttempts"
                type="number"
              />
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="mfa" v-model="accessPolicy.mfaEnabled" />
                <UiLabel for="mfa">启用多因素认证</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="ip-restriction" v-model="accessPolicy.ipRestriction" />
                <UiLabel for="ip-restriction">IP 地址限制</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="time-restriction" v-model="accessPolicy.timeRestriction" />
                <UiLabel for="time-restriction">时间限制</UiLabel>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 访问日志 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>访问日志</UiCardTitle>
          <UiCardDescription>最近的访问控制事件</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="log in accessLogs"
              :key="log.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="text-sm font-medium">{{ log.user }}</span>
                    <span class="text-sm text-muted-foreground">→</span>
                    <span class="text-sm font-medium">{{ log.resource }}</span>
                    <UiBadge :variant="getAccessResultColor(log.result) as any">
                      {{ getAccessResultText(log.result) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ log.action }} • IP: {{ log.ip }} • {{ log.timestamp }}
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
        <UiButton variant="outline" @click="testPolicy">
          <Play class="h-4 w-4 mr-2" />
          测试策略
        </UiButton>
        <UiButton @click="saveConfig">
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

const accessRules = ref([
  {
    id: 1,
    name: '管理员访问限制',
    type: 'role',
    description: '限制管理员只能在特定IP段访问',
    condition: 'role=admin && ip not in 192.168.1.0/24',
    action: 'deny',
    enabled: true,
  },
  {
    id: 2,
    name: '工作时间访问',
    type: 'time',
    description: '普通用户只能在工作时间访问',
    condition: 'role=user && time not in 09:00-18:00',
    action: 'deny',
    enabled: true,
  },
  {
    id: 3,
    name: '敏感数据访问',
    type: 'resource',
    description: '只有特定角色可以访问敏感数据',
    condition: 'resource=sensitive && role not in [admin, auditor]',
    action: 'deny',
    enabled: true,
  },
  {
    id: 4,
    name: '多因素认证',
    type: 'auth',
    description: '管理员登录需要多因素认证',
    condition: 'role=admin && action=login',
    action: 'require_mfa',
    enabled: false,
  },
])

const accessPolicy = ref({
  defaultPolicy: 'deny',
  sessionTimeout: 30,
  maxLoginAttempts: 5,
  mfaEnabled: true,
  ipRestriction: true,
  timeRestriction: false,
})

const accessLogs = ref([
  {
    id: 1,
    user: 'user01',
    resource: '/api/sensitive-data',
    action: '读取',
    result: 'denied',
    ip: '192.168.1.50',
    timestamp: '2024-01-15 14:30:22',
    reason: '权限不足',
  },
  {
    id: 2,
    user: 'admin',
    resource: '系统配置',
    action: '修改',
    result: 'allowed',
    ip: '192.168.1.101',
    timestamp: '2024-01-15 14:28:15',
    reason: '规则允许',
  },
  {
    id: 3,
    user: 'user02',
    resource: '报表数据',
    action: '导出',
    result: 'denied',
    ip: '192.168.1.52',
    timestamp: '2024-01-15 14:25:33',
    reason: '非工作时间',
  },
  {
    id: 4,
    user: 'auditor',
    resource: '审计日志',
    action: '查看',
    result: 'allowed',
    ip: '192.168.1.53',
    timestamp: '2024-01-15 14:20:45',
    reason: '规则允许',
  },
])

const getRuleTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    role: 'chart-1',
    time: 'chart-2',
    resource: 'chart-3',
    auth: 'chart-4',
  }
  return colors[type] || 'default'
}

const getRuleTypeText = (type: string) => {
  const texts: Record<string, string> = {
    role: '角色',
    time: '时间',
    resource: '资源',
    auth: '认证',
  }
  return texts[type] || type
}

const getActionText = (action: string) => {
  const texts: Record<string, string> = {
    allow: '允许',
    deny: '拒绝',
    require_mfa: '需要MFA',
  }
  return texts[action] || action
}

const getAccessResultColor = (result: string) => {
  const colors: Record<string, string> = {
    allowed: 'success',
    denied: 'destructive',
  }
  return colors[result] || 'default'
}

const getAccessResultText = (result: string) => {
  const texts: Record<string, string> = {
    allowed: '允许',
    denied: '拒绝',
  }
  return texts[result] || result
}

const addRule = () => {
  console.log('添加访问规则')
}

const editRule = (ruleId: number) => {
  console.log('编辑访问规则:', ruleId)
}

const deleteRule = (ruleId: number) => {
  console.log('删除访问规则:', ruleId)
}

const testPolicy = () => {
  console.log('测试访问策略')
}

const saveConfig = () => {
  console.log('保存访问控制配置')
}
</script>

<style></style>
