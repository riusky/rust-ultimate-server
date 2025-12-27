<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
import Page from '@/components/global-layout/basic-page.vue'

const serverConfig = ref({
  name: 'Web Server 01',
  ip: '192.168.1.101',
  port: 8080,
  cpuCores: 8,
  memory: 16,
  diskSize: 500,
  os: 'Ubuntu 22.04 LTS',
  kernel: '5.15.0-91-generic',
  timezone: 'Asia/Shanghai',
  language: 'zh_CN.UTF-8',
  sshPort: 22,
  maxConnections: 1000,
  autoStart: true,
  monitoring: true,
  backupEnabled: true,
})

const networkConfig = ref({
  dnsServers: ['8.8.8.8', '8.8.4.4'],
  gateway: '192.168.1.1',
  subnetMask: '255.255.255.0',
  mtu: 1500,
  ipv6Enabled: false,
})

const securityConfig = ref({
  firewallEnabled: true,
  sshKeyAuth: true,
  fail2banEnabled: true,
  maxLoginAttempts: 3,
  sessionTimeout: 30,
})

const saveConfig = () => {
  // 保存配置逻辑
  console.log('保存服务器配置:', serverConfig.value)
}

const restartServer = () => {
  // 重启服务器逻辑
  console.log('重启服务器')
}

const testConnection = () => {
  // 测试连接逻辑
  console.log('测试服务器连接')
}
</script>

<template>
  <Page
    :title="transformI18n('system.server-config.title')"
    :description="transformI18n('system.server-config.description')"
    sticky
  >
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- 基本配置 -->
      <UiCard class="lg:col-span-2">
        <UiCardHeader>
          <UiCardTitle>基本配置</UiCardTitle>
          <UiCardDescription>服务器基本信息和系统设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <UiLabel for="server-name">服务器名称</UiLabel>
                <UiInput id="server-name" v-model="serverConfig.name" />
              </div>
              <div>
                <UiLabel for="server-ip">IP 地址</UiLabel>
                <UiInput id="server-ip" v-model="serverConfig.ip" />
              </div>
              <div>
                <UiLabel for="server-port">端口</UiLabel>
                <UiInput id="server-port" v-model="serverConfig.port" type="number" />
              </div>
              <div>
                <UiLabel for="ssh-port">SSH 端口</UiLabel>
                <UiInput id="ssh-port" v-model="serverConfig.sshPort" type="number" />
              </div>
              <div>
                <UiLabel for="cpu-cores">CPU 核心数</UiLabel>
                <UiInput id="cpu-cores" v-model="serverConfig.cpuCores" type="number" />
              </div>
              <div>
                <UiLabel for="memory">内存 (GB)</UiLabel>
                <UiInput id="memory" v-model="serverConfig.memory" type="number" />
              </div>
              <div>
                <UiLabel for="disk-size">磁盘大小 (GB)</UiLabel>
                <UiInput id="disk-size" v-model="serverConfig.diskSize" type="number" />
              </div>
              <div>
                <UiLabel for="max-connections">最大连接数</UiLabel>
                <UiInput id="max-connections" v-model="serverConfig.maxConnections" type="number" />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <UiLabel for="timezone">时区</UiLabel>
                <UiInput id="timezone" v-model="serverConfig.timezone" />
              </div>
              <div>
                <UiLabel for="language">语言</UiLabel>
                <UiInput id="language" v-model="serverConfig.language" />
              </div>
            </div>

            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="auto-start" v-model="serverConfig.autoStart" />
                <UiLabel for="auto-start">开机自启动</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="monitoring" v-model="serverConfig.monitoring" />
                <UiLabel for="monitoring">启用监控</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="backup" v-model="serverConfig.backupEnabled" />
                <UiLabel for="backup">启用自动备份</UiLabel>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 网络配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>网络配置</UiCardTitle>
          <UiCardDescription>网络连接和通信设置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div>
              <UiLabel for="dns-servers">DNS 服务器</UiLabel>
              <UiInput id="dns-servers" :value="networkConfig.dnsServers.join(', ')" />
            </div>
            <div>
              <UiLabel for="gateway">网关</UiLabel>
              <UiInput id="gateway" v-model="networkConfig.gateway" />
            </div>
            <div>
              <UiLabel for="subnet-mask">子网掩码</UiLabel>
              <UiInput id="subnet-mask" v-model="networkConfig.subnetMask" />
            </div>
            <div>
              <UiLabel for="mtu">MTU</UiLabel>
              <UiInput id="mtu" v-model="networkConfig.mtu" type="number" />
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="ipv6" v-model="networkConfig.ipv6Enabled" />
              <UiLabel for="ipv6">启用 IPv6</UiLabel>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 安全配置 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>安全配置</UiCardTitle>
          <UiCardDescription>安全策略和访问控制</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="flex items-center space-x-2">
              <UiCheckbox id="firewall" v-model="securityConfig.firewallEnabled" />
              <UiLabel for="firewall">启用防火墙</UiLabel>
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="ssh-key" v-model="securityConfig.sshKeyAuth" />
              <UiLabel for="ssh-key">SSH 密钥认证</UiLabel>
            </div>
            <div class="flex items-center space-x-2">
              <UiCheckbox id="fail2ban" v-model="securityConfig.fail2banEnabled" />
              <UiLabel for="fail2ban">启用 Fail2ban</UiLabel>
            </div>
            <div>
              <UiLabel for="max-login-attempts">最大登录尝试次数</UiLabel>
              <UiInput
                id="max-login-attempts"
                v-model="securityConfig.maxLoginAttempts"
                type="number"
              />
            </div>
            <div>
              <UiLabel for="session-timeout">会话超时 (分钟)</UiLabel>
              <UiInput id="session-timeout" v-model="securityConfig.sessionTimeout" type="number" />
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 操作按钮 -->
      <div class="lg:col-span-3 flex justify-end space-x-4">
        <UiButton variant="outline" @click="testConnection">
          <Network class="h-4 w-4 mr-2" />
          测试连接
        </UiButton>
        <UiButton variant="outline" @click="restartServer">
          <RefreshCw class="h-4 w-4 mr-2" />
          重启服务器
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
