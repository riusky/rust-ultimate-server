<template>
  <Page
    :title="transformI18n('system.environment.title')"
    :description="transformI18n('system.environment.description')"
    sticky
  >
    <!-- 环境配置概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">环境数量</UiCardTitle>
          <Server class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">4</div>
          <p class="text-xs text-muted-foreground">配置环境</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃环境</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">3</div>
          <p class="text-xs text-muted-foreground">运行中</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">配置参数</UiCardTitle>
          <Settings class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">156</div>
          <p class="text-xs text-muted-foreground">环境变量</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">同步状态</UiCardTitle>
          <RefreshCw class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-success">100%</div>
          <p class="text-xs text-muted-foreground">配置同步</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 环境列表 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>环境配置</UiCardTitle>
          <UiCardDescription>多环境配置管理</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="env in environments"
              :key="env.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ env.name }}</h3>
                    <UiBadge :variant="getEnvStatusColor(env.status) as any">
                      {{ getEnvStatusText(env.status) }}
                    </UiBadge>
                    <UiBadge variant="outline">{{ env.type }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ env.description }}</p>
                  <div class="flex items-center space-x-4 mt-2 text-sm">
                    <span>主机: {{ env.host }}</span>
                    <span>端口: {{ env.port }}</span>
                    <span>数据库: {{ env.database }}</span>
                  </div>
                </div>
              </div>
              <div class="flex flex-col space-y-2">
                <UiButton variant="outline" size="sm" @click="editEnvironment(env.id)">
                  <Settings class="h-4 w-4 mr-1" />
                  配置
                </UiButton>
                <UiButton variant="outline" size="sm" @click="testConnection(env.id)">
                  <Wifi class="h-4 w-4 mr-1" />
                  测试
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 环境变量 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>环境变量</UiCardTitle>
          <UiCardDescription>全局环境变量配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="app-name">应用名称</UiLabel>
              <UiInput id="app-name" v-model="envVariables.appName" />
            </div>
            <div>
              <UiLabel for="app-version">应用版本</UiLabel>
              <UiInput id="app-version" v-model="envVariables.appVersion" />
            </div>
            <div>
              <UiLabel for="debug-mode">调试模式</UiLabel>
              <UiSelect v-model="envVariables.debugMode">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="true">启用</UiSelectItem>
                  <UiSelectItem value="false">禁用</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div>
              <UiLabel for="log-level">日志级别</UiLabel>
              <UiSelect v-model="envVariables.logLevel">
                <UiSelectTrigger>
                  <UiSelectValue />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="debug">调试</UiSelectItem>
                  <UiSelectItem value="info">信息</UiSelectItem>
                  <UiSelectItem value="warn">警告</UiSelectItem>
                  <UiSelectItem value="error">错误</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <div class="pt-4">
              <UiButton class="w-full" @click="syncEnvironments">
                <RefreshCw class="h-4 w-4 mr-2" />
                同步环境
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 配置同步 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>配置同步</UiCardTitle>
          <UiCardDescription>环境配置同步状态和日志</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="sync in syncLogs"
              :key="sync.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-3">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="text-sm font-medium">{{ sync.environment }}</span>
                    <span class="text-sm text-muted-foreground">→</span>
                    <span class="text-sm font-medium">{{ sync.action }}</span>
                    <UiBadge :variant="getSyncStatusColor(sync.status) as any">
                      {{ getSyncStatusText(sync.status) }}
                    </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ sync.timestamp }} • 用户: {{ sync.user }}
                  </p>
                  <p class="text-xs text-muted-foreground">
                    变更: {{ sync.changes }} • 耗时: {{ sync.duration }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="validateConfig">
          <CheckCircle class="h-4 w-4 mr-2" />
          验证配置
        </UiButton>
        <UiButton @click="saveConfig">
          <Save class="h-4 w-4 mr-2" />
          保存配置
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const environments = ref([
  {
    id: 1,
    name: '开发环境',
    status: 'active',
    type: 'development',
    description: '开发人员使用的测试环境',
    host: 'dev.example.com',
    port: '3306',
    database: 'app_dev',
  },
  {
    id: 2,
    name: '测试环境',
    status: 'active',
    type: 'testing',
    description: 'QA团队使用的测试环境',
    host: 'test.example.com',
    port: '3306',
    database: 'app_test',
  },
  {
    id: 3,
    name: '预生产环境',
    status: 'active',
    type: 'staging',
    description: '上线前的预发布环境',
    host: 'staging.example.com',
    port: '3306',
    database: 'app_staging',
  },
  {
    id: 4,
    name: '生产环境',
    status: 'inactive',
    type: 'production',
    description: '正式生产环境',
    host: 'prod.example.com',
    port: '3306',
    database: 'app_prod',
  },
])

const envVariables = ref({
  appName: 'Vue Ultimate Starter',
  appVersion: '1.0.0',
  debugMode: 'true',
  logLevel: 'info',
})

const syncLogs = ref([
  {
    id: 1,
    environment: '开发环境',
    action: '配置同步',
    status: 'success',
    timestamp: '2024-01-15 14:30:22',
    user: 'admin',
    changes: '12个参数',
    duration: '2.5秒',
  },
  {
    id: 2,
    environment: '测试环境',
    action: '配置同步',
    status: 'success',
    timestamp: '2024-01-15 14:28:15',
    user: 'admin',
    changes: '8个参数',
    duration: '1.8秒',
  },
  {
    id: 3,
    environment: '预生产环境',
    action: '配置同步',
    status: 'failed',
    timestamp: '2024-01-15 14:25:33',
    user: 'admin',
    changes: '0个参数',
    duration: '0.5秒',
  },
])

const getEnvStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    active: 'success',
    inactive: 'destructive',
    maintenance: 'warning',
  }
  return colors[status] || 'default'
}

const getEnvStatusText = (status: string) => {
  const texts: Record<string, string> = {
    active: '活跃',
    inactive: '停用',
    maintenance: '维护',
  }
  return texts[status] || status
}

const getSyncStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    success: 'success',
    failed: 'destructive',
    running: 'warning',
  }
  return colors[status] || 'default'
}

const getSyncStatusText = (status: string) => {
  const texts: Record<string, string> = {
    success: '成功',
    failed: '失败',
    running: '进行中',
  }
  return texts[status] || status
}

const editEnvironment = (envId: number) => {
  console.log('编辑环境配置:', envId)
}

const testConnection = (envId: number) => {
  console.log('测试环境连接:', envId)
}

const syncEnvironments = () => {
  console.log('同步环境配置')
}

const validateConfig = () => {
  console.log('验证环境配置')
}

const saveConfig = () => {
  console.log('保存环境配置')
}
</script>

<style></style>
