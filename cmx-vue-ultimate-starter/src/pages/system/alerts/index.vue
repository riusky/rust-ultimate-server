<template>
  <Page title="告警管理" description="管理系统告警和通知配置" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        新建告警规则
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 告警统计 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总告警数</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">156</div>
            <p class="text-xs text-muted-foreground">今日新增 12 条</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">未处理</UiCardTitle>
            <AlertCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-destructive">23</div>
            <p class="text-xs text-muted-foreground">需要立即处理</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">处理中</UiCardTitle>
            <Clock class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">8</div>
            <p class="text-xs text-muted-foreground">正在处理中</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">已解决</UiCardTitle>
            <CheckCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">125</div>
            <p class="text-xs text-muted-foreground">80% 解决率</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 告警列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>告警列表</UiCardTitle>
          <UiCardDescription>系统产生的所有告警信息</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">告警名称</th>
                  <th scope="col" class="px-6 py-3">级别</th>
                  <th scope="col" class="px-6 py-3">来源</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">发生时间</th>
                  <th scope="col" class="px-6 py-3">持续时间</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="alert in alerts" :key="alert.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-destructive/10 flex items-center justify-center"
                      >
                        <AlertTriangle class="w-4 h-4 text-destructive" />
                      </div>
                      <div>
                        <div class="font-medium">{{ alert.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ alert.description }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getSeverityVariant(alert.severity)">
                      {{ alert.severity }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ alert.source }}</td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(alert.status)">
                      {{ alert.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ alert.occurredAt }}</td>
                  <td class="px-6 py-4">{{ alert.duration }}</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Eye class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <CheckCircle class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Trash2 class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 告警规则 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>告警规则</UiCardTitle>
          <UiCardDescription>系统告警触发规则配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">CPU使用率告警</h3>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">CPU使用率超过80%时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="secondary">警告</UiBadge>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">内存使用率告警</h3>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">内存使用率超过85%时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="destructive">严重</UiBadge>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">磁盘空间告警</h3>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">磁盘使用率超过90%时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="secondary">警告</UiBadge>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">服务不可用告警</h3>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">关键服务停止运行时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="destructive">严重</UiBadge>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">网络延迟告警</h3>
                  <UiSwitch />
                </div>
                <p class="text-sm text-muted-foreground">网络延迟超过100ms时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="secondary">提醒</UiBadge>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <h3 class="font-medium">错误日志告警</h3>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">错误日志数量异常时触发</p>
                <div class="flex items-center space-x-2 text-sm">
                  <span class="text-muted-foreground">级别:</span>
                  <UiBadge variant="secondary">警告</UiBadge>
                </div>
              </div>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 通知渠道 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>通知渠道</UiCardTitle>
          <UiCardDescription>告警通知发送渠道配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <Mail class="w-5 h-5 text-chart-3" />
                    <h3 class="font-medium">邮件通知</h3>
                  </div>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">通过邮件发送告警通知</p>
                <div class="text-sm">
                  <span class="text-muted-foreground">接收人:</span>
                  <span class="ml-2">admin@company.com</span>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <MessageSquare class="w-5 h-5 text-chart-2" />
                    <h3 class="font-medium">短信通知</h3>
                  </div>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">通过短信发送紧急告警</p>
                <div class="text-sm">
                  <span class="text-muted-foreground">手机号:</span>
                  <span class="ml-2">+86 138****1234</span>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <Bell class="w-5 h-5 text-chart-4" />
                    <h3 class="font-medium">站内通知</h3>
                  </div>
                  <UiSwitch checked />
                </div>
                <p class="text-sm text-muted-foreground">系统内消息通知</p>
                <div class="text-sm">
                  <span class="text-muted-foreground">接收组:</span>
                  <span class="ml-2">运维团队</span>
                </div>
              </div>
              <div class="border rounded-lg p-4 space-y-3">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <Webhook class="w-5 h-5 text-chart-5" />
                    <h3 class="font-medium">Webhook</h3>
                  </div>
                  <UiSwitch />
                </div>
                <p class="text-sm text-muted-foreground">通过Webhook发送到第三方系统</p>
                <div class="text-sm">
                  <span class="text-muted-foreground">URL:</span>
                  <span class="ml-2">未配置</span>
                </div>
              </div>
            </div>
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
  AlertTriangle,
  AlertCircle,
  Clock,
  CheckCircle,
  Eye,
  Trash2,
  Mail,
  MessageSquare,
  Bell,
  Webhook,
} from 'lucide-vue-next'

const alerts = ref([
  {
    id: 1,
    name: 'CPU使用率过高',
    description: '服务器 DB-01 CPU使用率超过90%',
    severity: '严重',
    source: 'DB-01',
    status: '未处理',
    occurredAt: '2024-01-15 14:30:25',
    duration: '2小时15分钟',
  },
  {
    id: 2,
    name: '内存不足',
    description: '服务器 Web-02 内存使用率超过85%',
    severity: '警告',
    source: 'Web-02',
    status: '处理中',
    occurredAt: '2024-01-15 13:45:10',
    duration: '3小时',
  },
  {
    id: 3,
    name: '磁盘空间不足',
    description: '/var分区使用率超过95%',
    severity: '严重',
    source: 'File-01',
    status: '未处理',
    occurredAt: '2024-01-15 12:20:35',
    duration: '4小时25分钟',
  },
  {
    id: 4,
    name: '服务不可用',
    description: 'Nginx服务停止运行',
    severity: '严重',
    source: 'Web-01',
    status: '已解决',
    occurredAt: '2024-01-15 10:15:40',
    duration: '15分钟',
  },
  {
    id: 5,
    name: '网络延迟',
    description: '网络延迟超过200ms',
    severity: '提醒',
    source: '网络监控',
    status: '已解决',
    occurredAt: '2024-01-15 09:30:20',
    duration: '45分钟',
  },
  {
    id: 6,
    name: '数据库连接异常',
    description: '数据库连接池耗尽',
    severity: '警告',
    source: 'DB-02',
    status: '处理中',
    occurredAt: '2024-01-15 08:45:15',
    duration: '5小时40分钟',
  },
])

function getSeverityVariant(severity: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    严重: 'destructive',
    警告: 'secondary',
    提醒: 'secondary',
  }
  return variants[severity] || 'outline'
}

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    未处理: 'destructive',
    处理中: 'secondary',
    已解决: 'default',
  }
  return variants[status] || 'outline'
}
</script>

<style scoped></style>
