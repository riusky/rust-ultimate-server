<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const backupConfig = ref({
  autoBackup: true,
  backupInterval: 24,
  retentionDays: 30,
  compressionLevel: 'medium',
  encryption: false,
  backupLocation: '/var/backups',
  maxBackupSize: 100,
  notifyOnComplete: true,
})

const backupJobs = ref([
  {
    id: 1,
    name: '数据库备份',
    type: 'database',
    schedule: 'daily',
    lastRun: '2024-01-15 02:00:00',
    nextRun: '2024-01-16 02:00:00',
    status: 'completed',
    size: '45.2 GB',
    enabled: true,
  },
  {
    id: 2,
    name: '文件系统备份',
    type: 'filesystem',
    schedule: 'weekly',
    lastRun: '2024-01-14 03:00:00',
    nextRun: '2024-01-21 03:00:00',
    status: 'completed',
    size: '128.5 GB',
    enabled: true,
  },
  {
    id: 3,
    name: '配置备份',
    type: 'config',
    schedule: 'daily',
    lastRun: '2024-01-15 01:00:00',
    nextRun: '2024-01-16 01:00:00',
    status: 'failed',
    size: '0 B',
    enabled: true,
  },
])

const backupHistory = ref([
  {
    id: 1,
    jobName: '数据库备份',
    startTime: '2024-01-15 02:00:00',
    endTime: '2024-01-15 02:15:30',
    duration: '15分30秒',
    size: '45.2 GB',
    status: 'completed',
    location: '/var/backups/db_20240115_020000.tar.gz',
  },
  {
    id: 2,
    jobName: '文件系统备份',
    startTime: '2024-01-14 03:00:00',
    endTime: '2024-01-14 04:12:45',
    duration: '1小时12分45秒',
    size: '128.5 GB',
    status: 'completed',
    location: '/var/backups/fs_20240114_030000.tar.gz',
  },
  {
    id: 3,
    jobName: '配置备份',
    startTime: '2024-01-15 01:00:00',
    endTime: '2024-01-15 01:02:20',
    duration: '2分20秒',
    size: '0 B',
    status: 'failed',
    location: '/var/backups/config_20240115_010000.tar.gz',
    error: '磁盘空间不足',
  },
])

const storageStats = ref({
  totalBackupSize: '173.7 GB',
  availableSpace: '826.3 GB',
  backupCount: 24,
  oldestBackup: '2024-01-01',
  newestBackup: '2024-01-15',
})

const runBackup = (jobId: number) => {
  console.log('运行备份任务:', jobId)
}

const editJob = (jobId: number) => {
  console.log('编辑备份任务:', jobId)
}

const deleteJob = (jobId: number) => {
  console.log('删除备份任务:', jobId)
}

const restoreBackup = (backupId: number) => {
  console.log('恢复备份:', backupId)
}

const deleteBackup = (backupId: number) => {
  console.log('删除备份:', backupId)
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    completed: 'success',
    failed: 'destructive',
    running: 'warning',
    pending: 'outline',
  }
  return colors[status] || 'default'
}

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    completed: '完成',
    failed: '失败',
    running: '进行中',
    pending: '等待中',
  }
  return texts[status] || status
}

const getJobTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    database: 'chart-1',
    filesystem: 'chart-2',
    config: 'chart-3',
  }
  return colors[type] || 'default'
}

const getJobTypeText = (type: string) => {
  const texts: Record<string, string> = {
    database: '数据库',
    filesystem: '文件系统',
    config: '配置',
  }
  return texts[type] || type
}
</script>

<template>
  <Page
    :title="transformI18n('system.backup.title')"
    :description="transformI18n('system.backup.description')"
    sticky
  >
    <!-- 备份统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总备份大小</UiCardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.totalBackupSize }}</div>
          <p class="text-xs text-muted-foreground">{{ storageStats.backupCount }} 个备份</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">可用空间</UiCardTitle>
          <HardDrive class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.availableSpace }}</div>
          <p class="text-xs text-muted-foreground">剩余存储空间</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃任务</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ backupJobs.filter((job) => job.enabled).length }}</div>
          <p class="text-xs text-muted-foreground">已启用任务</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">备份范围</UiCardTitle>
          <Calendar class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ storageStats.oldestBackup }}</div>
          <p class="text-xs text-muted-foreground">至 {{ storageStats.newestBackup }}</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 备份配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>备份配置</UiCardTitle>
          <UiCardDescription>备份策略和全局设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="auto-backup" v-model="backupConfig.autoBackup" />
                <UiLabel for="auto-backup">启用自动备份</UiLabel>
              </div>
              <div>
                <UiLabel for="backup-interval">备份间隔 (小时)</UiLabel>
                <UiInput id="backup-interval" v-model="backupConfig.backupInterval" type="number" />
              </div>
              <div>
                <UiLabel for="retention-days">保留天数</UiLabel>
                <UiInput id="retention-days" v-model="backupConfig.retentionDays" type="number" />
              </div>
              <div>
                <UiLabel for="max-size">最大备份大小 (GB)</UiLabel>
                <UiInput id="max-size" v-model="backupConfig.maxBackupSize" type="number" />
              </div>
              <div>
                <UiLabel for="compression">压缩级别</UiLabel>
                <UiSelect v-model="backupConfig.compressionLevel">
                  <UiSelectTrigger>
                    <UiSelectValue />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="low">低</UiSelectItem>
                    <UiSelectItem value="medium">中</UiSelectItem>
                    <UiSelectItem value="high">高</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="encryption" v-model="backupConfig.encryption" />
                <UiLabel for="encryption">启用加密</UiLabel>
              </div>
            </div>
            <div>
              <UiLabel for="backup-location">备份位置</UiLabel>
              <UiInput id="backup-location" v-model="backupConfig.backupLocation" />
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="notify" v-model="backupConfig.notifyOnComplete" />
              <UiLabel for="notify">备份完成时通知</UiLabel>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 备份任务 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>备份任务</UiCardTitle>
          <UiCardDescription>已配置的备份任务</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div v-for="job in backupJobs" :key="job.id" class="p-3 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center space-x-2">
                  <h3 class="font-semibold">{{ job.name }}</h3>
                  <UiBadge :variant="getJobTypeColor(job.type) as any">
                    {{ getJobTypeText(job.type) }}
                  </UiBadge>
                </div>
                <UiSwitch v-model="job.enabled" />
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm mb-2">
                <div>
                  <span class="font-medium">计划:</span>
                  {{ job.schedule === 'daily' ? '每日' : '每周' }}
                </div>
                <div><span class="font-medium">大小:</span> {{ job.size }}</div>
                <div><span class="font-medium">上次运行:</span> {{ job.lastRun }}</div>
                <div>
                  <span class="font-medium">状态:</span>
                  <UiBadge :variant="getStatusColor(job.status) as any" class="ml-1">
                    {{ getStatusText(job.status) }}
                  </UiBadge>
                </div>
              </div>
              <div class="flex justify-between">
                <UiButton variant="outline" size="sm" @click="runBackup(job.id)">
                  <Play class="h-3 w-3 mr-1" />
                  立即运行
                </UiButton>
                <div class="flex space-x-1">
                  <UiButton variant="ghost" size="sm" @click="editJob(job.id)">
                    <Edit class="h-3 w-3" />
                  </UiButton>
                  <UiButton variant="ghost" size="sm" @click="deleteJob(job.id)">
                    <Trash2 class="h-3 w-3" />
                  </UiButton>
                </div>
              </div>
            </div>
          </div>
          <div class="mt-4">
            <UiButton class="w-full">
              <Plus class="h-4 w-4 mr-2" />
              添加任务
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 备份历史 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>备份历史</UiCardTitle>
          <UiCardDescription>最近的备份执行记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="backup in backupHistory"
              :key="backup.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ backup.jobName }}</h3>
                    <UiBadge :variant="getStatusColor(backup.status) as any">
                      {{ getStatusText(backup.status) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    开始时间: {{ backup.startTime }} • 持续时间: {{ backup.duration }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    大小: {{ backup.size }} • 位置: {{ backup.location }}
                    <span v-if="backup.error" class="text-destructive">
                      • 错误: {{ backup.error }}</span
                    >
                  </p>
                </div>
              </div>
              <div class="flex space-x-2">
                <UiButton
                  variant="outline"
                  size="sm"
                  :disabled="backup.status !== 'completed'"
                  @click="restoreBackup(backup.id)"
                >
                  <Upload class="h-4 w-4 mr-1" />
                  恢复
                </UiButton>
                <UiButton variant="outline" size="sm" @click="deleteBackup(backup.id)">
                  <Trash2 class="h-4 w-4 mr-1" />
                  删除
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline">
          <RefreshCw class="h-4 w-4 mr-2" />
          验证备份
        </UiButton>
        <UiButton>
          <Save class="h-4 w-4 mr-2" />
          保存配置
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<style></style>
