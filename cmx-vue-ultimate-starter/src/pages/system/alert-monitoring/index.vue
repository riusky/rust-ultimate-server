<template>
  <Page title="告警监控" description="实时监控系统告警状态和趋势" sticky>
    <template #actions>
      <UiButton>
        <RefreshCw class="w-4 h-4 mr-2" />
        刷新
      </UiButton>
      <UiButton variant="outline">
        <Filter class="w-4 h-4 mr-2" />
        筛选
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 告警概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">活跃告警</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-destructive">23</div>
            <p class="text-xs text-muted-foreground">需要立即处理</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">今日新增</UiCardTitle>
            <TrendingUp class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">12</div>
            <p class="text-xs text-muted-foreground">较昨日 +3</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">平均响应时间</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">15分钟</div>
            <p class="text-xs text-muted-foreground">告警响应时间</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">解决率</UiCardTitle>
            <CheckCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">85%</div>
            <p class="text-xs text-muted-foreground">告警解决成功率</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 告警趋势 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>告警趋势</UiCardTitle>
          <UiCardDescription>最近7天告警数量变化趋势</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-64 flex items-end justify-between space-x-2">
            <div
              v-for="(item, index) in alertTrends"
              :key="index"
              class="flex flex-col items-center flex-1"
            >
              <div class="flex items-end space-x-1 flex-1 w-full">
                <div
                  class="w-full bg-destructive rounded-t"
                  :style="{ height: `${item.critical * 2}px` }"
                ></div>
                <div
                  class="w-full bg-chart-4 rounded-t"
                  :style="{ height: `${item.warning * 2}px` }"
                ></div>
                <div
                  class="w-full bg-chart-3 rounded-t"
                  :style="{ height: `${item.info * 2}px` }"
                ></div>
              </div>
              <span class="text-xs mt-2">{{ item.day }}</span>
            </div>
          </div>
          <div class="flex justify-center space-x-6 mt-4 text-sm">
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-destructive rounded"></div>
              <span>严重</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-4 rounded"></div>
              <span>警告</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-3 rounded"></div>
              <span>提醒</span>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 实时告警 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>实时告警</UiCardTitle>
          <UiCardDescription>当前活跃的告警信息</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">告警名称</th>
                  <th scope="col" class="px-6 py-3">级别</th>
                  <th scope="col" class="px-6 py-3">来源</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">发生时间</th>
                  <th scope="col" class="px-6 py-3">持续时间</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="alert in activeAlerts"
                  :key="alert.id"
                  class="border-b hover:bg-muted/50"
                >
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-destructive/10 flex items-center justify-center"
                      >
                        <AlertTriangle class="w-4 h-4 text-destructive" />
                      </div>
                      <div>
                        <div class="font-medium">{{ alert.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ alert.description }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getSeverityVariant(alert.severity)">
                      {{ alert.severity }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ alert.source }}</td>
                  <td class="px-6 py-4">
                    <UiBadge variant="destructive">未处理</UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ alert.occurredAt }}</td>
                  <td class="px-6 py-4">{{ alert.duration }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 告警来源分布 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>告警来源分布</UiCardTitle>
          <UiCardDescription>各系统组件产生的告警数量</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <Server class="w-8 h-8 text-chart-3 mx-auto mb-2" />
              <p class="text-2xl font-bold">8</p>
              <p class="text-sm text-muted-foreground">服务器</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Database class="w-8 h-8 text-chart-2 mx-auto mb-2" />
              <p class="text-2xl font-bold">5</p>
              <p class="text-sm text-muted-foreground">数据库</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <Network class="w-8 h-8 text-chart-5 mx-auto mb-2" />
              <p class="text-2xl font-bold">3</p>
              <p class="text-sm text-muted-foreground">网络</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <HardDrive class="w-8 h-8 text-chart-4 mx-auto mb-2" />
              <p class="text-2xl font-bold">7</p>
              <p class="text-sm text-muted-foreground">存储</p>
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
  RefreshCw,
  Filter,
  AlertTriangle,
  TrendingUp,
  Clock,
  CheckCircle,
  Server,
  Database,
  Network,
  HardDrive,
} from 'lucide-vue-next'

const alertTrends = ref([
  { day: '周一', critical: 3, warning: 5, info: 8 },
  { day: '周二', critical: 2, warning: 4, info: 6 },
  { day: '周三', critical: 5, warning: 7, info: 10 },
  { day: '周四', critical: 4, warning: 6, info: 9 },
  { day: '周五', critical: 6, warning: 8, info: 12 },
  { day: '周六', critical: 3, warning: 5, info: 7 },
  { day: '周日', critical: 2, warning: 3, info: 5 },
])

const activeAlerts = ref([
  {
    id: 1,
    name: 'CPU使用率过高',
    description: '服务器 DB-01 CPU使用率超过90%',
    severity: '严重',
    source: 'DB-01',
    occurredAt: '2024-01-15 14:30:25',
    duration: '2小时15分钟',
  },
  {
    id: 2,
    name: '内存不足',
    description: '服务器 Web-02 内存使用率超过85%',
    severity: '警告',
    source: 'Web-02',
    occurredAt: '2024-01-15 13:45:10',
    duration: '3小时',
  },
  {
    id: 3,
    name: '磁盘空间不足',
    description: '/var分区使用率超过95%',
    severity: '严重',
    source: 'File-01',
    occurredAt: '2024-01-15 12:20:35',
    duration: '4小时25分钟',
  },
  {
    id: 4,
    name: '服务不可用',
    description: 'Nginx服务停止运行',
    severity: '严重',
    source: 'Web-01',
    occurredAt: '2024-01-15 11:15:40',
    duration: '5小时15分钟',
  },
  {
    id: 5,
    name: '网络延迟',
    description: '网络延迟超过200ms',
    severity: '提醒',
    source: '网络监控',
    occurredAt: '2024-01-15 10:30:20',
    duration: '6小时',
  },
])

function getSeverityVariant(severity: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    严重: 'destructive',
    警告: 'secondary',
    提醒: 'default',
  }
  return variants[severity] || 'outline'
}
</script>

<style scoped></style>
