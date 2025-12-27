<template>
  <Page title="数据库管理" description="管理系统数据库配置和状态监控" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        添加数据库
      </UiButton>
      <UiButton variant="outline">
        <RefreshCw class="w-4 h-4 mr-2" />
        刷新
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 数据库概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">数据库实例</UiCardTitle>
            <Database class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">8</div>
            <p class="text-xs text-muted-foreground">数据库实例总数</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">运行中</UiCardTitle>
            <Activity class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">7</div>
            <p class="text-xs text-muted-foreground">正常运行的数据库</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">连接数</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">245</div>
            <p class="text-xs text-muted-foreground">活跃数据库连接</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">查询数/秒</UiCardTitle>
            <TrendingUp class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">1.8k</div>
            <p class="text-xs text-muted-foreground">数据库查询速率</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 数据库列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>数据库列表</UiCardTitle>
          <UiCardDescription>系统数据库实例配置和状态</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">数据库</th>
                  <th scope="col" class="px-6 py-3">类型</th>
                  <th scope="col" class="px-6 py-3">版本</th>
                  <th scope="col" class="px-6 py-3">连接数</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">大小</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="db in databases" :key="db.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <Database class="w-4 h-4 text-primary" />
                      </div>
                      <div>
                        <div class="font-medium">{{ db.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ db.host }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge variant="outline">{{ db.type }}</UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ db.version }}</td>
                  <td class="px-6 py-4">
                    <span class="font-medium text-chart-3">{{ db.connections }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(db.status)">
                      {{ db.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ db.size }}</td>
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

      <!-- 性能指标 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>性能指标</UiCardTitle>
          <UiCardDescription>数据库性能监控指标</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <Cpu class="w-8 h-8 text-chart-3 mx-auto mb-2" />
              <p class="text-2xl font-bold">65%</p>
              <p class="text-sm text-muted-foreground">CPU使用率</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <HardDrive class="w-8 h-8 text-chart-2 mx-auto mb-2" />
              <p class="text-2xl font-bold">78%</p>
              <p class="text-sm text-muted-foreground">内存使用率</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-4 mx-auto mb-2" />
              <p class="text-2xl font-bold">45%</p>
              <p class="text-sm text-muted-foreground">磁盘使用率</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Activity class="w-8 h-8 text-chart-5 mx-auto mb-2" />
              <p class="text-2xl font-bold">1.8k</p>
              <p class="text-sm text-muted-foreground">查询数/秒</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 数据库类型分布 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>数据库类型分布</UiCardTitle>
          <UiCardDescription>不同类型数据库实例统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-3 mx-auto mb-2" />
              <p class="text-2xl font-bold">3</p>
              <p class="text-sm text-muted-foreground">MySQL</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-2 mx-auto mb-2" />
              <p class="text-2xl font-bold">2</p>
              <p class="text-sm text-muted-foreground">PostgreSQL</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-4 mx-auto mb-2" />
              <p class="text-2xl font-bold">2</p>
              <p class="text-sm text-muted-foreground">Redis</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-5 mx-auto mb-2" />
              <p class="text-2xl font-bold">1</p>
              <p class="text-sm text-muted-foreground">MongoDB</p>
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
  Database,
  Activity,
  Users,
  TrendingUp,
  Settings,
  Cpu,
  HardDrive,
} from 'lucide-vue-next'

const databases = ref([
  {
    id: 1,
    name: '用户数据库',
    host: '192.168.1.201:3306',
    type: 'MySQL',
    version: '8.0.32',
    connections: 85,
    status: '运行中',
    size: '2.3GB',
  },
  {
    id: 2,
    name: '业务数据库',
    host: '192.168.1.202:3306',
    type: 'MySQL',
    version: '8.0.32',
    connections: 120,
    status: '运行中',
    size: '5.6GB',
  },
  {
    id: 3,
    name: '日志数据库',
    host: '192.168.1.203:5432',
    type: 'PostgreSQL',
    version: '14.8',
    connections: 45,
    status: '运行中',
    size: '8.9GB',
  },
  {
    id: 4,
    name: '缓存数据库',
    host: '192.168.1.204:6379',
    type: 'Redis',
    version: '7.0.11',
    connections: 25,
    status: '运行中',
    size: '512MB',
  },
  {
    id: 5,
    name: '配置数据库',
    host: '192.168.1.205:5432',
    type: 'PostgreSQL',
    version: '14.8',
    connections: 15,
    status: '维护中',
    size: '1.2GB',
  },
  {
    id: 6,
    name: '会话数据库',
    host: '192.168.1.206:6379',
    type: 'Redis',
    version: '7.0.11',
    connections: 0,
    status: '停止',
    size: '256MB',
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
</script>

<style scoped></style>
