<template>
  <Page
    :title="transformI18n('system.parameters.title')"
    :description="transformI18n('system.parameters.description')"
    sticky
  >
    <!-- 参数配置概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总参数数</UiCardTitle>
          <Settings class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">156</div>
          <p class="text-xs text-muted-foreground">系统参数</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">已修改</UiCardTitle>
          <Edit class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold text-warning">23</div>
          <p class="text-xs text-muted-foreground">自定义参数</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">默认值</UiCardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">133</div>
          <p class="text-xs text-muted-foreground">使用默认值</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">分类数</UiCardTitle>
          <Folder class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">8</div>
          <p class="text-xs text-muted-foreground">参数分类</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 参数列表 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>参数配置</UiCardTitle>
          <UiCardDescription>系统参数配置和管理</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              v-for="param in parameters"
              :key="param.id"
              class="flex items-center justify-between p-4 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ param.name }}</h3>
                    <UiBadge :variant="getParamTypeColor(param.type)">
                      {{ getParamTypeText(param.type) }}
                    </UiBadge>
                    <UiBadge v-if="param.modified" variant="destructive"> 已修改 </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">{{ param.description }}</p>
                  <div class="flex items-center space-x-4 mt-2 text-sm">
                    <span>默认值: {{ param.defaultValue }}</span>
                    <span>当前值: {{ param.currentValue }}</span>
                  </div>
                </div>
              </div>
              <div class="flex items-center space-x-2">
                <UiInput
                  v-model="param.currentValue"
                  class="w-32"
                  :placeholder="param.defaultValue"
                />
                <UiButton variant="outline" size="sm" @click="resetParam(param.id)">
                  <RefreshCw class="h-4 w-4" />
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 参数分类 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>参数分类</UiCardTitle>
          <UiCardDescription>按分类管理参数</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div v-for="category in categories" :key="category.id" class="p-3 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <h3 class="font-semibold">{{ category.name }}</h3>
                <UiBadge variant="outline">{{ category.count }} 参数</UiBadge>
              </div>
              <p class="text-sm text-muted-foreground">{{ category.description }}</p>
              <div class="flex justify-between text-xs text-muted-foreground mt-2">
                <span>{{ category.modified }} 已修改</span>
                <UiButton variant="ghost" size="sm">
                  <Settings class="h-3 w-3 mr-1" />
                  配置
                </UiButton>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 参数导入导出 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>导入导出</UiCardTitle>
          <UiCardDescription>参数配置的导入和导出</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <h3 class="font-semibold">导出配置</h3>
              <p class="text-sm text-muted-foreground">将当前参数配置导出为配置文件</p>
              <div class="space-y-2">
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="export-modified" v-model="exportConfig.onlyModified" />
                  <UiLabel for="export-modified">仅导出已修改参数</UiLabel>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="export-sensitive" v-model="exportConfig.includeSensitive" />
                  <UiLabel for="export-sensitive">包含敏感参数</UiLabel>
                </div>
              </div>
              <UiButton class="w-full" @click="exportParameters">
                <Download class="h-4 w-4 mr-2" />
                导出配置
              </UiButton>
            </div>

            <div class="space-y-4">
              <h3 class="font-semibold">导入配置</h3>
              <p class="text-sm text-muted-foreground">从配置文件导入参数配置</p>
              <div class="space-y-2">
                <UiInput type="file" @change="handleFileImport" />
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="import-overwrite" v-model="importConfig.overwrite" />
                  <UiLabel for="import-overwrite">覆盖现有配置</UiLabel>
                </div>
              </div>
              <UiButton class="w-full" variant="outline" @click="importParameters">
                <Upload class="h-4 w-4 mr-2" />
                导入配置
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="resetAll">
          <RefreshCw class="h-4 w-4 mr-2" />
          重置所有
        </UiButton>
        <UiButton @click="saveParameters">
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

const parameters = ref([
  {
    id: 1,
    name: 'session_timeout',
    type: 'system',
    description: '用户会话超时时间（分钟）',
    defaultValue: '30',
    currentValue: '45',
    modified: true,
  },
  {
    id: 2,
    name: 'max_login_attempts',
    type: 'security',
    description: '最大登录尝试次数',
    defaultValue: '5',
    currentValue: '5',
    modified: false,
  },
  {
    id: 3,
    name: 'backup_interval',
    type: 'backup',
    description: '自动备份间隔（小时）',
    defaultValue: '24',
    currentValue: '12',
    modified: true,
  },
  {
    id: 4,
    name: 'log_retention_days',
    type: 'logging',
    description: '日志保留天数',
    defaultValue: '90',
    currentValue: '90',
    modified: false,
  },
  {
    id: 5,
    name: 'cache_size',
    type: 'performance',
    description: '缓存大小（MB）',
    defaultValue: '512',
    currentValue: '1024',
    modified: true,
  },
  {
    id: 6,
    name: 'email_smtp_host',
    type: 'notification',
    description: '邮件服务器地址',
    defaultValue: 'smtp.example.com',
    currentValue: 'smtp.company.com',
    modified: true,
  },
])

const categories = ref([
  {
    id: 1,
    name: '系统参数',
    description: '核心系统运行参数',
    count: 45,
    modified: 8,
  },
  {
    id: 2,
    name: '安全参数',
    description: '安全相关配置参数',
    count: 32,
    modified: 5,
  },
  {
    id: 3,
    name: '性能参数',
    description: '系统性能调优参数',
    count: 28,
    modified: 6,
  },
  {
    id: 4,
    name: '备份参数',
    description: '数据备份和恢复参数',
    count: 15,
    modified: 3,
  },
  {
    id: 5,
    name: '日志参数',
    description: '日志记录和管理参数',
    count: 18,
    modified: 1,
  },
  {
    id: 6,
    name: '通知参数',
    description: '通知和告警参数',
    count: 18,
    modified: 0,
  },
])

const exportConfig = ref({
  onlyModified: false,
  includeSensitive: false,
})

const importConfig = ref({
  overwrite: false,
})

const getParamTypeColor = (type: string): 'default' | 'secondary' | 'destructive' | 'outline' => {
  const colors: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    system: 'destructive',
    security: 'destructive',
    performance: 'secondary',
    backup: 'secondary',
    logging: 'outline',
    notification: 'outline',
  }
  return colors[type] || 'default'
}

const getParamTypeText = (type: string) => {
  const texts: Record<string, string> = {
    system: '系统',
    security: '安全',
    performance: '性能',
    backup: '备份',
    logging: '日志',
    notification: '通知',
  }
  return texts[type] || type
}

const resetParam = (paramId: number) => {
  const param = parameters.value.find((p) => p.id === paramId)
  if (param) {
    param.currentValue = param.defaultValue
    param.modified = false
  }
}

const resetAll = () => {
  parameters.value.forEach((param) => {
    param.currentValue = param.defaultValue
    param.modified = false
  })
}

const saveParameters = () => {
  console.log('保存参数配置')
}

const exportParameters = () => {
  console.log('导出参数配置')
}

const handleFileImport = (event: Event) => {
  console.log('处理文件导入', event)
}

const importParameters = () => {
  console.log('导入参数配置')
}
</script>

<style></style>
