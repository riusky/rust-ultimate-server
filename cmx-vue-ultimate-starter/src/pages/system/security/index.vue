<template>
  <Page title="安全策略" description="配置和管理系统安全策略" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        新建策略
      </UiButton>
      <UiButton variant="outline">
        <Download class="w-4 h-4 mr-2" />
        导出配置
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 安全概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">安全评分</UiCardTitle>
            <Shield class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">92</div>
            <p class="text-xs text-muted-foreground">/100 优秀</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">活跃策略</UiCardTitle>
            <FileText class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">15</div>
            <p class="text-xs text-muted-foreground">生效中的策略</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">安全事件</UiCardTitle>
            <AlertTriangle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">3</div>
            <p class="text-xs text-muted-foreground">今日安全事件</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">合规状态</UiCardTitle>
            <CheckCircle class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">100%</div>
            <p class="text-xs text-muted-foreground">合规检查通过</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 安全策略列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>安全策略列表</UiCardTitle>
          <UiCardDescription>系统安全策略配置和管理</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">策略名称</th>
                  <th scope="col" class="px-6 py-3">类型</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">优先级</th>
                  <th scope="col" class="px-6 py-3">最后更新</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="policy in securityPolicies"
                  :key="policy.id"
                  class="border-b hover:bg-muted/50"
                >
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <Shield class="w-4 h-4 text-primary" />
                      </div>
                      <div>
                        <div class="font-medium">{{ policy.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ policy.description }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge variant="outline">{{ policy.type }}</UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(policy.status)">
                      {{ policy.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getPriorityVariant(policy.priority)">
                      {{ policy.priority }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ policy.lastUpdated }}</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Edit class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Settings class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 安全配置 -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- 访问控制策略 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>访问控制策略</UiCardTitle>
            <UiCardDescription>配置系统访问控制规则</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">IP白名单</p>
                  <p class="text-sm text-muted-foreground">允许访问的IP地址列表</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">双因素认证</p>
                  <p class="text-sm text-muted-foreground">启用双因素身份验证</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">会话超时</p>
                  <p class="text-sm text-muted-foreground">自动登出空闲会话</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">密码策略</p>
                  <p class="text-sm text-muted-foreground">强制密码复杂度要求</p>
                </div>
                <UiSwitch checked />
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <!-- 网络安全策略 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>网络安全策略</UiCardTitle>
            <UiCardDescription>配置网络层安全防护</UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">防火墙</p>
                  <p class="text-sm text-muted-foreground">启用系统防火墙</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">DDoS防护</p>
                  <p class="text-sm text-muted-foreground">启用DDoS攻击防护</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">SSL/TLS加密</p>
                  <p class="text-sm text-muted-foreground">强制HTTPS连接</p>
                </div>
                <UiSwitch checked />
              </div>
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <p class="text-sm font-medium">端口扫描防护</p>
                  <p class="text-sm text-muted-foreground">检测和阻止端口扫描</p>
                </div>
                <UiSwitch />
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 安全建议 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>安全建议</UiCardTitle>
          <UiCardDescription>基于当前配置的安全优化建议</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="flex items-start space-x-3 p-4 border border-chart-3/20 bg-chart-3/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">更新SSL证书</p>
                <p class="text-sm text-muted-foreground">部分SSL证书即将过期，建议及时更新</p>
                <p class="text-xs text-muted-foreground mt-1">影响: 中等</p>
              </div>
              <UiBadge variant="secondary">建议</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-4/20 bg-chart-4/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">加强密码策略</p>
                <p class="text-sm text-muted-foreground">建议增加密码最小长度和复杂度要求</p>
                <p class="text-xs text-muted-foreground mt-1">影响: 低</p>
              </div>
              <UiBadge variant="secondary">建议</UiBadge>
            </div>
            <div
              class="flex items-start space-x-3 p-4 border border-chart-2/20 bg-chart-2/10 rounded-lg"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">启用审计日志</p>
                <p class="text-sm text-muted-foreground">建议启用完整的系统审计日志记录</p>
                <p class="text-xs text-muted-foreground mt-1">影响: 高</p>
              </div>
              <UiBadge variant="destructive">重要</UiBadge>
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
  Download,
  Shield,
  FileText,
  AlertTriangle,
  CheckCircle,
  Edit,
  Settings,
} from 'lucide-vue-next'

const securityPolicies = ref([
  {
    id: 1,
    name: '密码策略',
    description: '用户密码复杂度要求',
    type: '认证安全',
    status: '启用',
    priority: '高',
    lastUpdated: '2024-01-15',
  },
  {
    id: 2,
    name: '访问控制',
    description: '基于角色的访问控制',
    type: '权限管理',
    status: '启用',
    priority: '高',
    lastUpdated: '2024-01-14',
  },
  {
    id: 3,
    name: '会话管理',
    description: '用户会话超时设置',
    type: '会话安全',
    status: '启用',
    priority: '中',
    lastUpdated: '2024-01-13',
  },
  {
    id: 4,
    name: '网络防火墙',
    description: '网络流量过滤规则',
    type: '网络安全',
    status: '启用',
    priority: '高',
    lastUpdated: '2024-01-12',
  },
  {
    id: 5,
    name: '数据加密',
    description: '敏感数据加密策略',
    type: '数据安全',
    status: '启用',
    priority: '中',
    lastUpdated: '2024-01-11',
  },
  {
    id: 6,
    name: '审计日志',
    description: '安全事件记录策略',
    type: '审计安全',
    status: '禁用',
    priority: '低',
    lastUpdated: '2024-01-10',
  },
])

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    启用: 'default',
    禁用: 'outline',
  }
  return variants[status] || 'outline'
}

function getPriorityVariant(priority: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    高: 'destructive',
    中: 'secondary',
    低: 'outline',
  }
  return variants[priority] || 'outline'
}
</script>

<style scoped></style>
