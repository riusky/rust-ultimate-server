<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const servers = ref([
  {
    id: 1,
    name: 'Web Server 01',
    status: 'online',
    cpu: 45,
    memory: 68,
    disk: 32,
    network: 12,
    uptime: '15天 8小时',
    ip: '192.168.1.101',
    location: '数据中心 A',
  },
  {
    id: 2,
    name: 'Database Server',
    status: 'online',
    cpu: 78,
    memory: 85,
    disk: 45,
    network: 8,
    uptime: '12天 3小时',
    ip: '192.168.1.102',
    location: '数据中心 A',
  },
  {
    id: 3,
    name: 'App Server 01',
    status: 'warning',
    cpu: 92,
    memory: 75,
    disk: 28,
    network: 15,
    uptime: '8天 12小时',
    ip: '192.168.1.103',
    location: '数据中心 B',
  },
  {
    id: 4,
    name: 'Backup Server',
    status: 'offline',
    cpu: 0,
    memory: 0,
    disk: 0,
    network: 0,
    uptime: '0天 0小时',
    ip: '192.168.1.104',
    location: '数据中心 B',
  },
])

const statusColors: Record<string, string> = {
  online: 'success',
  warning: 'warning',
  offline: 'destructive',
}

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    online: '在线',
    warning: '警告',
    offline: '离线',
  }
  return statusMap[status] || status
}
</script>

<template>
  <Page
    :title="transformI18n('system.server-status.title')"
    :description="transformI18n('system.server-status.description')"
    sticky
  >
    <!-- 服务器状态概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">在线服务器</UiCardTitle>
          <div class="h-4 w-4 rounded-full bg-success"></div>
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">2</div>
          <p class="text-xs text-muted-foreground">总服务器: 4</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">警告服务器</UiCardTitle>
          <div class="h-4 w-4 rounded-full bg-warning"></div>
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">1</div>
          <p class="text-xs text-muted-foreground">需要关注</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">离线服务器</UiCardTitle>
          <div class="h-4 w-4 rounded-full bg-destructive"></div>
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">1</div>
          <p class="text-xs text-muted-foreground">需要检查</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">平均负载</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">53.8%</div>
          <p class="text-xs text-muted-foreground">系统平均使用率</p>
        </UiCardContent>
      </UiCard>
    </div>

    <!-- 服务器状态表格 -->
    <UiCard>
      <UiCardHeader>
        <UiCardTitle>服务器状态</UiCardTitle>
        <UiCardDescription>所有服务器的实时状态和性能指标</UiCardDescription>
      </UiCardHeader>
      <UiCardContent>
        <div class="space-y-4">
          <div
            v-for="server in servers"
            :key="server.id"
            class="flex items-center justify-between p-4 rounded-lg border"
          >
            <div class="flex items-center space-x-4">
              <div class="flex flex-col">
                <div class="flex items-center space-x-2">
                  <h3 class="font-semibold">{{ server.name }}</h3>
                  <UiBadge :variant="statusColors[server.status] as any">
                    {{ getStatusText(server.status) }}
                  </UiBadge>
                </div>
                <p class="text-sm text-muted-foreground">{{ server.ip }} - {{ server.location }}</p>
                <p class="text-xs text-muted-foreground">运行时间: {{ server.uptime }}</p>
              </div>
            </div>

            <div class="flex items-center space-x-6">
              <!-- CPU 使用率 -->
              <div class="text-center">
                <div class="text-sm font-medium">CPU</div>
                <div
                  class="text-lg font-bold"
                  :class="{
                    'text-success': server.cpu < 70,
                    'text-warning': server.cpu >= 70 && server.cpu < 90,
                    'text-destructive': server.cpu >= 90,
                  }"
                >
                  {{ server.cpu }}%
                </div>
              </div>

              <!-- 内存使用率 -->
              <div class="text-center">
                <div class="text-sm font-medium">内存</div>
                <div
                  class="text-lg font-bold"
                  :class="{
                    'text-success': server.memory < 70,
                    'text-warning': server.memory >= 70 && server.memory < 90,
                    'text-destructive': server.memory >= 90,
                  }"
                >
                  {{ server.memory }}%
                </div>
              </div>

              <!-- 磁盘使用率 -->
              <div class="text-center">
                <div class="text-sm font-medium">磁盘</div>
                <div
                  class="text-lg font-bold"
                  :class="{
                    'text-success': server.disk < 70,
                    'text-warning': server.disk >= 70 && server.disk < 90,
                    'text-destructive': server.disk >= 90,
                  }"
                >
                  {{ server.disk }}%
                </div>
              </div>

              <!-- 网络流量 -->
              <div class="text-center">
                <div class="text-sm font-medium">网络</div>
                <div class="text-lg font-bold text-chart-1">{{ server.network }} MB/s</div>
              </div>

              <!-- 操作按钮 -->
              <div class="flex space-x-2">
                <UiButton variant="outline" size="sm">
                  <Monitor class="h-4 w-4 mr-1" />
                  监控
                </UiButton>
                <UiButton variant="outline" size="sm">
                  <Settings class="h-4 w-4 mr-1" />
                  配置
                </UiButton>
              </div>
            </div>
          </div>
        </div>
      </UiCardContent>
    </UiCard>
  </Page>
</template>

<style></style>
