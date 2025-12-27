<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const storageStats = ref({
  totalSpace: '2.0 TB',
  usedSpace: '1.2 TB',
  freeSpace: '0.8 TB',
  usagePercentage: 60,
})

const storageDevices = ref([
  {
    id: 1,
    name: 'sda1',
    type: 'SSD',
    mountPoint: '/',
    totalSize: '500 GB',
    usedSize: '320 GB',
    freeSize: '180 GB',
    usage: 64,
    status: 'healthy',
  },
  {
    id: 2,
    name: 'sdb1',
    type: 'HDD',
    mountPoint: '/data',
    totalSize: '1.5 TB',
    usedSize: '880 GB',
    freeSize: '620 GB',
    usage: 59,
    status: 'healthy',
  },
  {
    id: 3,
    name: 'sdc1',
    type: 'SSD',
    mountPoint: '/backup',
    totalSize: '1.0 TB',
    usedSize: '450 GB',
    freeSize: '550 GB',
    usage: 45,
    status: 'warning',
  },
])

const storagePools = ref([
  {
    id: 1,
    name: 'Data Pool',
    type: 'RAID 5',
    totalSize: '4.0 TB',
    usedSize: '2.8 TB',
    freeSize: '1.2 TB',
    usage: 70,
    devices: ['sda1', 'sdb1', 'sdc1'],
    status: 'healthy',
  },
  {
    id: 2,
    name: 'Backup Pool',
    type: 'RAID 1',
    totalSize: '2.0 TB',
    usedSize: '1.5 TB',
    freeSize: '0.5 TB',
    usage: 75,
    devices: ['sdd1', 'sde1'],
    status: 'healthy',
  },
])

const recentActivities = ref([
  {
    id: 1,
    type: 'backup',
    description: '数据库备份完成',
    size: '45.2 GB',
    timestamp: '2024-01-15 14:30:22',
    status: 'completed',
  },
  {
    id: 2,
    type: 'cleanup',
    description: '临时文件清理',
    size: '12.8 GB',
    timestamp: '2024-01-15 13:45:10',
    status: 'completed',
  },
  {
    id: 3,
    type: 'snapshot',
    description: '系统快照创建',
    size: '25.6 GB',
    timestamp: '2024-01-15 12:15:33',
    status: 'completed',
  },
])

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    healthy: 'success',
    warning: 'warning',
    error: 'destructive',
  }
  return colors[status] || 'default'
}

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    healthy: '健康',
    warning: '警告',
    error: '错误',
  }
  return texts[status] || status
}

const runCleanup = () => {
  console.log('运行存储清理')
}

const createSnapshot = () => {
  console.log('创建系统快照')
}

const expandStorage = () => {
  console.log('扩展存储空间')
}
</script>

<template>
  <Page
    :title="transformI18n('system.storage.title')"
    :description="transformI18n('system.storage.description')"
    sticky
  >
    <!-- 存储概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总存储空间</UiCardTitle>
          <HardDrive class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.totalSpace }}</div>
          <p class="text-xs text-muted-foreground">总容量</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">已使用</UiCardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.usedSpace }}</div>
          <p class="text-xs text-muted-foreground">{{ storageStats.usagePercentage }}% 使用率</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">可用空间</UiCardTitle>
          <Archive class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.freeSpace }}</div>
          <p class="text-xs text-muted-foreground">剩余空间</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">存储设备</UiCardTitle>
          <Server class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageDevices.length }}</div>
          <p class="text-xs text-muted-foreground">在线设备</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 存储设备 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>存储设备</UiCardTitle>
          <UiCardDescription>物理存储设备和分区信息</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="device in storageDevices"
              :key="device.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <HardDrive class="h-8 w-8 text-chart-1" />
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ device.name }}</h3>
                    <UiBadge :variant="getStatusColor(device.status) as any">
                      {{ getStatusText(device.status) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ device.type }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">挂载点: {{ device.mountPoint }}</p>
                  <div class="flex items-center space-x-4 mt-2 text-sm">
                    <span>总大小: {{ device.totalSize }}</span>
                    <span>已使用: {{ device.usedSize }}</span>
                    <span>可用: {{ device.freeSize }}</span>
                  </div>
                  <div class="mt-2">
                    <div class="w-full bg-muted rounded-full h-2">
                      <div
                        class="h-2 rounded-full transition-all"
                        :class="{
                          'bg-success': device.usage < 80,
                          'bg-warning': device.usage >= 80 && device.usage < 90,
                          'bg-destructive': device.usage >= 90,
                        }"
                        :style="{ width: device.usage + '%' }"
                      ></div>
                    </div>
                    <div class="flex justify-between text-xs text-muted-foreground mt-1">
                      <span>0%</span>
                      <span>{{ device.usage }}%</span>
                      <span>100%</span>
                    </div>
                  </div>
                </div>
              </div>
              <div class="flex space-x-2">
                <UiButton variant="outline" size="sm">
                  <Settings class="h-4 w-4 mr-1" />
                  配置
                </UiButton>
                <UiButton variant="outline" size="sm">
                  <RefreshCw class="h-4 w-4 mr-1" />
                  检查
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 存储池 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>存储池</UiCardTitle>
          <UiCardDescription>逻辑存储池配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div v-for="pool in storagePools" :key="pool.id" class="p-4 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <h3 class="font-semibold">{{ pool.name }}</h3>
                <UiBadge :variant="getStatusColor(pool.status) as any">
                  {{ getStatusText(pool.status) }}
                </UiBadge>
              </div>
              <p class="text-sm text-muted-foreground mb-2">{{ pool.type }}</p>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span>总大小:</span>
                  <span>{{ pool.totalSize }}</span>
                </div>
                <div class="flex justify-between">
                  <span>已使用:</span>
                  <span>{{ pool.usedSize }}</span>
                </div>
                <div class="flex justify-between">
                  <span>可用:</span>
                  <span>{{ pool.freeSize }}</span>
                </div>
                <div class="flex justify-between">
                  <span>使用率:</span>
                  <span
                    :class="{
                      'text-success': pool.usage < 80,
                      'text-warning': pool.usage >= 80 && pool.usage < 90,
                      'text-destructive': pool.usage >= 90,
                    }"
                    >{{ pool.usage }}%</span
                  >
                </div>
              </div>
              <div class="mt-3">
                <p class="text-xs text-muted-foreground">设备: {{ pool.devices.join(', ') }}</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 最近活动 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>最近活动</UiCardTitle>
          <UiCardDescription>存储相关的操作记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="activity in recentActivities"
              :key="activity.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ activity.description }}</h3>
                    <UiBadge variant="secondary">{{ activity.status }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    类型: {{ activity.type }} • 大小: {{ activity.size }} • 时间:
                    {{ activity.timestamp }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="runCleanup">
          <Trash2 class="h-4 w-4 mr-2" />
          运行清理
        </UiButton>
        <UiButton variant="outline" @click="createSnapshot">
          <Camera class="h-4 w-4 mr-2" />
          创建快照
        </UiButton>
        <UiButton @click="expandStorage">
          <Plus class="h-4 w-4 mr-2" />
          扩展存储
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<style></style>
