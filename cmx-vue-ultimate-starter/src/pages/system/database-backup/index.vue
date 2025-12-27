<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const backupConfig = ref({
  autoBackup: true,
  backupInterval: 24,
  retentionDays: 30,
  compression: true,
  encryption: false,
  backupPath: '/var/backups/database',
  maxBackupSize: 10,
})

const backupHistory = ref([
  {
    id: 1,
    database: 'app_db',
    size: '2.3 GB',
    status: 'completed',
    startTime: '2024-01-15 02:00:00',
    endTime: '2024-01-15 02:15:30',
    duration: '15分30秒',
  },
  {
    id: 2,
    database: 'user_db',
    size: '1.8 GB',
    status: 'completed',
    startTime: '2024-01-15 02:00:00',
    endTime: '2024-01-15 02:12:45',
    duration: '12分45秒',
  },
  {
    id: 3,
    database: 'log_db',
    size: '4.1 GB',
    status: 'failed',
    startTime: '2024-01-14 02:00:00',
    endTime: '2024-01-14 02:08:20',
    duration: '8分20秒',
    error: '磁盘空间不足',
  },
  {
    id: 4,
    database: 'app_db',
    size: '2.2 GB',
    status: 'completed',
    startTime: '2024-01-14 02:00:00',
    endTime: '2024-01-14 02:14:10',
    duration: '14分10秒',
  },
])

const databases = ref([
  { name: 'app_db', size: '25.4 GB', tables: 42, status: 'online' },
  { name: 'user_db', size: '18.7 GB', tables: 28, status: 'online' },
  { name: 'log_db', size: '45.2 GB', tables: 15, status: 'online' },
  { name: 'config_db', size: '0.8 GB', tables: 8, status: 'offline' },
])

const selectedDatabases = ref(['app_db', 'user_db'])

const startBackup = () => {
  // 开始备份逻辑
  console.log('开始备份选中的数据库:', selectedDatabases.value)
}

const restoreBackup = (backupId: number) => {
  // 恢复备份逻辑
  console.log('恢复备份:', backupId)
}

const deleteBackup = (backupId: number) => {
  // 删除备份逻辑
  console.log('删除备份:', backupId)
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    completed: 'success',
    failed: 'destructive',
    running: 'warning',
  }
  return colors[status] || 'default'
}

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    completed: '完成',
    failed: '失败',
    running: '进行中',
  }
  return texts[status] || status
}
</script>

<template>
  <Page
    :title="transformI18n('system.database-backup.title')"
    :description="transformI18n('system.database-backup.description')"
    sticky
  >
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 备份配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>备份配置</UiCardTitle>
          <UiCardDescription>数据库备份策略和设置</UiCardDescription>
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
              <div class="flex items-center space-x-2">
                <UiCheckbox id="compression" v-model="backupConfig.compression" />
                <UiLabel for="compression">启用压缩</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="encryption" v-model="backupConfig.encryption" />
                <UiLabel for="encryption">启用加密</UiLabel>
              </div>
            </div>
            <div>
              <UiLabel for="backup-path">备份路径</UiLabel>
              <UiInput id="backup-path" v-model="backupConfig.backupPath" />
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 数据库选择 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>数据库选择</UiCardTitle>
          <UiCardDescription>选择要备份的数据库</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="db in databases"
              :key="db.name"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <UiCheckbox
                  :id="`db-${db.name}`"
                  :checked="selectedDatabases.includes(db.name)"
                  @update:checked="
                    (checked: boolean) => {
                      if (checked) {
                        selectedDatabases.push(db.name)
                      } else {
                        selectedDatabases = selectedDatabases.filter((name) => name !== db.name)
                      }
                    }
                  "
                  :disabled="db.status === 'offline'"
                />
                <div>
                  <UiLabel :for="`db-${db.name}`" class="font-medium">{{ db.name }}</UiLabel>
                  <p class="text-sm text-muted-foreground">{{ db.size }} • {{ db.tables }} 表</p>
                </div>
              </div>
              <UiBadge :variant="db.status === 'online' ? 'secondary' : 'destructive'">
                {{ db.status === 'online' ? '在线' : '离线' }}
              </UiBadge>
            </div>
          </div>
          <div class="mt-4">
            <UiButton class="w-full" @click="startBackup">
              <Download class="h-4 w-4 mr-2" />
              立即备份
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 备份历史 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>备份历史</UiCardTitle>
          <UiCardDescription>最近的数据库备份记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="backup in backupHistory"
              :key="backup.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <Database class="h-8 w-8 text-chart-1" />
                <div class="flex-1">
                  <div class="flex items-center space-x-2">
                    <h3 class="font-semibold">{{ backup.database }}</h3>
                    <UiBadge :variant="getStatusColor(backup.status) as any">
                      {{ getStatusText(backup.status) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    大小: {{ backup.size }} • 开始时间: {{ backup.startTime }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    持续时间: {{ backup.duration }}
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

      <!-- 备份统计 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>备份统计</UiCardTitle>
          <UiCardDescription>备份性能和存储使用情况</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-1">12.3 GB</div>
              <p class="text-sm text-muted-foreground">总备份大小</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-2">87%</div>
              <p class="text-sm text-muted-foreground">备份成功率</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-3">45.2 GB</div>
              <p class="text-sm text-muted-foreground">可用存储空间</p>
            </div>
            <div class="text-center p-4 rounded-lg bg-muted/50">
              <div class="text-2xl font-bold text-chart-4">3</div>
              <p class="text-sm text-muted-foreground">今日备份次数</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<style></style>
