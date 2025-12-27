<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="数据分析" description="系统数据统计和分析报告" sticky>
    <div class="max-w-6xl mx-auto space-y-8">
      <!-- 数据概览 -->
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总访问量</UiCardTitle>
            <Eye class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">12.4K</div>
            <p class="text-xs text-muted-foreground">+18% 较上月</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">活跃用户</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">892</div>
            <p class="text-xs text-muted-foreground">+8% 较上月</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">转化率</UiCardTitle>
            <TrendingUp class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">24.3%</div>
            <p class="text-xs text-muted-foreground">+3.2% 较上月</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">平均停留</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">3.2m</div>
            <p class="text-xs text-muted-foreground">+45s 较上月</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 图表区域 -->
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-7">
        <UiCard class="col-span-4">
          <UiCardHeader>
            <UiCardTitle>访问趋势</UiCardTitle>
            <UiCardDescription> 最近30天的访问量变化趋势 </UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="pl-2">
            <div class="h-[300px] flex items-center justify-center">
              <div class="text-center">
                <BarChart3 class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
                <p class="text-muted-foreground">访问趋势图表</p>
                <p class="text-sm text-muted-foreground">这里将展示详细的访问数据图表</p>
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard class="col-span-3">
          <UiCardHeader>
            <UiCardTitle>用户来源</UiCardTitle>
            <UiCardDescription> 用户访问来源分布 </UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
                  <span class="text-sm">直接访问</span>
                </div>
                <span class="text-sm font-medium">42%</span>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                  <span class="text-sm">搜索引擎</span>
                </div>
                <span class="text-sm font-medium">35%</span>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <div class="w-3 h-3 bg-purple-500 rounded-full"></div>
                  <span class="text-sm">社交媒体</span>
                </div>
                <span class="text-sm font-medium">15%</span>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <div class="w-3 h-3 bg-orange-500 rounded-full"></div>
                  <span class="text-sm">其他来源</span>
                </div>
                <span class="text-sm font-medium">8%</span>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 数据报告 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>数据报告</UiCardTitle>
          <UiCardDescription> 详细的数据分析和洞察报告 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex items-center justify-between p-4 border rounded-lg">
              <div class="space-y-1">
                <p class="text-sm font-medium">月度用户行为报告</p>
                <p class="text-sm text-muted-foreground">用户活跃度和行为模式分析</p>
              </div>
              <UiButton variant="outline" size="sm"> 查看报告 </UiButton>
            </div>
            <div class="flex items-center justify-between p-4 border rounded-lg">
              <div class="space-y-1">
                <p class="text-sm font-medium">转化率优化分析</p>
                <p class="text-sm text-muted-foreground">用户转化路径和优化建议</p>
              </div>
              <UiButton variant="outline" size="sm"> 查看报告 </UiButton>
            </div>
            <div class="flex items-center justify-between p-4 border rounded-lg">
              <div class="space-y-1">
                <p class="text-sm font-medium">性能监控报告</p>
                <p class="text-sm text-muted-foreground">系统性能和用户体验指标</p>
              </div>
              <UiButton variant="outline" size="sm"> 查看报告 </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script setup lang="ts">
import { BarChart3, Clock, Eye, TrendingUp, Users } from 'lucide-vue-next'
import Page from '@/components/global-layout/basic-page.vue'
</script>

<route lang="yaml">
meta:
  auth: true
</route>
