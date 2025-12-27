<template>
  <Page title="命令行工具" description="系统命令行工具和远程终端" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        新建会话
      </UiButton>
      <UiButton variant="outline">
        <Download class="w-4 h-4 mr-2" />
        导出日志
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 终端会话概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">活跃会话</UiCardTitle>
            <Terminal class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">3</div>
            <p class="text-xs text-muted-foreground">当前活跃终端会话</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">今日命令</UiCardTitle>
            <Command class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">245</div>
            <p class="text-xs text-muted-foreground">今日执行命令数</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">错误命令</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">12</div>
            <p class="text-xs text-muted-foreground">今日错误命令数</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">平均响应</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-5">45ms</div>
            <p class="text-xs text-muted-foreground">命令平均响应时间</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 终端模拟器 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>终端模拟器</UiCardTitle>
          <UiCardDescription>Web终端模拟器，支持基本命令执行</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="border rounded-lg bg-black text-green-400 font-mono text-sm">
            <div class="p-4 border-b border-gray-700">
              <div class="flex items-center space-x-2">
                <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                <div class="text-gray-400 ml-2">terminal@system:~</div>
              </div>
            </div>
            <div class="p-4 h-64 overflow-y-auto">
              <div class="space-y-2">
                <div class="flex">
                  <span class="text-blue-400">user@system:~$</span>
                  <span class="ml-2">ls -la</span>
                </div>
                <div class="text-gray-300">
                  <div>drwxr-xr-x 12 user user 4096 Jan 15 14:30 .</div>
                  <div>drwxr-xr-x 3 root root 4096 Jan 10 09:15 ..</div>
                  <div>-rw-r--r-- 1 user user 220 Jan 10 09:15 .bash_logout</div>
                  <div>-rw-r--r-- 1 user user 3771 Jan 10 09:15 .bashrc</div>
                  <div>drwx------ 2 user user 4096 Jan 15 14:30 .cache</div>
                  <div>drwxr-xr-x 3 user user 4096 Jan 15 14:30 .config</div>
                </div>
                <div class="flex">
                  <span class="text-blue-400">user@system:~$</span>
                  <span class="ml-2">pwd</span>
                </div>
                <div class="text-gray-300">/home/user</div>
                <div class="flex">
                  <span class="text-blue-400">user@system:~$</span>
                  <span class="ml-2 flex-1">
                    <input
                      type="text"
                      class="bg-transparent border-none outline-none w-full text-green-400"
                      placeholder="输入命令..."
                      v-model="currentCommand"
                      @keyup.enter="executeCommand"
                    />
                  </span>
                </div>
              </div>
            </div>
          </div>
          <div class="mt-4 flex items-center space-x-4 text-sm">
            <div class="flex items-center space-x-2">
              <UiLabel for="server-select">目标服务器:</UiLabel>
              <UiSelect id="server-select" default-value="local">
                <UiSelectTrigger class="w-[200px]">
                  <UiSelectValue placeholder="选择服务器" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="local">本地服务器</UiSelectItem>
                  <UiSelectItem value="web-01">Web-01 (192.168.1.101)</UiSelectItem>
                  <UiSelectItem value="db-01">DB-01 (192.168.1.102)</UiSelectItem>
                  <UiSelectItem value="file-01">File-01 (192.168.1.103)</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
            <UiButton size="sm" @click="executeCommand">
              <Play class="w-4 h-4 mr-2" />
              执行
            </UiButton>
            <UiButton variant="outline" size="sm">
              <Trash2 class="w-4 h-4 mr-2" />
              清屏
            </UiButton>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 常用命令 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>常用命令</UiCardTitle>
          <UiCardDescription>系统管理和运维常用命令</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div
              class="border rounded-lg p-4 space-y-2 cursor-pointer hover:bg-muted/50"
              @click="insertCommand('systemctl status nginx')"
            >
              <Server class="w-6 h-6 text-chart-3" />
              <h3 class="font-medium">服务状态</h3>
              <p class="text-sm text-muted-foreground">检查服务运行状态</p>
              <code class="text-xs bg-muted px-2 py-1 rounded block">systemctl status</code>
            </div>
            <div
              class="border rounded-lg p-4 space-y-2 cursor-pointer hover:bg-muted/50"
              @click="insertCommand('df -h')"
            >
              <HardDrive class="w-6 h-6 text-chart-4" />
              <h3 class="font-medium">磁盘空间</h3>
              <p class="text-sm text-muted-foreground">查看磁盘使用情况</p>
              <code class="text-xs bg-muted px-2 py-1 rounded block">df -h</code>
            </div>
            <div
              class="border rounded-lg p-4 space-y-2 cursor-pointer hover:bg-muted/50"
              @click="insertCommand('free -h')"
            >
              <Cpu class="w-6 h-6 text-chart-2" />
              <h3 class="font-medium">内存使用</h3>
              <p class="text-sm text-muted-foreground">查看内存使用情况</p>
              <code class="text-xs bg-muted px-2 py-1 rounded block">free -h</code>
            </div>
            <div
              class="border rounded-lg p-4 space-y-2 cursor-pointer hover:bg-muted/50"
              @click="insertCommand('top -n 1')"
            >
              <Activity class="w-6 h-6 text-chart-5" />
              <h3 class="font-medium">系统监控</h3>
              <p class="text-sm text-muted-foreground">实时系统监控</p>
              <code class="text-xs bg-muted px-2 py-1 rounded block">top</code>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 命令历史 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>命令历史</UiCardTitle>
          <UiCardDescription>最近执行的命令记录</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">时间</th>
                  <th scope="col" class="px-6 py-3">用户</th>
                  <th scope="col" class="px-6 py-3">命令</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">执行时间</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="cmd in commandHistory" :key="cmd.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">{{ cmd.timestamp }}</td>
                  <td class="px-6 py-4">{{ cmd.user }}</td>
                  <td class="px-6 py-4">
                    <code class="text-xs bg-muted px-2 py-1 rounded">{{ cmd.command }}</code>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="cmd.status === '成功' ? 'default' : 'destructive'">
                      {{ cmd.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ cmd.duration }}ms</td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import {
  Plus,
  Download,
  Terminal,
  Command,
  AlertTriangle,
  Clock,
  Play,
  Trash2,
  Server,
  HardDrive,
  Cpu,
  Activity,
} from 'lucide-vue-next'

const currentCommand = ref('')

const commandHistory = ref([
  {
    id: 1,
    timestamp: '2024-01-15 14:30:25',
    user: 'admin',
    command: 'systemctl status nginx',
    status: '成功',
    duration: 45,
  },
  {
    id: 2,
    timestamp: '2024-01-15 14:28:10',
    user: 'admin',
    command: 'df -h',
    status: '成功',
    duration: 32,
  },
  {
    id: 3,
    timestamp: '2024-01-15 14:25:45',
    user: 'admin',
    command: 'free -h',
    status: '成功',
    duration: 28,
  },
  {
    id: 4,
    timestamp: '2024-01-15 14:23:20',
    user: 'admin',
    command: 'invalid-command',
    status: '失败',
    duration: 15,
  },
  {
    id: 5,
    timestamp: '2024-01-15 14:20:15',
    user: 'admin',
    command: 'top -n 1',
    status: '成功',
    duration: 120,
  },
  {
    id: 6,
    timestamp: '2024-01-15 14:18:30',
    user: 'admin',
    command: 'netstat -tulpn',
    status: '成功',
    duration: 65,
  },
])

function insertCommand(command: string) {
  currentCommand.value = command
}

function executeCommand() {
  if (currentCommand.value.trim()) {
    // 模拟命令执行
    console.log('执行命令:', currentCommand.value)
    currentCommand.value = ''
  }
}
</script>

<style scoped></style>
