<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const networkConfig = ref({
  hostname: 'server-01',
  domain: 'example.com',
  ipv4Address: '192.168.1.101',
  subnetMask: '255.255.255.0',
  gateway: '192.168.1.1',
  dnsServers: ['8.8.8.8', '8.8.4.4'],
  mtu: 1500,
  ipv6Enabled: false,
  ipv6Address: '2001:db8::1',
  dhcpEnabled: false,
})

const networkInterfaces = ref([
  {
    name: 'eth0',
    status: 'up',
    speed: '1000 Mbps',
    mac: '00:1B:44:11:3A:B7',
    ipv4: '192.168.1.101',
    ipv6: '2001:db8::1',
    rxBytes: '1.2 GB',
    txBytes: '0.8 GB',
  },
  {
    name: 'eth1',
    status: 'down',
    speed: '100 Mbps',
    mac: '00:1B:44:11:3A:B8',
    ipv4: '',
    ipv6: '',
    rxBytes: '0 B',
    txBytes: '0 B',
  },
  {
    name: 'lo',
    status: 'up',
    speed: 'N/A',
    mac: '00:00:00:00:00:00',
    ipv4: '127.0.0.1',
    ipv6: '::1',
    rxBytes: '45.2 MB',
    txBytes: '45.2 MB',
  },
])

const firewallRules = ref([
  {
    id: 1,
    name: 'SSH Access',
    protocol: 'TCP',
    port: '22',
    source: '0.0.0.0/0',
    action: 'allow',
    enabled: true,
  },
  {
    id: 2,
    name: 'HTTP Access',
    protocol: 'TCP',
    port: '80',
    source: '0.0.0.0/0',
    action: 'allow',
    enabled: true,
  },
  {
    id: 3,
    name: 'HTTPS Access',
    protocol: 'TCP',
    port: '443',
    source: '0.0.0.0/0',
    action: 'allow',
    enabled: true,
  },
  {
    id: 4,
    name: 'MySQL Block',
    protocol: 'TCP',
    port: '3306',
    source: '0.0.0.0/0',
    action: 'deny',
    enabled: true,
  },
])

const networkStats = ref({
  totalConnections: 245,
  activeConnections: 32,
  bandwidthUsage: '15.2 Mbps',
  packetLoss: '0.1%',
})

const saveConfig = () => {
  console.log('保存网络配置:', networkConfig.value)
}

const restartNetwork = () => {
  console.log('重启网络服务')
}

const testConnectivity = () => {
  console.log('测试网络连通性')
}

const getInterfaceStatusColor = (status: string) => {
  return status === 'up' ? 'success' : 'destructive'
}

const getInterfaceStatusText = (status: string) => {
  return status === 'up' ? '在线' : '离线'
}

const getFirewallActionColor = (action: string) => {
  return action === 'allow' ? 'success' : 'destructive'
}

const getFirewallActionText = (action: string) => {
  return action === 'allow' ? '允许' : '拒绝'
}
</script>

<template>
  <Page
    :title="transformI18n('system.network.title')"
    :description="transformI18n('system.network.description')"
    sticky
  >
    <!-- 网络统计 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">总连接数</UiCardTitle>
          <Network class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ networkStats.totalConnections }}</div>
          <p class="text-xs text-muted-foreground">今日统计</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">活跃连接</UiCardTitle>
          <Activity class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ networkStats.activeConnections }}</div>
          <p class="text-xs text-muted-foreground">实时</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">带宽使用</UiCardTitle>
          <TrendingUp class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ networkStats.bandwidthUsage }}</div>
          <p class="text-xs text-muted-foreground">当前速率</p>
        </UiCardContent>
      </UiCard>

      <UiCard class="p-4">
        <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <UiCardTitle class="text-sm font-medium">丢包率</UiCardTitle>
          <AlertTriangle class="h-4 w-4 text-muted-foreground" />
        </UiCardHeader>
        <UiCardContent>
          <div class="text-2xl font-bold">{{ networkStats.packetLoss }}</div>
          <p class="text-xs text-muted-foreground">最近1小时</p>
        </UiCardContent>
      </UiCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 基本网络配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>基本网络配置</UiCardTitle>
          <UiCardDescription>网络接口和协议设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <UiLabel for="hostname">主机名</UiLabel>
                <UiInput id="hostname" v-model="networkConfig.hostname" />
              </div>
              <div>
                <UiLabel for="domain">域名</UiLabel>
                <UiInput id="domain" v-model="networkConfig.domain" />
              </div>
              <div>
                <UiLabel for="ipv4">IPv4 地址</UiLabel>
                <UiInput id="ipv4" v-model="networkConfig.ipv4Address" />
              </div>
              <div>
                <UiLabel for="subnet">子网掩码</UiLabel>
                <UiInput id="subnet" v-model="networkConfig.subnetMask" />
              </div>
              <div>
                <UiLabel for="gateway">网关</UiLabel>
                <UiInput id="gateway" v-model="networkConfig.gateway" />
              </div>
              <div>
                <UiLabel for="mtu">MTU</UiLabel>
                <UiInput id="mtu" v-model="networkConfig.mtu" type="number" />
              </div>
            </div>
            <div>
              <UiLabel for="dns">DNS 服务器</UiLabel>
              <UiInput id="dns" :value="networkConfig.dnsServers.join(', ')" />
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="ipv6" v-model="networkConfig.ipv6Enabled" />
              <UiLabel for="ipv6">启用 IPv6</UiLabel>
            </div>
            <div v-if="networkConfig.ipv6Enabled">
              <UiLabel for="ipv6-address">IPv6 地址</UiLabel>
              <UiInput id="ipv6-address" v-model="networkConfig.ipv6Address" />
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="dhcp" v-model="networkConfig.dhcpEnabled" />
              <UiLabel for="dhcp">启用 DHCP</UiLabel>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 网络接口 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>网络接口</UiCardTitle>
          <UiCardDescription>系统网络接口状态</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div v-for="iface in networkInterfaces" :key="iface.name" class="p-3 rounded-lg border">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center space-x-2">
                  <h3 class="font-semibold">{{ iface.name }}</h3>
                  <UiBadge :variant="getInterfaceStatusColor(iface.status) as any">
                    {{ getInterfaceStatusText(iface.status) }}
                  </UiBadge>
                </div>
                <div class="text-sm text-muted-foreground">{{ iface.speed }}</div>
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm">
                <div><span class="font-medium">MAC:</span> {{ iface.mac }}</div>
                <div><span class="font-medium">IPv4:</span> {{ iface.ipv4 || 'N/A' }}</div>
                <div><span class="font-medium">接收:</span> {{ iface.rxBytes }}</div>
                <div><span class="font-medium">发送:</span> {{ iface.txBytes }}</div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 防火墙规则 -->
      <UiCard class="lg:col-span-3">
        <UiCardHeader>
          <UiCardTitle>防火墙规则</UiCardTitle>
          <UiCardDescription>网络访问控制规则</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-3">
            <div
              v-for="rule in firewallRules"
              :key="rule.id"
              class="flex items-center justify-between p-3 rounded-lg border"
            >
              <div class="flex items-center space-x-4">
                <div class="flex-1">
                  <div class="flex items-center space-x-2 mb-1">
                    <h3 class="font-semibold">{{ rule.name }}</h3>
                    <UiBadge :variant="getFirewallActionColor(rule.action) as any">
                      {{ getFirewallActionText(rule.action) }}
                    </UiBadge>
                    <UiBadge v-if="!rule.enabled" variant="outline"> 禁用 </UiBadge>
                  </div>
                  <p class="text-sm text-muted-foreground">
                    {{ rule.protocol }}:{{ rule.port }} • 来源: {{ rule.source }}
                  </p>
                </div>
              </div>
              <div class="flex items-center space-x-2">
                <UiSwitch v-model="rule.enabled" />
                <UiButton variant="outline" size="sm">
                  <Edit class="h-4 w-4" />
                </UiButton>
                <UiButton variant="outline" size="sm">
                  <Trash2 class="h-4 w-4" />
                </UiButton>
              </div>
            </div>
          </div>
          <div class="mt-4">
            <UiButton variant="outline">
              <Plus class="h-4 w-4 mr-2" />
              添加规则
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="testConnectivity">
          <Wifi class="h-4 w-4 mr-2" />
          测试连通性
        </UiButton>
        <UiButton variant="outline" @click="restartNetwork">
          <RefreshCw class="h-4 w-4 mr-2" />
          重启网络
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
