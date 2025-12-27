<template>
  <Page title="系统监控" description="实时监控系统运行状态和性能指标" sticky>
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- CPU 监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>CPU 使用率</UiCardTitle>
          <UiCardDescription>实时 CPU 使用情况监控</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm font-medium">当前使用率</span>
              <span class="text-2xl font-bold text-chart-2">32%</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div class="h-full bg-chart-2" style="width: 32%"></div>
            </div>
            <div class="grid grid-cols-3 gap-4 text-sm">
              <div>
                <p class="text-muted-foreground">用户进程</p>
                <p class="font-medium">25%</p>
              </div>
              <div>
                <p class="text-muted-foreground">系统进程</p>
                <p class="font-medium">7%</p>
              </div>
              <div>
                <p class="text-muted-foreground">空闲</p>
                <p class="font-medium">68%</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 内存监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>内存使用</UiCardTitle>
          <UiCardDescription>内存使用情况监控</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm font-medium">已使用</span>
              <span class="text-2xl font-bold text-chart-3">3.2GB / 8GB</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div class="h-full bg-chart-3" style="width: 40%"></div>
            </div>
            <div class="grid grid-cols-3 gap-4 text-sm">
              <div>
                <p class="text-muted-foreground">应用程序</p>
                <p class="font-medium">2.1GB</p>
              </div>
              <div>
                <p class="text-muted-foreground">缓存</p>
                <p class="font-medium">0.8GB</p>
              </div>
              <div>
                <p class="text-muted-foreground">可用</p>
                <p class="font-medium">4.8GB</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 磁盘监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>磁盘使用</UiCardTitle>
          <UiCardDescription>磁盘空间使用情况</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm font-medium">根分区</span>
              <span class="text-2xl font-bold text-chart-4">128GB / 500GB</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div class="h-full bg-chart-4" style="width: 25%"></div>
            </div>
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-muted-foreground">已使用</p>
                <p class="font-medium">128GB</p>
              </div>
              <div>
                <p class="text-muted-foreground">可用</p>
                <p class="font-medium">372GB</p>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 网络监控 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>网络流量</UiCardTitle>
          <UiCardDescription>实时网络流量监控</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-sm font-medium">上行速度</p>
                <p class="text-2xl font-bold text-chart-5">1.2 MB/s</p>
              </div>
              <div>
                <p class="text-sm font-medium">下行速度</p>
                <p class="text-2xl font-bold text-chart-5">2.4 MB/s</p>
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex justify-between text-sm">
                <span>eth0</span>
                <span>连接正常</span>
              </div>
              <div class="flex justify-between text-sm">
                <span>lo</span>
                <span>本地回环</span>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 进程监控 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>进程监控</UiCardTitle>
          <UiCardDescription>系统进程运行状态</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-2">
            <div class="grid grid-cols-5 gap-4 p-2 bg-secondary rounded text-sm font-medium">
              <span>进程名</span>
              <span>PID</span>
              <span>CPU</span>
              <span>内存</span>
              <span>状态</span>
            </div>
            <div class="grid grid-cols-5 gap-4 p-2 border-b text-sm">
              <span>nginx</span>
              <span>1234</span>
              <span>2.1%</span>
              <span>45MB</span>
              <span><UiBadge variant="default">运行中</UiBadge></span>
            </div>
            <div class="grid grid-cols-5 gap-4 p-2 border-b text-sm">
              <span>mysql</span>
              <span>1235</span>
              <span>5.3%</span>
              <span>320MB</span>
              <span><UiBadge variant="default">运行中</UiBadge></span>
            </div>
            <div class="grid grid-cols-5 gap-4 p-2 border-b text-sm">
              <span>redis</span>
              <span>1236</span>
              <span>0.8%</span>
              <span>12MB</span>
              <span><UiBadge variant="default">运行中</UiBadge></span>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
</script>

<style scoped></style>
