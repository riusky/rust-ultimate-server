<template>
  <Page title="容量规划" description="系统容量规划和预测分析" sticky>
    <template #actions>
      <UiButton>
        <Download class="w-4 h-4 mr-2" />
        导出报告
      </UiButton>
      <UiButton variant="outline">
        <Settings class="w-4 h-4 mr-2" />
        配置
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 容量概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">当前使用率</UiCardTitle>
            <Activity class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">65%</div>
            <p class="text-xs text-muted-foreground">整体资源使用率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">预计耗尽时间</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">180天</div>
            <p class="text-xs text-muted-foreground">基于当前增长率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">月增长率</UiCardTitle>
            <TrendingUp class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">8.5%</div>
            <p class="text-xs text-muted-foreground">资源使用增长率</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">推荐扩容</UiCardTitle>
            <Plus class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-5">+50%</div>
            <p class="text-xs text-muted-foreground">建议扩容比例</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 容量预测 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>容量预测</UiCardTitle>
          <UiCardDescription>未来12个月资源使用预测</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="h-64 flex items-end justify-between space-x-2">
            <div
              v-for="(item, index) in capacityForecast"
              :key="index"
              class="flex flex-col items-center flex-1"
            >
              <div class="flex flex-col items-center space-y-1 flex-1 w-full">
                <div
                  class="w-full bg-chart-3 rounded-t"
                  :style="{ height: `${item.current}px` }"
                ></div>
                <div
                  class="w-full bg-chart-3/30 rounded-t border border-chart-3/50"
                  :style="{ height: `${item.forecast}px` }"
                ></div>
              </div>
              <span class="text-xs mt-2">{{ item.month }}</span>
            </div>
          </div>
          <div class="flex justify-center space-x-6 mt-4 text-sm">
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-3 rounded"></div>
              <span>当前使用</span>
            </div>
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 bg-chart-3/30 border border-chart-3/50 rounded"></div>
              <span>预测使用</span>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 资源类型分析 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>资源类型分析</UiCardTitle>
          <UiCardDescription>各类型资源使用情况和预测</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="border rounded-lg p-4 space-y-3">
              <div class="flex items-center space-x-2">
                <Cpu class="w-5 h-5 text-chart-3" />
                <h3 class="font-medium">CPU资源</h3>
              </div>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>当前使用</span>
                  <span class="font-medium">32%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>预测峰值</span>
                  <span class="font-medium text-chart-4">68%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>耗尽时间</span>
                  <span class="font-medium">240天</span>
                </div>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-3">
              <div class="flex items-center space-x-2">
                <HardDrive class="w-5 h-5 text-chart-2" />
                <h3 class="font-medium">内存资源</h3>
              </div>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>当前使用</span>
                  <span class="font-medium">40%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>预测峰值</span>
                  <span class="font-medium text-chart-4">85%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>耗尽时间</span>
                  <span class="font-medium">150天</span>
                </div>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-3">
              <div class="flex items-center space-x-2">
                <Database class="w-5 h-5 text-chart-4" />
                <h3 class="font-medium">存储资源</h3>
              </div>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>当前使用</span>
                  <span class="font-medium">25%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>预测峰值</span>
                  <span class="font-medium text-chart-4">45%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>耗尽时间</span>
                  <span class="font-medium">360天</span>
                </div>
              </div>
            </div>
            <div class="border rounded-lg p-4 space-y-3">
              <div class="flex items-center space-x-2">
                <Network class="w-5 h-5 text-chart-5" />
                <h3 class="font-medium">网络资源</h3>
              </div>
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>当前使用</span>
                  <span class="font-medium">28%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>预测峰值</span>
                  <span class="font-medium text-chart-4">52%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span>耗尽时间</span>
                  <span class="font-medium">280天</span>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 扩容建议 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>扩容建议</UiCardTitle>
          <UiCardDescription>基于容量预测的扩容方案</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="flex items-start space-x-3 p-4 border border-chart-3/20 bg-chart-3/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">内存扩容</p>
                <p class="text-sm text-muted-foreground">建议在90天内将内存容量增加50%</p>
                <p class="text-xs text-muted-foreground mt-1">优先级: 高</p>
              </div>
              <UiBadge variant="destructive">紧急</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-4/20 bg-chart-4/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">CPU扩容</p>
                <p class="text-sm text-muted-foreground">建议在180天内将CPU核心数增加30%</p>
                <p class="text-xs text-muted-foreground mt-1">优先级: 中</p>
              </div>
              <UiBadge variant="secondary">中等</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-2/20 bg-chart-2/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">存储扩容</p>
                <p class="text-sm text-muted-foreground">建议在240天内将存储容量增加40%</p>
                <p class="text-xs text-muted-foreground mt-1">优先级: 低</p>
              </div>
              <UiBadge variant="outline">一般</UiBadge>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 成本分析 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>成本分析</UiCardTitle>
          <UiCardDescription>扩容方案的成本估算</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-3">¥15,000</p>
              <p class="text-sm text-muted-foreground">硬件成本</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-4">¥5,000</p>
              <p class="text-sm text-muted-foreground">实施成本</p>
            </div>
            <div class="text-center p-4 border rounded-lg">
              <p class="text-2xl font-bold text-chart-2">¥20,000</p>
              <p class="text-sm text-muted-foreground">总成本</p>
            </div>
          </div>
          <div class="mt-4 text-sm text-muted-foreground">
            <p>注：以上成本估算基于当前市场价格，实际成本可能有所浮动。</p>
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
  Settings,
  Activity,
  Clock,
  TrendingUp,
  Plus,
  Cpu,
  HardDrive,
  Database,
  Network,
} from 'lucide-vue-next'

const capacityForecast = ref([
  { month: '1月', current: 40, forecast: 45 },
  { month: '2月', current: 42, forecast: 48 },
  { month: '3月', current: 45, forecast: 52 },
  { month: '4月', current: 48, forecast: 56 },
  { month: '5月', current: 52, forecast: 60 },
  { month: '6月', current: 55, forecast: 64 },
  { month: '7月', current: 58, forecast: 68 },
  { month: '8月', current: 62, forecast: 72 },
  { month: '9月', current: 65, forecast: 76 },
  { month: '10月', current: 68, forecast: 80 },
  { month: '11月', current: 72, forecast: 84 },
  { month: '12月', current: 75, forecast: 88 },
])
</script>

<style scoped></style>
