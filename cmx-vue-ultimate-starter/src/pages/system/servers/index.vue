<template>
  <Page title="服务器管理" description="管理系统服务器和运行状态" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        添加服务器
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 服务器概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总服务器数</UiCardTitle>
            <Server class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">12</div>
            <p class="text-xs text-muted-foreground">+2 较上月</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">运行中</UiCardTitle>
            <Activity class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">10</div>
            <p class="text-xs text-muted-foreground">83% 正常运行</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">异常</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-destructive">2</div>
            <p class="text-xs text-muted-foreground">需要关注</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">维护中</UiCardTitle>
            <Wrench class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">0</div>
            <p class="text-xs text-muted-foreground">无维护服务器</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 服务器列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>服务器列表</UiCardTitle>
          <UiCardDescription>管理系统中的所有服务器</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">服务器名称</th>
                  <th scope="col" class="px-6 py-3">IP地址</th>
                  <th scope="col" class="px-6 py-3">类型</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">CPU</th>
                  <th scope="col" class="px-6 py-3">内存</th>
                  <th scope="col" class="px-6 py-3">磁盘</th>
                  <th scope="col" class="px-6 py-3">操作</th>
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
                        <div class="text-sm text-muted-foreground">{{ server.os }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">{{ server.ip }}</td>
                  <td class="px-6 py-4">
                    <UiBadge variant="outline">{{ server.type }}</UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(server.status)">
                      {{ server.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <div class="w-16 bg-muted rounded-full h-2">
                        <div
                          class="bg-chart-2 h-2 rounded-full"
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
                          class="bg-chart-3 h-2 rounded-full"
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
                      <UiButton variant="ghost" size="sm">
                        <Monitor class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Settings class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Power class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 服务器分组 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>服务器分组</UiCardTitle>
          <UiCardDescription>按业务分组管理服务器</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="border rounded-lg p-4 space-y-2">
              <div class="flex items-center space-x-2">
                <Globe class="w-5 h-5 text-chart-3" />
                <h3 class="font-medium">Web服务器</h3>
              </div>
              <p class="text-sm text-muted-foreground">4台服务器</p>
              <div class="flex items-center space-x-1">
                <div class="w-2 h-2 bg-chart-2 rounded-full"></div>
                <span class="text-xs">全部正常</span>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-2">
              <div class="flex items-center space-x-2">
                <Database class="w-5 h-5 text-chart-2" />
                <h3 class="font-medium">数据库服务器</h3>
              </div>
              <p class="text-sm text-muted-foreground">3台服务器</p>
              <div class="flex items-center space-x-1">
                <div class="w-2 h-2 bg-chart-4 rounded-full"></div>
                <span class="text-xs">1台警告</span>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-2">
              <div class="flex items-center space-x-2">
                <FileText class="w-5 h-5 text-chart-5" />
                <h3 class="font-medium">文件服务器</h3>
              </div>
              <p class="text-sm text-muted-foreground">2台服务器</p>
              <div class="flex items-center space-x-1">
                <div class="w-2 h-2 bg-destructive rounded-full"></div>
                <span class="text-xs">1台异常</span>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-2">
              <div class="flex items-center space-x-2">
                <Cloud class="w-5 h-5 text-chart-4" />
                <h3 class="font-medium">应用服务器</h3>
              </div>
              <p class="text-sm text-muted-foreground">3台服务器</p>
              <div class="flex items-center space-x-1">
                <div class="w-2 h-2 bg-chart-2 rounded-full"></div>
                <span class="text-xs">全部正常</span>
              </div>
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
  Server,
  Activity,
  AlertTriangle,
  Wrench,
  Monitor,
  Settings,
  Power,
  Globe,
  Database,
  FileText,
  Cloud,
} from 'lucide-vue-next'

const servers = ref([
  {
    id: 1,
    name: 'Web-01',
    ip: '192.168.1.101',
    os: 'Ubuntu 22.04',
    type: 'Web服务器',
    status: '运行中',
    cpu: 25,
    memory: 45,
    disk: 32,
  },
  {
    id: 2,
    name: 'DB-01',
    ip: '192.168.1.102',
    os: 'CentOS 8',
    type: '数据库服务器',
    status: '运行中',
    cpu: 65,
    memory: 78,
    disk: 45,
  },
  {
    id: 3,
    name: 'File-01',
    ip: '192.168.1.103',
    os: 'Windows Server 2022',
    type: '文件服务器',
    status: '异常',
    cpu: 92,
    memory: 85,
    disk: 88,
  },
  {
    id: 4,
    name: 'App-01',
    ip: '192.168.1.104',
    os: 'Ubuntu 20.04',
    type: '应用服务器',
    status: '运行中',
    cpu: 35,
    memory: 42,
    disk: 28,
  },
  {
    id: 5,
    name: 'DB-02',
    ip: '192.168.1.105',
    os: 'CentOS 7',
    type: '数据库服务器',
    status: '警告',
    cpu: 75,
    memory: 82,
    disk: 65,
  },
  {
    id: 6,
    name: 'Web-02',
    ip: '192.168.1.106',
    os: 'Debian 11',
    type: 'Web服务器',
    status: '运行中',
    cpu: 18,
    memory: 32,
    disk: 25,
  },
])

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    运行中: 'default',
    警告: 'secondary',
    异常: 'destructive',
    维护中: 'outline',
  }
  return variants[status] || 'outline'
}
</script>

<style scoped></style>
