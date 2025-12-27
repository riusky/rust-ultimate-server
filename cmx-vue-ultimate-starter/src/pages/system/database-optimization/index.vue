<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const performanceStats = ref({
  queryTime: 125,
  connections: 45,
  cacheHitRate: 92,
  slowQueries: 8,
})

const optimizationSuggestions = ref([
  {
    id: 1,
    title: '索引优化',
    description: '为 user 表的 email 字段添加索引',
    impact: 'high',
    estimatedTime: '5分钟',
    status: 'pending',
  },
  {
    id: 2,
    title: '查询缓存',
    description: '启用查询缓存以提高重复查询性能',
    impact: 'medium',
    estimatedTime: '2分钟',
    status: 'completed',
  },
  {
    id: 3,
    title: '表优化',
    description: '对 orders 表进行碎片整理',
    impact: 'medium',
    estimatedTime: '10分钟',
    status: 'pending',
  },
  {
    id: 4,
    title: '连接池',
    description: '调整最大连接数设置',
    impact: 'low',
    estimatedTime: '1分钟',
    status: 'pending',
  },
])

const slowQueries = ref([
  {
    id: 1,
    query: 'SELECT * FROM users WHERE created_at > NOW() - INTERVAL 30 DAY',
    executionTime: 2.8,
    rowsExamined: 125000,
    rowsReturned: 1500,
    timestamp: '2024-01-15 14:30:22',
  },
  {
    id: 2,
    query: 'SELECT COUNT(*) FROM orders WHERE status = "pending"',
    executionTime: 1.5,
    rowsExamined: 85000,
    rowsReturned: 1,
    timestamp: '2024-01-15 14:25:10',
  },
  {
    id: 3,
    query: 'UPDATE products SET stock = stock - 1 WHERE id IN (SELECT product_id FROM order_items)',
    executionTime: 3.2,
    rowsExamined: 45000,
    rowsReturned: 0,
    timestamp: '2024-01-15 14:20:45',
  },
])

const databaseSettings = ref({
  queryCacheSize: 64,
  maxConnections: 100,
  innodbBufferPoolSize: 1024,
  keyBufferSize: 256,
  sortBufferSize: 2,
  readBufferSize: 128,
  tmpTableSize: 64,
})

const runOptimization = (suggestionId: number) => {
  console.log('运行优化建议:', suggestionId)
}

const analyzePerformance = () => {
  console.log('分析数据库性能')
}

const getImpactColor = (impact: string) => {
  const colors: Record<string, string> = {
    high: 'destructive',
    medium: 'warning',
    low: 'success',
  }
  return colors[impact] || 'default'
}

const getImpactText = (impact: string) => {
  const texts: Record<string, string> = {
    high: '高影响',
    medium: '中影响',
    low: '低影响',
  }
  return texts[impact] || impact
}
</script>

<template>
  <Page
    :title="transformI18n('system.database-optimization.title')"
    :description="transformI18n('system.database-optimization.description')"
    sticky
  >
    <!-- 性能概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">平均查询时间</UiCardTitle>
          <Clock class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ performanceStats.queryTime }}ms</div>
          <p class="text-xs text-muted-foreground">较上周 -15ms</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃连接</UiCardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ performanceStats.connections }}</div>
          <p class="text-xs text-muted-foreground">最大: 100</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">缓存命中率</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ performanceStats.cacheHitRate }}%</div>
          <p class="text-xs text-muted-foreground">目标: 95%</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">慢查询</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ performanceStats.slowQueries }}</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 优化建议 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>优化建议</UiCardTitle>
          <UiCardDescription>基于性能分析的建议改进措施</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="suggestion in optimizationSuggestions"
              :key="suggestion.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ suggestion.title }}</h3>
                    <UiBadge :variant="getImpactColor(suggestion.impact) as any">
                      {{ getImpactText(suggestion.impact) }}
                    </UiBadge>
                    <UiBadge v-if="suggestion.status === 'completed'" variant="secondary">
                      已完成
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ suggestion.description }}</p>
                  <p class="text-xs text-muted-foreground">
                    预计时间: {{ suggestion.estimatedTime }}
                  </p>
                </div>
              </div>
              <UiButton
                v-if="suggestion.status === 'pending'"
                variant="outline"
                size="sm"
                @click="runOptimization(suggestion.id)"
              >
                <Play class="h-4 w-4 mr-1" />
                执行
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 数据库设置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>数据库设置</UiCardTitle>
          <UiCardDescription>关键性能参数配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="query-cache">查询缓存大小 (MB)</UiLabel>
              <UiInput id="query-cache" v-model="databaseSettings.queryCacheSize" type="number" />
            </div>
            <div>
              <UiLabel for="max-connections">最大连接数</UiLabel>
              <UiInput
                id="max-connections"
                v-model="databaseSettings.maxConnections"
                type="number"
              />
            </div>
            <div>
              <UiLabel for="buffer-pool">InnoDB 缓冲池 (MB)</UiLabel>
              <UiInput
                id="buffer-pool"
                v-model="databaseSettings.innodbBufferPoolSize"
                type="number"
              />
            </div>
            <div>
              <UiLabel for="key-buffer">键缓冲区 (MB)</UiLabel>
              <UiInput id="key-buffer" v-model="databaseSettings.keyBufferSize" type="number" />
            </div>
            <div class="pt-4">
              <UiButton class="w-full" @click="analyzePerformance">
                <RefreshCw class="h-4 w-4 mr-2" />
                重新分析性能
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 慢查询分析 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>慢查询分析</UiCardTitle>
          <UiCardDescription>执行时间超过 1 秒的查询</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div v-for="query in slowQueries" :key="query.id" class="p-4 rounded-lg border">
              <div class="flex items-start justify-between mb-2">
                <div>
                  <code class="text-sm bg-muted p-2 rounded block">{{ query.query }}</code>
                </div>
                <div class="text-right">
                  <div class="text-lg font-bold text-destructive">{{ query.executionTime }}s</div>
                  <p class="text-sm text-muted-foreground">执行时间</p>
                </div>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                <div>
                  <span class="font-medium">扫描行数:</span>
                  {{ query.rowsExamined.toLocaleString() }}
                </div>
                <div>
                  <span class="font-medium">返回行数:</span>
                  {{ query.rowsReturned.toLocaleString() }}
                </div>
                <div><span class="font-medium">时间:</span> {{ query.timestamp }}</div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<style></style>
