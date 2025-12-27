<template>
  <Page title="资源使用" description="监控和管理系统资源使用情况" sticky>
    <template #actions>
      <UiButton>
        <Download class="w-4 h-4 mr-2" />
        导出报告
      </UiButton>
      <UiButton variant="outline">
        <RefreshCw class="w-4 h-4 mr-2" />
        刷新
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 资源概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">CPU使用率</UiCardTitle>
            <Cpu class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">32%</div>
            <p class="text-xs text-muted-foreground">较上周 +2%</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">内存使用</UiCardTitle>
            <HardDrive class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">3.2GB / 8GB</div>
            <p class="text-xs text-muted-foreground">40% 使用率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">磁盘空间</UiCardTitle>
            <Database class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">128GB / 500GB</div>
            <p class="text-xs text-muted-foreground">25% 使用率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">网络流量</UiCardTitle>
            <Network class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-5">2.4MB/s</div>
            <p class="text-xs text-muted-foreground">较上周 -0.5MB/s</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 资源使用趋势 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>资源使用趋势</UiCardTitle>
          <UiCardDescription>最近24小时资源使用情况</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-64 flex items-end justify-between space-x-1">
            <div
              v-for="(item, index) in resourceTrends"
              :key="index"
              class="flex flex-col items-center flex-1"
            >
              <div class="flex items-end space-x-1 flex-1 w-full">
                <div class="w-full bg-chart-3 rounded-t" :style="{ height: `${item.cpu}px` }"></div>
                <div
                  class="w-full bg-chart-2 rounded-t"
                  :style="{ height: `${item.memory}px` }"
                ></div>
                <div
                  class="w-full bg-chart-4 rounded-t"
                  :style="{ height: `${item.disk}px` }"
                ></div>
              </div>
              <span class="text-xs mt-2">{{ item.time }}</span>
            </div>
          </div>
          <div class="flex justify-center space-x-6 mt-4 text-sm">
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-3 rounded"></div>
              <span>CPU</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-2 rounded"></div>
              <span>内存</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-4 rounded"></div>
              <span>磁盘</span>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 服务器资源详情 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>服务器资源详情</UiCardTitle>
          <UiCardDescription>各服务器资源使用情况</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">服务器</th>
                  <th scope="col" class="px-6 py-3">CPU</th>
                  <th scope="col" class="px-6 py-3">内存</th>
                  <th scope="col" class="px-6 py-3">磁盘</th>
                  <th scope="col" class="px-6 py-3">网络</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="server in servers" :key="server.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <Server class="w-4 h-4 text-primary" />
                      </div>
                      <div>
                        <div class="font-medium">{{ server.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ server.ip }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <div class="w-16 bg-muted rounded-full h-2">
                        <div
                          class="bg-chart-3 h-2 rounded-full"
                          :style="{ width: `${server.cpu}%` }"
                        ></div>
                      </div>
                      <span class="text-sm">{{ server.cpu }}%</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <div class="w-16 bg-muted rounded-full h-2">
                        <div
                          class="bg-chart-2 h-2 rounded-full"
                          :style="{ width: `${server.memory}%` }"
                        ></div>
                      </div>
                      <span class="text-sm">{{ server.memory }}%</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <div class="w-16 bg-muted rounded-full h-2">
                        <div
                          class="bg-chart-4 h-2 rounded-full"
                          :style="{ width: `${server.disk}%` }"
                        ></div>
                      </div>
                      <span class="text-sm">{{ server.disk }}%</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <div class="w-16 bg-muted rounded-full h-2">
                        <div
                          class="bg-chart-5 h-2 rounded-full"
                          :style="{ width: `${server.network}%` }"
                        ></div>
                      </div>
                      <span class="text-sm">{{ server.network }}%</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(server.status)">
                      {{ server.status }}
                    </UiBadge>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 资源优化建议 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>资源优化建议</UiCardTitle>
          <UiCardDescription>基于当前资源使用情况的优化建议</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="flex items-start space-x-3 p-4 border border-chart-3/20 bg-chart-3/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">内存优化</p>
                <p class="text-sm text-muted-foreground">建议清理缓存和优化内存分配策略</p>
              </div>
              <UiBadge variant="secondary">中等优先级</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-4/20 bg-chart-4/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">磁盘清理</p>
                <p class="text-sm text-muted-foreground">建议清理临时文件和日志文件</p>
              </div>
              <UiBadge variant="secondary">低优先级</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-2/20 bg-chart-2/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">网络优化</p>
                <p class="text-sm text-muted-foreground">建议优化网络配置和负载均衡</p>
              </div>
              <UiBadge variant="secondary">低优先级</UiBadge>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import { Download, RefreshCw, Cpu, HardDrive, Database, Network, Server } from 'lucide-vue-next'

const resourceTrends = ref([
  { time: '00:00', cpu: 20, memory: 35, disk: 22 },
  { time: '04:00', cpu: 18, memory: 32, disk: 20 },
  { time: '08:00', cpu: 45, memory: 60, disk: 28 },
  { time: '12:00', cpu: 65, memory: 75, disk: 35 },
  { time: '16:00', cpu: 55, memory: 65, disk: 32 },
  { time: '20:00', cpu: 35, memory: 45, disk: 25 },
  { time: '24:00', cpu: 25, memory: 38, disk: 23 },
])

const servers = ref([
  {
    id: 1,
    name: 'Web-01',
    ip: '192.168.1.101',
    cpu: 25,
    memory: 45,
    disk: 32,
    network: 28,
    status: '正常',
  },
  {
    id: 2,
    name: 'DB-01',
    ip: '192.168.1.102',
    cpu: 65,
    memory: 78,
    disk: 45,
    network: 15,
    status: '警告',
  },
  {
    id: 3,
    name: 'File-01',
    ip: '192.168.1.103',
    cpu: 92,
    memory: 85,
    disk: 88,
    network: 22,
    status: '异常',
  },
  {
    id: 4,
    name: 'App-01',
    ip: '192.168.1.104',
    cpu: 35,
    memory: 42,
    disk: 28,
    network: 18,
    status: '正常',
  },
  {
    id: 5,
    name: 'DB-02',
    ip: '192.168.1.105',
    cpu: 75,
    memory: 82,
    disk: 65,
    network: 12,
    status: '警告',
  },
])

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    正常: 'default',
    警告: 'secondary',
    异常: 'destructive',
  }
  return variants[status] || 'outline'
}
</script>

<style scoped></style>
