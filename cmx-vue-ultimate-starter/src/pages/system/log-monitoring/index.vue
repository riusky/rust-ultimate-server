<template>
  <Page title="日志监控" description="实时监控和分析系统日志" sticky>
    <template #actions>
      <UiButton>
        <Download class="w-4 h-4 mr-2" />
        导出日志
      </UiButton>
      <UiButton variant="outline">
        <Filter class="w-4 h-4 mr-2" />
        筛选
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 日志统计 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总日志数</UiCardTitle>
            <FileText class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">12,456</div>
            <p class="text-xs text-muted-foreground">今日新增 1,234 条</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">错误日志</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-destructive">45</div>
            <p class="text-xs text-muted-foreground">较昨日 +5</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">警告日志</UiCardTitle>
            <AlertCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">128</div>
            <p class="text-xs text-muted-foreground">较昨日 -12</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">日志大小</UiCardTitle>
            <HardDrive class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">2.3GB</div>
            <p class="text-xs text-muted-foreground">总日志文件大小</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 日志级别分布 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>日志级别分布</UiCardTitle>
          <UiCardDescription>不同级别日志的数量统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <div class="w-3 h-3 bg-destructive rounded-full mx-auto mb-2"></div>
              <p class="text-2xl font-bold">45</p>
              <p class="text-sm text-muted-foreground">ERROR</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <div class="w-3 h-3 bg-chart-4 rounded-full mx-auto mb-2"></div>
              <p class="text-2xl font-bold">128</p>
              <p class="text-sm text-muted-foreground">WARN</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <div class="w-3 h-3 bg-chart-3 rounded-full mx-auto mb-2"></div>
              <p class="text-2xl font-bold">2,345</p>
              <p class="text-sm text-muted-foreground">INFO</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <div class="w-3 h-3 bg-chart-2 rounded-full mx-auto mb-2"></div>
              <p class="text-2xl font-bold">8,934</p>
              <p class="text-sm text-muted-foreground">DEBUG</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <div class="w-3 h-3 bg-muted-foreground rounded-full mx-auto mb-2"></div>
              <p class="text-2xl font-bold">1,004</p>
              <p class="text-sm text-muted-foreground">TRACE</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 实时日志流 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>实时日志流</UiCardTitle>
          <UiCardDescription>最近产生的系统日志</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">时间</th>
                  <th scope="col" class="px-6 py-3">级别</th>
                  <th scope="col" class="px-6 py-3">来源</th>
                  <th scope="col" class="px-6 py-3">消息</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="log in logs" :key="log.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">{{ log.timestamp }}</td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getLogLevelVariant(log.level)">
                      {{ log.level }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ log.source }}</td>
                  <td class="px-6 py-4">
                    <div class="max-w-md">
                      <p class="text-sm">{{ log.message }}</p>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 日志搜索 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>日志搜索</UiCardTitle>
          <UiCardDescription>搜索和分析历史日志</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex flex-col sm:flex-row gap-4">
              <div class="flex-1">
                <UiInput placeholder="搜索日志内容..." class="max-w-lg">
                  <template #prefix>
                    <Search class="w-4 h-4" />
                  </template>
                </UiInput>
              </div>
              <UiSelect default-value="all">
                <UiSelectTrigger class="w-[180px]">
                  <UiSelectValue placeholder="日志级别" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="all">全部级别</UiSelectItem>
                  <UiSelectItem value="error">ERROR</UiSelectItem>
                  <UiSelectItem value="warn">WARN</UiSelectItem>
                  <UiSelectItem value="info">INFO</UiSelectItem>
                  <UiSelectItem value="debug">DEBUG</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="flex items-center space-x-4 text-sm">
              <div class="flex items-center space-x-2">
                <UiLabel for="start-time">开始时间</UiLabel>
                <UiInput id="start-time" type="datetime-local" />
              </div>
              <div class="flex items-center space-x-2">
                <UiLabel for="end-time">结束时间</UiLabel>
                <UiInput id="end-time" type="datetime-local" />
              </div>
              <UiButton>搜索</UiButton>
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
  Download,
  Filter,
  FileText,
  AlertTriangle,
  AlertCircle,
  HardDrive,
  Search,
} from 'lucide-vue-next'

const logs = ref([
  {
    id: 1,
    timestamp: '2024-01-15 14:30:25',
    level: 'ERROR',
    source: 'nginx',
    message: 'Connection refused while connecting to upstream',
  },
  {
    id: 2,
    timestamp: '2024-01-15 14:30:20',
    level: 'WARN',
    source: 'mysql',
    message: 'Aborted connection to database',
  },
  {
    id: 3,
    timestamp: '2024-01-15 14:30:15',
    level: 'INFO',
    source: 'application',
    message: 'User login successful: admin',
  },
  {
    id: 4,
    timestamp: '2024-01-15 14:30:10',
    level: 'DEBUG',
    source: 'redis',
    message: 'Cache hit for key: user_session_123',
  },
  {
    id: 5,
    timestamp: '2024-01-15 14:30:05',
    level: 'ERROR',
    source: 'api',
    message: 'Failed to process request: timeout',
  },
  {
    id: 6,
    timestamp: '2024-01-15 14:30:00',
    level: 'INFO',
    source: 'system',
    message: 'System backup completed successfully',
  },
])

function getLogLevelVariant(level: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    ERROR: 'destructive',
    WARN: 'secondary',
    INFO: 'default',
    DEBUG: 'outline',
  }
  return variants[level] || 'outline'
}
</script>

<style scoped></style>
