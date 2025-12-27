<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="图表统计" description="这是一个图表统计示例，展示各种数据可视化图表">
    <div class="max-w-7xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">图表统计</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          使用各种图表类型展示数据，支持交互式数据可视化
        </p>
      </div>

      <!-- 统计概览 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <UiCard v-for="stat in stats" :key="stat.title">
          <UiCardContent class="p-6">
            <div class="flex items-center justify-between">
              <div class="space-y-1">
                <p class="text-sm font-medium text-muted-foreground">{{ stat.title }}</p>
                <p class="text-2xl font-bold">{{ stat.value }}</p>
                <div
                  class="flex items-center gap-1 text-xs"
                  :class="stat.trend > 0 ? 'text-green-600' : 'text-red-600'"
                >
                  <UiTrendingUp v-if="stat.trend > 0" class="w-3 h-3" />
                  <UiTrendingDown v-else class="w-3 h-3" />
                  <span>{{ Math.abs(stat.trend) }}%</span>
                  <span class="text-muted-foreground">较上月</span>
                </div>
              </div>
              <div class="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
                <component :is="stat.icon" class="w-6 h-6 text-primary" />
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 图表区域 -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- 柱状图 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>销售趋势</UiCardTitle>
            <UiCardDescription>近6个月销售额变化趋势</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="h-80 flex items-center justify-center bg-muted/20 rounded-lg">
              <div class="text-center text-muted-foreground">
                <UiBarChart3 class="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p>柱状图区域</p>
                <p class="text-sm">这里将展示销售数据的柱状图</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <!-- 折线图 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>用户增长</UiCardTitle>
            <UiCardDescription>近12个月用户数量变化</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="h-80 flex items-center justify-center bg-muted/20 rounded-lg">
              <div class="text-center text-muted-foreground">
                <UiTrendingUp class="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p>折线图区域</p>
                <p class="text-sm">这里将展示用户增长的折线图</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <!-- 饼图 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>产品分类占比</UiCardTitle>
            <UiCardDescription>各产品类别的销售占比</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="h-80 flex items-center justify-center bg-muted/20 rounded-lg">
              <div class="text-center text-muted-foreground">
                <UiPieChart class="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p>饼图区域</p>
                <p class="text-sm">这里将展示产品分类的饼图</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <!-- 雷达图 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>技能评估</UiCardTitle>
            <UiCardDescription>团队成员技能分布</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="h-80 flex items-center justify-center bg-muted/20 rounded-lg">
              <div class="text-center text-muted-foreground">
                <UiRadar class="w-12 h-12 mx-auto mb-2 opacity-50" />
                <p>雷达图区域</p>
                <p class="text-sm">这里将展示技能评估的雷达图</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 数据表格 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>详细数据</UiCardTitle>
          <UiCardDescription>图表对应的详细数据表格</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="rounded-md border">
            <UiTable>
              <UiTableHeader>
                <UiTableRow>
                  <UiTableHead>月份</UiTableHead>
                  <UiTableHead>销售额</UiTableHead>
                  <UiTableHead>用户数</UiTableHead>
                  <UiTableHead>订单数</UiTableHead>
                  <UiTableHead>增长率</UiTableHead>
                </UiTableRow>
              </UiTableHeader>
              <UiTableBody>
                <UiTableRow v-for="row in tableData" :key="row.month">
                  <UiTableCell class="font-medium">{{ row.month }}</UiTableCell>
                  <UiTableCell>¥{{ row.sales.toLocaleString() }}</UiTableCell>
                  <UiTableCell>{{ row.users.toLocaleString() }}</UiTableCell>
                  <UiTableCell>{{ row.orders.toLocaleString() }}</UiTableCell>
                  <UiTableCell>
                    <span :class="row.growth >= 0 ? 'text-green-600' : 'text-red-600'">
                      {{ row.growth >= 0 ? '+' : '' }}{{ row.growth }}%
                    </span>
                  </UiTableCell>
                </UiTableRow>
              </UiTableBody>
            </UiTable>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 图表配置 -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>图表设置</UiCardTitle>
            <UiCardDescription>自定义图表显示选项</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="space-y-2">
              <UiLabel>图表类型</UiLabel>
              <UiSelect>
                <UiSelectTrigger>
                  <UiSelectValue placeholder="选择图表类型" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="bar">柱状图</UiSelectItem>
                  <UiSelectItem value="line">折线图</UiSelectItem>
                  <UiSelectItem value="pie">饼图</UiSelectItem>
                  <UiSelectItem value="radar">雷达图</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <UiLabel>时间范围</UiLabel>
              <UiSelect>
                <UiSelectTrigger>
                  <UiSelectValue placeholder="选择时间范围" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="7d">最近7天</UiSelectItem>
                  <UiSelectItem value="30d">最近30天</UiSelectItem>
                  <UiSelectItem value="90d">最近90天</UiSelectItem>
                  <UiSelectItem value="1y">最近1年</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="show-grid" />
              <UiLabel for="show-grid">显示网格线</UiLabel>
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="show-labels" />
              <UiLabel for="show-labels">显示数据标签</UiLabel>
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard>
          <UiCardHeader>
            <UiCardTitle>导出选项</UiCardTitle>
            <UiCardDescription>导出图表和数据</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="space-y-2">
              <UiLabel>导出格式</UiLabel>
              <UiSelect>
                <UiSelectTrigger>
                  <UiSelectValue placeholder="选择导出格式" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="png">PNG 图片</UiSelectItem>
                  <UiSelectItem value="svg">SVG 矢量图</UiSelectItem>
                  <UiSelectItem value="pdf">PDF 文档</UiSelectItem>
                  <UiSelectItem value="csv">CSV 数据</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="space-y-2">
              <UiLabel>图表尺寸</UiLabel>
              <UiSelect>
                <UiSelectTrigger>
                  <UiSelectValue placeholder="选择图表尺寸" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="small">小尺寸 (800×600)</UiSelectItem>
                  <UiSelectItem value="medium">中尺寸 (1200×800)</UiSelectItem>
                  <UiSelectItem value="large">大尺寸 (1920×1080)</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="flex gap-2 pt-2">
              <UiButton class="flex-1">
                <UiDownload class="w-4 h-4 mr-2" />
                导出图表
              </UiButton>
              <UiButton variant="outline">
                <UiShare class="w-4 h-4 mr-2" />
                分享
              </UiButton>
            </div>
          </UiCardContent>
        </UiCard>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import { Users, ShoppingCart, DollarSign, Activity } from 'lucide-vue-next'

// 统计数据
const stats = [
  {
    title: '总销售额',
    value: '¥245,678',
    trend: 12.5,
    icon: DollarSign,
  },
  {
    title: '总用户数',
    value: '12,458',
    trend: 8.2,
    icon: Users,
  },
  {
    title: '订单数量',
    value: '3,456',
    trend: -3.2,
    icon: ShoppingCart,
  },
  {
    title: '活跃用户',
    value: '8,932',
    trend: 15.7,
    icon: Activity,
  },
]

// 表格数据
const tableData = [
  {
    month: '2024-01',
    sales: 45678,
    users: 2345,
    orders: 1234,
    growth: 12.5,
  },
  {
    month: '2023-12',
    sales: 40567,
    users: 2189,
    orders: 1156,
    growth: 8.2,
  },
  {
    month: '2023-11',
    sales: 37456,
    users: 2023,
    orders: 1089,
    growth: 5.7,
  },
  {
    month: '2023-10',
    sales: 35432,
    users: 1912,
    orders: 987,
    growth: 3.2,
  },
  {
    month: '2023-09',
    sales: 34321,
    users: 1854,
    orders: 876,
    growth: -2.1,
  },
  {
    month: '2023-08',
    sales: 35067,
    users: 1893,
    orders: 912,
    growth: 4.5,
  },
]
</script>

<style scoped></style>
