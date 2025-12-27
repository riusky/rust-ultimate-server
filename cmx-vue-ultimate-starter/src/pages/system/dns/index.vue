<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const dnsConfig = ref({
  primaryDns: '8.8.8.8',
  secondaryDns: '8.8.4.4',
  cacheSize: 512,
  forwarders: ['8.8.8.8', '1.1.1.1'],
  recursion: true,
  dnssec: true,
  queryLogging: false,
})

const dnsRecords = ref([
  {
    id: 1,
    name: 'example.com',
    type: 'A',
    value: '192.168.1.101',
    ttl: 3600,
    enabled: true,
  },
  {
    id: 2,
    name: 'www.example.com',
    type: 'CNAME',
    value: 'example.com',
    ttl: 3600,
    enabled: true,
  },
  {
    id: 3,
    name: 'mail.example.com',
    type: 'A',
    value: '192.168.1.102',
    ttl: 3600,
    enabled: true,
  },
  {
    id: 4,
    name: 'example.com',
    type: 'MX',
    value: '10 mail.example.com',
    ttl: 3600,
    enabled: true,
  },
  {
    id: 5,
    name: '_dmarc.example.com',
    type: 'TXT',
    value: 'v=DMARC1; p=none',
    ttl: 3600,
    enabled: false,
  },
])

const dnsStats = ref({
  queries: 12450,
  cacheHits: 8920,
  cacheMisses: 3530,
  responseTime: 45,
})

const queryLogs = ref([
  {
    id: 1,
    timestamp: '2024-01-15 14:30:22',
    client: '192.168.1.50',
    query: 'www.example.com',
    type: 'A',
    response: '192.168.1.101',
    responseTime: 23,
  },
  {
    id: 2,
    timestamp: '2024-01-15 14:29:15',
    client: '192.168.1.51',
    query: 'mail.google.com',
    type: 'A',
    response: '142.250.191.78',
    responseTime: 67,
  },
  {
    id: 3,
    timestamp: '2024-01-15 14:28:33',
    client: '192.168.1.52',
    query: 'api.example.com',
    type: 'A',
    response: 'NXDOMAIN',
    responseTime: 12,
  },
])

const saveConfig = () => {
  console.log('保存DNS配置:', dnsConfig.value)
}

const restartService = () => {
  console.log('重启DNS服务')
}

const flushCache = () => {
  console.log('清空DNS缓存')
}

const addRecord = () => {
  console.log('添加DNS记录')
}

const editRecord = (recordId: number) => {
  console.log('编辑DNS记录:', recordId)
}

const deleteRecord = (recordId: number) => {
  console.log('删除DNS记录:', recordId)
}

const getRecordTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    A: 'success',
    CNAME: 'warning',
    MX: 'chart-1',
    TXT: 'chart-2',
    NS: 'chart-3',
  }
  return colors[type] || 'default'
}
</script>

<template>
  <Page
    :title="transformI18n('system.dns.title')"
    :description="transformI18n('system.dns.description')"
    sticky
  >
    <!-- DNS统计概览 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总查询数</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ dnsStats.queries.toLocaleString() }}</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">缓存命中率</UiCardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">
            {{ Math.round((dnsStats.cacheHits / dnsStats.queries) * 100) }}%
          </div>
          <p class="text-xs text-muted-foreground">
            命中: {{ dnsStats.cacheHits.toLocaleString() }}
          </p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">平均响应时间</UiCardTitle>
          <Clock class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ dnsStats.responseTime }}ms</div>
          <p class="text-xs text-muted-foreground">较上周 -5ms</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">缓存未命中</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ dnsStats.cacheMisses.toLocaleString() }}</div>
          <p class="text-xs text-muted-foreground">需要外部查询</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- DNS配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>DNS配置</UiCardTitle>
          <UiCardDescription>DNS服务器基本设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <UiLabel for="primary-dns">主DNS服务器</UiLabel>
                <UiInput id="primary-dns" v-model="dnsConfig.primaryDns" />
              </div>
              <div>
                <UiLabel for="secondary-dns">备用DNS服务器</UiLabel>
                <UiInput id="secondary-dns" v-model="dnsConfig.secondaryDns" />
              </div>
              <div>
                <UiLabel for="cache-size">缓存大小 (MB)</UiLabel>
                <UiInput id="cache-size" v-model="dnsConfig.cacheSize" type="number" />
              </div>
            </div>
            <div>
              <UiLabel for="forwarders">转发器</UiLabel>
              <UiInput id="forwarders" :value="dnsConfig.forwarders.join(', ')" />
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="recursion" v-model="dnsConfig.recursion" />
                <UiLabel for="recursion">启用递归查询</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="dnssec" v-model="dnsConfig.dnssec" />
                <UiLabel for="dnssec">启用DNSSEC验证</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="query-logging" v-model="dnsConfig.queryLogging" />
                <UiLabel for="query-logging">启用查询日志</UiLabel>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- DNS记录管理 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>DNS记录</UiCardTitle>
          <UiCardDescription>域名解析记录管理</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div v-for="record in dnsRecords" :key="record.id" class="p-3 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center space-x-2">
                  <h3 class="font-semibold">{{ record.name }}</h3>
                  <UiBadge :variant="getRecordTypeColor(record.type) as any">
                    {{ record.type }}
                  </UiBadge>
                  <UiBadge v-if="!record.enabled" variant="outline"> 禁用 </UiBadge>
                </div>
              </div>
              <p class="text-sm text-muted-foreground mb-1">{{ record.value }}</p>
              <div class="flex justify-between text-xs text-muted-foreground">
                <span>TTL: {{ record.ttl }}</span>
                <div class="flex space-x-1">
                  <UiButton variant="ghost" size="sm" @click="editRecord(record.id)">
                    <Edit class="h-3 w-3" />
                  </UiButton>
                  <UiButton variant="ghost" size="sm" @click="deleteRecord(record.id)">
                    <Trash2 class="h-3 w-3" />
                  </UiButton>
                </div>
              </div>
            </div>
          </div>
          <div class="mt-4">
            <UiButton class="w-full" @click="addRecord">
              <Plus class="h-4 w-4 mr-2" />
              添加记录
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 查询日志 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>查询日志</UiCardTitle>
          <UiCardDescription>最近的DNS查询记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="log in queryLogs"
              :key="log.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="text-sm font-medium">{{ log.client }}</span>
                    <span class="text-sm text-muted-foreground">→</span>
                    <span class="text-sm font-medium">{{ log.query }}</span>
                    <UiBadge variant="outline">{{ log.type }}</UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ log.timestamp }} • 响应: {{ log.response }} • 时间: {{ log.responseTime }}ms
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="flushCache">
          <RefreshCw class="h-4 w-4 mr-2" />
          清空缓存
        </UiButton>
        <UiButton variant="outline" @click="restartService">
          <Power class="h-4 w-4 mr-2" />
          重启服务
        </UiButton>
        <UiButton @click="saveConfig">
          <Save class="h-4 w-4 mr-2" />
          保存配置
        </UiButton>
      </div>
    </div>
  </Page>
</template>

<style></style>
