<template>
  <Page title="负载均衡" description="管理系统负载均衡配置和状态监控" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        添加负载均衡器
      </UiButton>
      <UiButton variant="outline">
        <RefreshCw class="w-4 h-4 mr-2" />
        刷新
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 负载均衡概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">负载均衡器</UiCardTitle>
            <Server class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">3</div>
            <p class="text-xs text-muted-foreground">活跃负载均衡器</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">后端服务器</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">12</div>
            <p class="text-xs text-muted-foreground">后端服务器总数</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">请求数/秒</UiCardTitle>
            <Activity class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">1.2k</div>
            <p class="text-xs text-muted-foreground">当前请求速率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">错误率</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">0.2%</div>
            <p class="text-xs text-muted-foreground">请求错误率</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 负载均衡器列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>负载均衡器列表</UiCardTitle>
          <UiCardDescription>系统负载均衡器配置和状态</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">负载均衡器</th>
                  <th scope="col" class="px-6 py-3">协议</th>
                  <th scope="col" class="px-6 py-3">后端服务器</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">请求数/秒</th>
                  <th scope="col" class="px-6 py-3">错误率</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="lb in loadBalancers" :key="lb.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <Server class="w-4 h-4 text-primary" />
                      </div>
                      <div>
                        <div class="font-medium">{{ lb.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ lb.address }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge variant="outline">{{ lb.protocol }}</UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <span class="font-medium">{{ lb.backendCount }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(lb.status)">
                      {{ lb.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <span class="font-medium text-chart-3">{{ lb.requestsPerSecond }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <span
                      class="font-medium"
                      :class="lb.errorRate > 1 ? 'text-destructive' : 'text-chart-2'"
                    >
                      {{ lb.errorRate }}%
                    </span>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Settings class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Activity class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 后端服务器状态 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>后端服务器状态</UiCardTitle>
          <UiCardDescription>负载均衡器后端服务器健康状态</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="server in backendServers"
              :key="server.id"
              class="border rounded-lg p-4 space-y-3"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <Server class="w-5 h-5 text-chart-3" />
                  <h3 class="font-medium">{{ server.name }}</h3>
                </div>
                <UiBadge :variant="getHealthVariant(server.health)">
                  {{ server.health }}
                </UiBadge>
              </div>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-muted-foreground">地址:</span>
                  <span>{{ server.address }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">权重:</span>
                  <span>{{ server.weight }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">连接数:</span>
                  <span>{{ server.connections }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">响应时间:</span>
                  <span>{{ server.responseTime }}ms</span>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 负载均衡算法 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>负载均衡算法</UiCardTitle>
          <UiCardDescription>配置负载均衡分发策略</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="border rounded-lg p-4 space-y-2 text-center">
              <Scale class="w-8 h-8 text-chart-3 mx-auto" />
              <h3 class="font-medium">轮询</h3>
              <p class="text-sm text-muted-foreground">按顺序分发请求</p>
              <UiBadge variant="default">已启用</UiBadge>
            </div>
            <div class="border rounded-lg p-4 space-y-2 text-center">
              <Cpu class="w-8 h-8 text-chart-4 mx-auto" />
              <h3 class="font-medium">最少连接</h3>
              <p class="text-sm text-muted-foreground">选择连接数最少的服务器</p>
              <UiBadge variant="outline">未启用</UiBadge>
            </div>
            <div class="border rounded-lg p-4 space-y-2 text-center">
              <Activity class="w-8 h-8 text-chart-5 mx-auto" />
              <h3 class="font-medium">源IP哈希</h3>
              <p class="text-sm text-muted-foreground">基于客户端IP分发</p>
              <UiBadge variant="outline">未启用</UiBadge>
            </div>
            <div class="border rounded-lg p-4 space-y-2 text-center">
              <BarChart3 class="w-8 h-8 text-chart-2 mx-auto" />
              <h3 class="font-medium">加权轮询</h3>
              <p class="text-sm text-muted-foreground">基于权重分发请求</p>
              <UiBadge variant="outline">未启用</UiBadge>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import {
  Plus,
  RefreshCw,
  Server,
  Users,
  Activity,
  AlertTriangle,
  Settings,
  Scale,
  Cpu,
  BarChart3,
} from 'lucide-vue-next'

const loadBalancers = ref([
  {
    id: 1,
    name: 'Web-LB-01',
    address: '192.168.1.10:80',
    protocol: 'HTTP',
    backendCount: 4,
    status: '运行中',
    requestsPerSecond: '850',
    errorRate: 0.1,
  },
  {
    id: 2,
    name: 'API-LB-01',
    address: '192.168.1.11:443',
    protocol: 'HTTPS',
    backendCount: 3,
    status: '运行中',
    requestsPerSecond: '1.2k',
    errorRate: 0.3,
  },
  {
    id: 3,
    name: 'File-LB-01',
    address: '192.168.1.12:8080',
    protocol: 'HTTP',
    backendCount: 2,
    status: '维护中',
    requestsPerSecond: '450',
    errorRate: 1.2,
  },
])

const backendServers = ref([
  {
    id: 1,
    name: 'Web-01',
    address: '192.168.1.101:80',
    health: '健康',
    weight: 1,
    connections: 125,
    responseTime: 45,
  },
  {
    id: 2,
    name: 'Web-02',
    address: '192.168.1.102:80',
    health: '健康',
    weight: 1,
    connections: 118,
    responseTime: 42,
  },
  {
    id: 3,
    name: 'Web-03',
    address: '192.168.1.103:80',
    health: '警告',
    weight: 1,
    connections: 95,
    responseTime: 68,
  },
  {
    id: 4,
    name: 'Web-04',
    address: '192.168.1.104:80',
    health: '健康',
    weight: 2,
    connections: 210,
    responseTime: 38,
  },
  {
    id: 5,
    name: 'API-01',
    address: '192.168.1.105:3000',
    health: '健康',
    weight: 1,
    connections: 85,
    responseTime: 52,
  },
  {
    id: 6,
    name: 'API-02',
    address: '192.168.1.106:3000',
    health: '异常',
    weight: 1,
    connections: 0,
    responseTime: 0,
  },
])

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    运行中: 'default',
    维护中: 'secondary',
    停止: 'destructive',
  }
  return variants[status] || 'outline'
}

function getHealthVariant(health: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    健康: 'default',
    警告: 'secondary',
    异常: 'destructive',
  }
  return variants[health] || 'outline'
}
</script>

<style scoped></style>
