<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="API文档" description="完整的API接口文档和使用说明">
    <div class="max-w-7xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">API文档</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          完整的RESTful API接口文档，包含请求示例和响应格式
        </p>
      </div>

      <!-- API概览 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">API概览</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <UiCard v-for="endpoint in apiOverview" :key="endpoint.method">
            <UiCardContent class="p-6 text-center">
              <div
                class="w-12 h-12 mx-auto mb-3 rounded-lg bg-primary/10 flex items-center justify-center"
              >
                <component :is="endpoint.icon" class="w-6 h-6 text-primary" />
              </div>
              <h3 class="font-semibold mb-1">{{ endpoint.method }}</h3>
              <p class="text-sm text-muted-foreground">{{ endpoint.description }}</p>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- API端点列表 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">API端点</h2>
        <div class="space-y-4">
          <UiCard v-for="endpoint in apiEndpoints" :key="endpoint.path" class="overflow-hidden">
            <UiCardHeader class="pb-3">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <UiBadge :variant="getMethodVariant(endpoint.method)" class="font-mono">
                      {{ endpoint.method }}
                    </UiBadge>
                    <code class="text-lg font-mono bg-muted px-2 py-1 rounded">
                      {{ endpoint.path }}
                    </code>
                  </div>
                  <UiCardDescription>{{ endpoint.description }}</UiCardDescription>
                </div>
                <UiButton variant="outline" size="sm" @click="toggleEndpoint(endpoint.path)">
                  <UiChevronDown
                    class="w-4 h-4 transition-transform"
                    :class="{ 'rotate-180': openEndpoints.includes(endpoint.path) }"
                  />
                </UiButton>
              </div>
            </UiCardHeader>

            <UiCardContent v-if="openEndpoints.includes(endpoint.path)" class="pt-0 space-y-6">
              <!-- 请求参数 -->
              <div class="space-y-3">
                <h4 class="font-medium">请求参数</h4>
                <div class="rounded-md border overflow-hidden">
                  <UiTable>
                    <UiTableHeader>
                      <UiTableRow>
                        <UiTableHead>参数名</UiTableHead>
                        <UiTableHead>类型</UiTableHead>
                        <UiTableHead>必填</UiTableHead>
                        <UiTableHead>描述</UiTableHead>
                      </UiTableRow>
                    </UiTableHeader>
                    <UiTableBody>
                      <UiTableRow v-for="param in endpoint.parameters" :key="param.name">
                        <UiTableCell class="font-mono text-sm">
                          {{ param.name }}
                        </UiTableCell>
                        <UiTableCell>
                          <UiBadge variant="outline" class="text-xs">
                            {{ param.type }}
                          </UiBadge>
                        </UiTableCell>
                        <UiTableCell>
                          <UiBadge
                            :variant="param.required ? 'destructive' : 'outline'"
                            class="text-xs"
                          >
                            {{ param.required ? '是' : '否' }}
                          </UiBadge>
                        </UiTableCell>
                        <UiTableCell class="text-sm">
                          {{ param.description }}
                        </UiTableCell>
                      </UiTableRow>
                    </UiTableBody>
                  </UiTable>
                </div>
              </div>

              <!-- 请求示例 -->
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <h4 class="font-medium">请求示例</h4>
                  <UiButton variant="outline" size="sm" @click="copyCode(endpoint.requestExample)">
                    <UiCopy class="w-4 h-4 mr-2" />
                    复制代码
                  </UiButton>
                </div>
                <div class="bg-muted rounded-lg p-4 overflow-x-auto">
                  <pre
                    class="text-sm font-mono whitespace-pre"
                  ><code>{{ endpoint.requestExample }}</code></pre>
                </div>
              </div>

              <!-- 响应示例 -->
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <h4 class="font-medium">响应示例</h4>
                  <UiButton variant="outline" size="sm" @click="copyCode(endpoint.responseExample)">
                    <UiCopy class="w-4 h-4 mr-2" />
                    复制代码
                  </UiButton>
                </div>
                <div class="bg-muted rounded-lg p-4 overflow-x-auto">
                  <pre
                    class="text-sm font-mono whitespace-pre"
                  ><code>{{ endpoint.responseExample }}</code></pre>
                </div>
              </div>

              <!-- 状态码 -->
              <div class="space-y-3">
                <h4 class="font-medium">状态码</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div
                    v-for="code in endpoint.statusCodes"
                    :key="code.code"
                    class="flex items-center justify-between p-3 border rounded-lg"
                  >
                    <div class="flex items-center gap-3">
                      <UiBadge :variant="getStatusCodeVariant(code.code)" class="font-mono">
                        {{ code.code }}
                      </UiBadge>
                      <span class="text-sm">{{ code.description }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 认证说明 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>认证说明</UiCardTitle>
          <UiCardDescription> API接口的认证方式和安全要求 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="space-y-4">
          <div class="space-y-2">
            <h4 class="font-medium">Bearer Token认证</h4>
            <p class="text-sm text-muted-foreground">
              所有API请求都需要在请求头中包含有效的Bearer Token进行身份验证。
            </p>
            <div class="bg-muted rounded-lg p-4">
              <pre class="text-sm font-mono">Authorization: Bearer &lt;your_token&gt;</pre>
            </div>
          </div>

          <div class="space-y-2">
            <h4 class="font-medium">获取Token</h4>
            <p class="text-sm text-muted-foreground">
              通过登录接口获取访问令牌，令牌有效期为24小时。
            </p>
            <div class="bg-muted rounded-lg p-4">
              <pre class="text-sm font-mono whitespace-pre"><code>POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "your_password"
}</code></pre>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 复制成功提示 -->
      <UiAlert
        v-if="copySuccess"
        variant="default"
        class="fixed bottom-4 right-4 w-80 animate-in slide-in-from-bottom"
      >
        <UiCheckCircle class="w-4 h-4" />
        <UiAlertTitle>复制成功</UiAlertTitle>
        <UiAlertDescription> 代码已复制到剪贴板 </UiAlertDescription>
      </UiAlert>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import { FileText, Database, User, Settings } from 'lucide-vue-next'

// API概览数据
const apiOverview = [
  {
    method: 'GET',
    description: '获取数据',
    icon: FileText,
  },
  {
    method: 'POST',
    description: '创建数据',
    icon: Database,
  },
  {
    method: 'PUT',
    description: '更新数据',
    icon: Settings,
  },
  {
    method: 'DELETE',
    description: '删除数据',
    icon: User,
  },
]

// API端点数据
const apiEndpoints = [
  {
    method: 'GET',
    path: '/api/users',
    description: '获取用户列表，支持分页和搜索',
    parameters: [
      {
        name: 'page',
        type: 'number',
        required: false,
        description: '页码，默认为1',
      },
      {
        name: 'limit',
        type: 'number',
        required: false,
        description: '每页数量，默认为10',
      },
      {
        name: 'search',
        type: 'string',
        required: false,
        description: '搜索关键词',
      },
      {
        name: 'status',
        type: 'string',
        required: false,
        description: '状态筛选：active, inactive',
      },
    ],
    requestExample: `GET /api/users?page=1&limit=10&search=john&status=active
Authorization: Bearer your_token_here`,
    responseExample: `{
  "success": true,
  "data": {
    "users": [
      {
        "id": 1,
        "name": "John Doe",
        "email": "john@example.com",
        "status": "active",
        "createdAt": "2024-01-15T08:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 10,
      "total": 100,
      "pages": 10
    }
  }
}`,
    statusCodes: [
      { code: 200, description: '成功获取用户列表' },
      { code: 401, description: '未授权访问' },
      { code: 500, description: '服务器内部错误' },
    ],
  },
  {
    method: 'POST',
    path: '/api/users',
    description: '创建新用户',
    parameters: [
      {
        name: 'name',
        type: 'string',
        required: true,
        description: '用户姓名',
      },
      {
        name: 'email',
        type: 'string',
        required: true,
        description: '用户邮箱',
      },
      {
        name: 'password',
        type: 'string',
        required: true,
        description: '用户密码',
      },
      {
        name: 'role',
        type: 'string',
        required: false,
        description: '用户角色：user, admin',
      },
    ],
    requestExample: `POST /api/users
Content-Type: application/json
Authorization: Bearer your_token_here

{
  "name": "Jane Smith",
  "email": "jane@example.com",
  "password": "secure_password",
  "role": "user"
}`,
    responseExample: `{
  "success": true,
  "data": {
    "user": {
      "id": 2,
      "name": "Jane Smith",
      "email": "jane@example.com",
      "role": "user",
      "status": "active",
      "createdAt": "2024-01-20T10:30:00Z"
    }
  }
}`,
    statusCodes: [
      { code: 201, description: '用户创建成功' },
      { code: 400, description: '请求参数错误' },
      { code: 409, description: '邮箱已存在' },
      { code: 401, description: '未授权访问' },
      { code: 500, description: '服务器内部错误' },
    ],
  },
  {
    method: 'PUT',
    path: '/api/users/{id}',
    description: '更新用户信息',
    parameters: [
      {
        name: 'id',
        type: 'number',
        required: true,
        description: '用户ID',
      },
      {
        name: 'name',
        type: 'string',
        required: false,
        description: '用户姓名',
      },
      {
        name: 'email',
        type: 'string',
        required: false,
        description: '用户邮箱',
      },
      {
        name: 'status',
        type: 'string',
        required: false,
        description: '用户状态：active, inactive',
      },
    ],
    requestExample: `PUT /api/users/1
Content-Type: application/json
Authorization: Bearer your_token_here

{
  "name": "John Updated",
  "status": "inactive"
}`,
    responseExample: `{
  "success": true,
  "data": {
    "user": {
      "id": 1,
      "name": "John Updated",
      "email": "john@example.com",
      "status": "inactive",
      "updatedAt": "2024-01-20T14:45:00Z"
    }
  }
}`,
    statusCodes: [
      { code: 200, description: '用户更新成功' },
      { code: 400, description: '请求参数错误' },
      { code: 404, description: '用户不存在' },
      { code: 401, description: '未授权访问' },
      { code: 500, description: '服务器内部错误' },
    ],
  },
  {
    method: 'DELETE',
    path: '/api/users/{id}',
    description: '删除用户',
    parameters: [
      {
        name: 'id',
        type: 'number',
        required: true,
        description: '用户ID',
      },
    ],
    requestExample: `DELETE /api/users/1
Authorization: Bearer your_token_here`,
    responseExample: `{
  "success": true,
  "message": "用户删除成功"
}`,
    statusCodes: [
      { code: 200, description: '用户删除成功' },
      { code: 404, description: '用户不存在' },
      { code: 401, description: '未授权访问' },
      { code: 500, description: '服务器内部错误' },
    ],
  },
]

// 响应式数据
const openEndpoints = ref<string[]>([])
const copySuccess = ref(false)

// 方法
const getMethodVariant = (method: string) => {
  switch (method) {
    case 'GET':
      return 'default'
    case 'POST':
      return 'secondary'
    case 'PUT':
      return 'outline'
    case 'DELETE':
      return 'destructive'
    default:
      return 'outline'
  }
}

const getStatusCodeVariant = (code: number) => {
  if (code >= 200 && code < 300) return 'default'
  if (code >= 400 && code < 500) return 'destructive'
  return 'outline'
}

const toggleEndpoint = (path: string) => {
  const index = openEndpoints.value.indexOf(path)
  if (index > -1) {
    openEndpoints.value.splice(index, 1)
  } else {
    openEndpoints.value.push(path)
  }
}

const copyCode = async (code: string) => {
  try {
    await navigator.clipboard.writeText(code)
    copySuccess.value = true

    // 3秒后隐藏成功提示
    setTimeout(() => {
      copySuccess.value = false
    }, 3000)
  } catch (err) {
    console.error('复制失败:', err)
  }
}
</script>

<style scoped></style>
