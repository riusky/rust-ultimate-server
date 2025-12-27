<template>
  <Page title="性能监控" description="系统性能指标监控和分析" sticky>
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 性能概览 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>性能概览</UiCardTitle>
          <UiCardDescription>系统整体性能指标</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-2">98.5%</p>
              <p class="text-sm text-muted-foreground">系统可用性</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-3">45ms</p>
              <p class="text-sm text-muted-foreground">平均响应时间</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-4">1.2k</p>
              <p class="text-sm text-muted-foreground">QPS</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-5">99.9%</p>
              <p class="text-sm text-muted-foreground">成功率</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 响应时间趋势 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>响应时间趋势</UiCardTitle>
          <UiCardDescription>最近24小时响应时间变化</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-48 flex items-end justify-between space-x-1">
            <div
              v-for="(item, index) in responseTimes"
              :key="index"
              class="flex flex-col items-center"
            >
              <div
                class="w-6 bg-chart-3 rounded-t"
                :style="{ height: `${item.value * 2}px` }"
              ></div>
              <span class="text-xs mt-1">{{ item.time }}</span>
            </div>
          </div>
          <div class="mt-4 grid grid-cols-3 gap-4 text-sm">
            <div>
              <p class="text-muted-foreground">最小</p>
              <p class="font-medium">32ms</p>
            </div>
            <div>
              <p class="text-muted-foreground">平均</p>
              <p class="font-medium">45ms</p>
            </div>
            <div>
              <p class="text-muted-foreground">最大</p>
              <p class="font-medium">89ms</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 吞吐量监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>吞吐量监控</UiCardTitle>
          <UiCardDescription>请求处理能力指标</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-48 flex items-end justify-between space-x-1">
            <div
              v-for="(item, index) in throughput"
              :key="index"
              class="flex flex-col items-center"
            >
              <div
                class="w-6 bg-chart-2 rounded-t"
                :style="{ height: `${item.value / 50}px` }"
              ></div>
              <span class="text-xs mt-1">{{ item.time }}</span>
            </div>
          </div>
          <div class="mt-4 grid grid-cols-3 gap-4 text-sm">
            <div>
              <p class="text-muted-foreground">最小</p>
              <p class="font-medium">850</p>
            </div>
            <div>
              <p class="text-muted-foreground">平均</p>
              <p class="font-medium">1.2k</p>
            </div>
            <div>
              <p class="text-muted-foreground">最大</p>
              <p class="font-medium">1.8k</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 错误率监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>错误率监控</UiCardTitle>
          <UiCardDescription>系统错误和异常统计</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-48 flex items-end justify-between space-x-1">
            <div
              v-for="(item, index) in errorRates"
              :key="index"
              class="flex flex-col items-center"
            >
              <div
                class="w-6 bg-destructive rounded-t"
                :style="{ height: `${item.value * 100}px` }"
              ></div>
              <span class="text-xs mt-1">{{ item.time }}</span>
            </div>
          </div>
          <div class="mt-4 grid grid-cols-3 gap-4 text-sm">
            <div>
              <p class="text-muted-foreground">5xx错误</p>
              <p class="font-medium">12</p>
            </div>
            <div>
              <p class="text-muted-foreground">4xx错误</p>
              <p class="font-medium">45</p>
            </div>
            <div>
              <p class="text-muted-foreground">成功率</p>
              <p class="font-medium">99.9%</p>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 资源使用率 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>资源使用率</UiCardTitle>
          <UiCardDescription>系统资源使用情况</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm">CPU使用率</span>
              <span class="text-sm font-medium">32%</span>
            </div>
            <div class="h-2 bg-muted rounded-full">
              <div class="h-full bg-chart-3 rounded-full" style="width: 32%"></div>
            </div>

            <div class="flex justify-between items-center">
              <span class="text-sm">内存使用率</span>
              <span class="text-sm font-medium">40%</span>
            </div>
            <div class="h-2 bg-muted rounded-full">
              <div class="h-full bg-chart-2 rounded-full" style="width: 40%"></div>
            </div>

            <div class="flex justify-between items-center">
              <span class="text-sm">磁盘I/O</span>
              <span class="text-sm font-medium">15%</span>
            </div>
            <div class="h-2 bg-muted rounded-full">
              <div class="h-full bg-chart-4 rounded-full" style="width: 15%"></div>
            </div>

            <div class="flex justify-between items-center">
              <span class="text-sm">网络I/O</span>
              <span class="text-sm font-medium">28%</span>
            </div>
            <div class="h-2 bg-muted rounded-full">
              <div class="h-full bg-chart-5 rounded-full" style="width: 28%"></div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 性能告警 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>性能告警</UiCardTitle>
          <UiCardDescription>性能相关告警信息</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              class="flex items-start space-x-3 p-3 border border-chart-3/20 bg-chart-3/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">响应时间异常</p>
                <p class="text-sm text-muted-foreground">API响应时间超过阈值</p>
                <p class="text-xs text-muted-foreground">10分钟前</p>
              </div>
              <UiBadge variant="secondary">警告</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-3 border border-destructive/20 bg-destructive/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">内存使用过高</p>
                <p class="text-sm text-muted-foreground">内存使用率超过85%</p>
                <p class="text-xs text-muted-foreground">30分钟前</p>
              </div>
              <UiBadge variant="destructive">严重</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-3 border border-chart-3/20 bg-chart-3/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">磁盘空间不足</p>
                <p class="text-sm text-muted-foreground">/var分区使用率90%</p>
                <p class="text-xs text-muted-foreground">1小时前</p>
              </div>
              <UiBadge variant="secondary">提醒</UiBadge>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'

const responseTimes = ref([
  { time: '00:00', value: 42 },
  { time: '04:00', value: 38 },
  { time: '08:00', value: 65 },
  { time: '12:00', value: 89 },
  { time: '16:00', value: 72 },
  { time: '20:00', value: 48 },
  { time: '24:00', value: 35 },
])

const throughput = ref([
  { time: '00:00', value: 850 },
  { time: '04:00', value: 920 },
  { time: '08:00', value: 1500 },
  { time: '12:00', value: 1800 },
  { time: '16:00', value: 1650 },
  { time: '20:00', value: 1200 },
  { time: '24:00', value: 950 },
])

const errorRates = ref([
  { time: '00:00', value: 0.001 },
  { time: '04:00', value: 0.0005 },
  { time: '08:00', value: 0.002 },
  { time: '12:00', value: 0.003 },
  { time: '16:00', value: 0.0015 },
  { time: '20:00', value: 0.001 },
  { time: '24:00', value: 0.0008 },
])
</script>

<style scoped></style>
