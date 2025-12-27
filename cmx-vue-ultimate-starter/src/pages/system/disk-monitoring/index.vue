<template>
  <Page
    :title="transformI18n('system.disk-monitoring.title')"
    :description="transformI18n('system.disk-monitoring.description')"
    sticky
  >
    <!-- 磁盘监控概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总磁盘空间</UiCardTitle>
          <HardDrive class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ diskStats.totalSpace }}</div>
          <p class="text-xs text-muted-foreground">所有磁盘</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">已使用空间</UiCardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ diskStats.usedSpace }}</div>
          <p class="text-xs text-muted-foreground">{{ diskStats.usagePercentage }}% 使用率</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">可用空间</UiCardTitle>
          <Archive class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ diskStats.freeSpace }}</div>
          <p class="text-xs text-muted-foreground">剩余空间</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">磁盘数量</UiCardTitle>
          <Server class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ diskStats.diskCount }}</div>
          <p class="text-xs text-muted-foreground">在线设备</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 磁盘状态 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>磁盘状态</UiCardTitle>
          <UiCardDescription>磁盘使用情况和性能指标</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="disk in diskStatus"
              :key="disk.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <HardDrive class="h-8 w-8 text-chart-1" />
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ disk.name }}</h3>
                    <UiBadge :variant="getStatusColor(disk.status) as any">
                      {{ getStatusText(disk.status) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ disk.type }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">挂载点: {{ disk.mountPoint }}</p>
                  <div class="grid grid-cols-3 gap-4 mt-2 text-sm">
                    <div><span class="font-medium">总大小:</span> {{ disk.totalSize }}</div>
                    <div><span class="font-medium">已使用:</span> {{ disk.usedSize }}</div>
                    <div><span class="font-medium">可用:</span> {{ disk.freeSize }}</div>
                  </div>
                  <div class="mt-2">
                    <div class="w-full bg-muted rounded-full h-2">
                      <div
                        class="h-2 rounded-full transition-all"
                        :class="{
                          'bg-success': disk.usage < 80,
                          'bg-warning': disk.usage >= 80 && disk.usage < 90,
                          'bg-destructive': disk.usage >= 90,
                        }"
                        :style="{ width: disk.usage + '%' }"
                      ></div>
                    </div>
                    <div class="flex justify-between text-xs text-muted-foreground mt-1">
                      <span>0%</span>
                      <span>{{ disk.usage }}%</span>
                      <span>100%</span>
                    </div>
                  </div>
                </div>
              </div>
              <div class="text-right">
                <div class="text-lg font-bold" :class="getIoColor(disk.ioUtilization)">
                  {{ disk.ioUtilization }}%
                </div>
                <p class="text-sm text-muted-foreground">I/O 使用率</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 性能指标 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>性能指标</UiCardTitle>
          <UiCardDescription>磁盘读写性能统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-1">
                {{ performanceStats.readSpeed }} MB/s
              </div>
              <p class="text-sm text-muted-foreground">平均读取速度</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-2">
                {{ performanceStats.writeSpeed }} MB/s
              </div>
              <p class="text-sm text-muted-foreground">平均写入速度</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-3">{{ performanceStats.iops }} IOPS</div>
              <p class="text-sm text-muted-foreground">平均 IOPS</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-4">{{ performanceStats.queueLength }}</div>
              <p class="text-sm text-muted-foreground">平均队列长度</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 磁盘警报 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>磁盘警报</UiCardTitle>
          <UiCardDescription>磁盘相关的警告和错误</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="alert in diskAlerts"
              :key="alert.id"
              class="flex items-center justify-between p-3 rounded-lg border"
              :class="{
                'border-destructive/20 bg-destructive/10': alert.severity === 'high',
                'border-warning/20 bg-warning/10': alert.severity === 'medium',
                'border-chart-3/20 bg-chart-3/10': alert.severity === 'low',
              }"
            >
              <div class="flex items-center space-x-3">
                <AlertTriangle
                  class="h-5 w-5"
                  :class="{
                    'text-destructive': alert.severity === 'high',
                    'text-warning': alert.severity === 'medium',
                    'text-chart-3': alert.severity === 'low',
                  }"
                />
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ alert.title }}</h3>
                    <UiBadge :variant="getAlertSeverityColor(alert.severity) as any">
                      {{ getAlertSeverityText(alert.severity) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ alert.description }}</p>
                  <p class="text-xs text-muted-foreground">{{ alert.timestamp }}</p>
                </div>
              </div>
              <UiButton variant="outline" size="sm">
                <Settings class="h-4 w-4 mr-1" />
                处理
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline">
          <RefreshCw class="h-4 w-4 mr-2" />
          刷新状态
        </UiButton>
        <UiButton variant="outline">
          <AlertTriangle class="h-4 w-4 mr-2" />
          运行诊断
        </UiButton>
        <UiButton>
          <Download class="h-4 w-4 mr-2" />
          导出报告
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const diskStats = ref({
  totalSpace: '2.0 TB',
  usedSpace: '1.2 TB',
  freeSpace: '0.8 TB',
  usagePercentage: 60,
  diskCount: 3,
})

const diskStatus = ref([
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
    ioUtilization: 45,
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
    ioUtilization: 32,
  },
  {
    id: 3,
    name: 'sdc1',
    type: 'SSD',
    mountPoint: '/backup',
    totalSize: '1.0 TB',
    usedSize: '850 GB',
    freeSize: '150 GB',
    usage: 85,
    status: 'warning',
    ioUtilization: 68,
  },
])

const performanceStats = ref({
  readSpeed: 125,
  writeSpeed: 89,
  iops: 2450,
  queueLength: 2.3,
})

const diskAlerts = ref([
  {
    id: 1,
    title: '磁盘空间不足',
    description: '/backup 分区使用率超过 85%',
    severity: 'high',
    timestamp: '2024-01-15 14:30:22',
  },
  {
    id: 2,
    title: 'I/O 性能下降',
    description: 'sdc1 磁盘 I/O 使用率持续偏高',
    severity: 'medium',
    timestamp: '2024-01-15 13:45:10',
  },
  {
    id: 3,
    title: '磁盘温度警告',
    description: 'sdb1 磁盘温度超过 50°C',
    severity: 'low',
    timestamp: '2024-01-15 12:15:33',
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

const getIoColor = (ioUtilization: number) => {
  if (ioUtilization >= 80) return 'text-destructive'
  if (ioUtilization >= 60) return 'text-warning'
  return 'text-success'
}

const getAlertSeverityColor = (severity: string) => {
  const colors: Record<string, string> = {
    high: 'destructive',
    medium: 'warning',
    low: 'chart-3',
  }
  return colors[severity] || 'default'
}

const getAlertSeverityText = (severity: string) => {
  const texts: Record<string, string> = {
    high: '高',
    medium: '中',
    low: '低',
  }
  return texts[severity] || severity
}
</script>

<style></style>
