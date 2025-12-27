<template>
  <Page
    :title="transformI18n('system.session.title')"
    :description="transformI18n('system.session.description')"
    sticky
  >
    <!-- 会话统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃会话</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ sessionStats.activeSessions }}</div>
          <p class="text-xs text-muted-foreground">当前在线</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">今日会话</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ sessionStats.todaySessions }}</div>
          <p class="text-xs text-muted-foreground">今日总数</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">平均时长</UiCardTitle>
          <Clock class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ sessionStats.avgDuration }}</div>
          <p class="text-xs text-muted-foreground">会话平均时长</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">异常会话</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-destructive">{{ sessionStats.abnormalSessions }}</div>
          <p class="text-xs text-muted-foreground">需要关注</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 活跃会话 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>活跃会话</UiCardTitle>
          <UiCardDescription>当前活跃的用户会话</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="session in activeSessions"
              :key="session.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ session.user }}</h3>
                    <UiBadge :variant="getSessionStatusColor(session.status) as any">
                      {{ getSessionStatusText(session.status) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ session.role }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground mb-2">
                    IP: {{ session.ip }} • 登录时间: {{ session.loginTime }}
                  </p>
                  <div class="flex items-center space-x-4 text-sm">
                    <span>最后活动: {{ session.lastActivity }}</span>
                    <span>持续时间: {{ session.duration }}</span>
                  </div>
                </div>
              </div>
              <div class="flex flex-col space-y-2">
                <UiButton variant="outline" size="sm" @click="viewSessionDetails(session.id)">
                  <Eye class="h-4 w-4 mr-1" />
                  详情
                </UiButton>
                <UiButton variant="outline" size="sm" @click="terminateSession(session.id)">
                  <LogOut class="h-4 w-4 mr-1" />
                  终止
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 会话配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>会话配置</UiCardTitle>
          <UiCardDescription>会话管理策略和设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="session-timeout">会话超时 (分钟)</UiLabel>
              <UiInput id="session-timeout" v-model="sessionConfig.timeout" type="number" />
            </div>
            <div>
              <UiLabel for="max-sessions">最大会话数</UiLabel>
              <UiInput id="max-sessions" v-model="sessionConfig.maxSessions" type="number" />
            </div>
            <div>
              <UiLabel for="idle-timeout">空闲超时 (分钟)</UiLabel>
              <UiInput id="idle-timeout" v-model="sessionConfig.idleTimeout" type="number" />
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="concurrent-sessions" v-model="sessionConfig.allowConcurrent" />
                <UiLabel for="concurrent-sessions">允许并发会话</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="session-lock" v-model="sessionConfig.sessionLock" />
                <UiLabel for="session-lock">会话锁定</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="ip-tracking" v-model="sessionConfig.ipTracking" />
                <UiLabel for="ip-tracking">IP 地址跟踪</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="device-tracking" v-model="sessionConfig.deviceTracking" />
                <UiLabel for="device-tracking">设备跟踪</UiLabel>
              </div>
            </div>
            <div class="pt-4">
              <UiButton class="w-full" @click="clearExpiredSessions">
                <Trash2 class="h-4 w-4 mr-2" />
                清理过期会话
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 会话历史 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>会话历史</UiCardTitle>
          <UiCardDescription>最近的会话活动记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="history in sessionHistory"
              :key="history.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="text-sm font-medium">{{ history.user }}</span>
                    <span class="text-sm text-muted-foreground">→</span>
                    <span class="text-sm font-medium">{{ history.action }}</span>
                    <UiBadge :variant="getHistoryTypeColor(history.type) as any">
                      {{ getHistoryTypeText(history.type) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    IP: {{ history.ip }} • {{ history.timestamp }}
                  </p>
                  <p class="text-xs text-muted-foreground">
                    持续时间: {{ history.duration }} • 原因: {{ history.reason }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="refreshSessions">
          <RefreshCw class="h-4 w-4 mr-2" />
          刷新会话
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

const sessionStats = ref({
  activeSessions: 42,
  todaySessions: 156,
  avgDuration: '25分钟',
  abnormalSessions: 3,
})

const activeSessions = ref([
  {
    id: 1,
    user: 'admin',
    role: '管理员',
    status: 'active',
    ip: '192.168.1.101',
    loginTime: '2024-01-15 14:30:22',
    lastActivity: '2分钟前',
    duration: '25分钟',
  },
  {
    id: 2,
    user: 'user01',
    role: '用户',
    status: 'active',
    ip: '192.168.1.50',
    loginTime: '2024-01-15 14:28:15',
    lastActivity: '5分钟前',
    duration: '28分钟',
  },
  {
    id: 3,
    user: 'user02',
    role: '审计员',
    status: 'idle',
    ip: '192.168.1.51',
    loginTime: '2024-01-15 14:25:33',
    lastActivity: '15分钟前',
    duration: '31分钟',
  },
  {
    id: 4,
    user: 'user03',
    role: '用户',
    status: 'suspicious',
    ip: '203.0.113.45',
    loginTime: '2024-01-15 14:20:45',
    lastActivity: '1分钟前',
    duration: '36分钟',
  },
])

const sessionConfig = ref({
  timeout: 30,
  maxSessions: 10,
  idleTimeout: 15,
  allowConcurrent: true,
  sessionLock: true,
  ipTracking: true,
  deviceTracking: false,
})

const sessionHistory = ref([
  {
    id: 1,
    user: 'user04',
    action: '登录',
    type: 'login',
    ip: '192.168.1.52',
    timestamp: '2024-01-15 14:30:22',
    duration: '45分钟',
    reason: '正常退出',
  },
  {
    id: 2,
    user: 'user05',
    action: '会话超时',
    type: 'timeout',
    ip: '192.168.1.53',
    timestamp: '2024-01-15 14:28:15',
    duration: '30分钟',
    reason: '空闲超时',
  },
  {
    id: 3,
    user: 'admin',
    action: '强制终止',
    type: 'terminated',
    ip: '192.168.1.101',
    timestamp: '2024-01-15 14:25:33',
    duration: '15分钟',
    reason: '安全策略',
  },
  {
    id: 4,
    user: 'user06',
    action: '异常退出',
    type: 'abnormal',
    ip: '203.0.113.46',
    timestamp: '2024-01-15 14:20:45',
    duration: '8分钟',
    reason: 'IP地址异常',
  },
])

const getSessionStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    active: 'success',
    idle: 'warning',
    suspicious: 'destructive',
  }
  return colors[status] || 'default'
}

const getSessionStatusText = (status: string) => {
  const texts: Record<string, string> = {
    active: '活跃',
    idle: '空闲',
    suspicious: '可疑',
  }
  return texts[status] || status
}

const getHistoryTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    login: 'success',
    logout: 'secondary',
    timeout: 'warning',
    terminated: 'destructive',
    abnormal: 'destructive',
  }
  return colors[type] || 'default'
}

const getHistoryTypeText = (type: string) => {
  const texts: Record<string, string> = {
    login: '登录',
    logout: '退出',
    timeout: '超时',
    terminated: '终止',
    abnormal: '异常',
  }
  return texts[type] || type
}

const viewSessionDetails = (sessionId: number) => {
  console.log('查看会话详情:', sessionId)
}

const terminateSession = (sessionId: number) => {
  console.log('终止会话:', sessionId)
}

const clearExpiredSessions = () => {
  console.log('清理过期会话')
}

const refreshSessions = () => {
  console.log('刷新会话列表')
}

const saveConfig = () => {
  console.log('保存会话配置')
}
</script>

<style></style>
