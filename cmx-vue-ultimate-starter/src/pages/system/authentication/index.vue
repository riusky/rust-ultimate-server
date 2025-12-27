<template>
  <Page
    :title="transformI18n('system.authentication.title')"
    :description="transformI18n('system.authentication.description')"
    sticky
  >
    <!-- 认证统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总认证事件</UiCardTitle>
          <Key class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">1,245</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">登录失败</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">23</div>
          <p class="text-xs text-muted-foreground">异常尝试</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">MFA 启用率</UiCardTitle>
          <Shield class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">78%</div>
          <p class="text-xs text-muted-foreground">多因素认证</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">会话活跃</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">42</div>
          <p class="text-xs text-muted-foreground">活跃会话</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 认证配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>认证配置</UiCardTitle>
          <UiCardDescription>身份认证策略和设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <UiLabel for="password-min-length">密码最小长度</UiLabel>
                <UiInput
                  id="password-min-length"
                  v-model="authConfig.passwordMinLength"
                  type="number"
                />
              </div>
              <div>
                <UiLabel for="password-expiry">密码过期天数</UiLabel>
                <UiInput
                  id="password-expiry"
                  v-model="authConfig.passwordExpiryDays"
                  type="number"
                />
              </div>
              <div>
                <UiLabel for="session-timeout">会话超时 (分钟)</UiLabel>
                <UiInput id="session-timeout" v-model="authConfig.sessionTimeout" type="number" />
              </div>
              <div>
                <UiLabel for="max-login-attempts">最大登录尝试次数</UiLabel>
                <UiInput
                  id="max-login-attempts"
                  v-model="authConfig.maxLoginAttempts"
                  type="number"
                />
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="password-complexity" v-model="authConfig.passwordComplexity" />
                <UiLabel for="password-complexity">密码复杂度要求</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="password-history" v-model="authConfig.passwordHistory" />
                <UiLabel for="password-history">密码历史检查</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="account-lockout" v-model="authConfig.accountLockout" />
                <UiLabel for="account-lockout">账户锁定机制</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="session-limit" v-model="authConfig.sessionLimit" />
                <UiLabel for="session-limit">会话数量限制</UiLabel>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 多因素认证 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>多因素认证</UiCardTitle>
          <UiCardDescription>MFA 配置和管理</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="mfa-method">MFA 方法</UiLabel>
              <UiSelect v-model="mfaConfig.method">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="totp">TOTP (时间令牌)</UiSelectItem>
                  <UiSelectItem value="sms">SMS 验证码</UiSelectItem>
                  <UiSelectItem value="email">邮箱验证码</UiSelectItem>
                  <UiSelectItem value="push">推送通知</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div>
              <UiLabel for="mfa-required">MFA 要求</UiLabel>
              <UiSelect v-model="mfaConfig.required">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="all">所有用户</UiSelectItem>
                  <UiSelectItem value="admin">仅管理员</UiSelectItem>
                  <UiSelectItem value="sensitive">敏感操作</UiSelectItem>
                  <UiSelectItem value="optional">可选</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="mfa-backup" v-model="mfaConfig.backupCodes" />
                <UiLabel for="mfa-backup">启用备份代码</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="mfa-remember" v-model="mfaConfig.rememberDevice" />
                <UiLabel for="mfa-remember">记住设备</UiLabel>
              </div>
            </div>
            <div class="pt-4">
              <UiButton class="w-full" @click="testMFA">
                <Shield class="h-4 w-4 mr-2" />
                测试 MFA
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 认证日志 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>认证日志</UiCardTitle>
          <UiCardDescription>最近的认证事件记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="log in authLogs"
              :key="log.id"
              class="flex items-center justify-between p-3 rounded-lg border"
              :class="{
                'border-destructive/20 bg-destructive/10': log.result === 'failed',
                'border-success/20 bg-success/10': log.result === 'success',
              }"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="text-sm font-medium">{{ log.user }}</span>
                    <span class="text-sm text-muted-foreground">→</span>
                    <span class="text-sm font-medium">{{ log.action }}</span>
                    <UiBadge :variant="getAuthResultColor(log.result) as any">
                      {{ getAuthResultText(log.result) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    IP: {{ log.ip }} • {{ log.timestamp }}
                  </p>
                  <p class="text-xs text-muted-foreground">
                    方法: {{ log.method }} • 原因: {{ log.reason }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="resetConfig">
          <RefreshCw class="h-4 w-4 mr-2" />
          重置配置
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

const authConfig = ref({
  passwordMinLength: 8,
  passwordExpiryDays: 90,
  sessionTimeout: 30,
  maxLoginAttempts: 5,
  passwordComplexity: true,
  passwordHistory: true,
  accountLockout: true,
  sessionLimit: true,
})

const mfaConfig = ref({
  method: 'totp',
  required: 'admin',
  backupCodes: true,
  rememberDevice: true,
})

const authLogs = ref([
  {
    id: 1,
    user: 'admin',
    action: '登录',
    result: 'success',
    ip: '192.168.1.101',
    timestamp: '2024-01-15 14:30:22',
    method: '密码+MFA',
    reason: '正常登录',
  },
  {
    id: 2,
    user: 'user01',
    action: '登录',
    result: 'failed',
    ip: '192.168.1.50',
    timestamp: '2024-01-15 14:28:15',
    method: '密码',
    reason: '密码错误',
  },
  {
    id: 3,
    user: 'user02',
    action: '密码修改',
    result: 'success',
    ip: '192.168.1.51',
    timestamp: '2024-01-15 14:25:33',
    method: '密码',
    reason: '定期更新',
  },
  {
    id: 4,
    user: 'user03',
    action: 'MFA 设置',
    result: 'success',
    ip: '192.168.1.52',
    timestamp: '2024-01-15 14:20:45',
    method: 'TOTP',
    reason: '启用多因素认证',
  },
])

const getAuthResultColor = (result: string) => {
  const colors: Record<string, string> = {
    success: 'success',
    failed: 'destructive',
  }
  return colors[result] || 'default'
}

const getAuthResultText = (result: string) => {
  const texts: Record<string, string> = {
    success: '成功',
    failed: '失败',
  }
  return texts[result] || result
}

const testMFA = () => {
  console.log('测试多因素认证')
}

const resetConfig = () => {
  console.log('重置认证配置')
}

const saveConfig = () => {
  console.log('保存认证配置')
}
</script>

<style></style>
