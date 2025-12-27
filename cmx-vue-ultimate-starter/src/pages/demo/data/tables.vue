<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="数据表格" description="这是一个数据表格示例，展示用户数据的管理和操作">
    <div class="space-y-6">
      <!-- 表格操作栏 -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3">
          <UiInput placeholder="搜索用户..." class="w-full sm:w-64" v-model="searchQuery" />
          <UiSelect v-model="statusFilter">
            <UiSelectTrigger class="w-32">
              <UiSelectValue placeholder="状态筛选" />
            </UiSelectTrigger>
            <UiSelectContent>
              <UiSelectItem value="all">全部</UiSelectItem>
              <UiSelectItem value="active">活跃</UiSelectItem>
              <UiSelectItem value="inactive">非活跃</UiSelectItem>
              <UiSelectItem value="banned">已禁用</UiSelectItem>
            </UiSelectContent>
          </UiSelect>
        </div>
        <div class="flex gap-2">
          <UiButton @click="exportData">
            <UiDownload class="w-4 h-4 mr-2" />
            导出数据
          </UiButton>
          <UiButton @click="showAddDialog = true">
            <UiPlus class="w-4 h-4 mr-2" />
            添加用户
          </UiButton>
        </div>
      </div>

      <!-- 数据表格 -->
      <UiCard>
        <UiCardContent class="p-0">
          <div class="rounded-md border">
            <UiTable>
              <UiTableHeader>
                <UiTableRow>
                  <UiTableHead class="w-12">
                    <UiCheckbox :checked="selectAll" @update:checked="toggleSelectAll" />
                  </UiTableHead>
                  <UiTableHead>用户信息</UiTableHead>
                  <UiTableHead>状态</UiTableHead>
                  <UiTableHead>角色</UiTableHead>
                  <UiTableHead>注册时间</UiTableHead>
                  <UiTableHead class="text-right">操作</UiTableHead>
                </UiTableRow>
              </UiTableHeader>
              <UiTableBody>
                <UiTableRow
                  v-for="user in paginatedUsers"
                  :key="user.id"
                  :class="selectedUsers.includes(user.id) ? 'bg-muted/50' : ''"
                >
                  <UiTableCell>
                    <UiCheckbox
                      :checked="selectedUsers.includes(user.id)"
                      @update:checked="(checked: boolean) => toggleSelectUser(user.id, checked)"
                    />
                  </UiTableCell>
                  <UiTableCell>
                    <div class="flex items-center gap-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <span class="text-sm font-medium text-primary">
                          {{ user.name.charAt(0).toUpperCase() }}
                        </span>
                      </div>
                      <div>
                        <p class="font-medium">{{ user.name }}</p>
                        <p class="text-sm text-muted-foreground">{{ user.email }}</p>
                      </div>
                    </div>
                  </UiTableCell>
                  <UiTableCell>
                    <UiBadge :variant="getStatusVariant(user.status)" class="capitalize">
                      {{ getStatusText(user.status) }}
                    </UiBadge>
                  </UiTableCell>
                  <UiTableCell>
                    <UiBadge variant="outline" class="capitalize">
                      {{ user.role }}
                    </UiBadge>
                  </UiTableCell>
                  <UiTableCell>
                    {{ formatDate(user.createdAt) }}
                  </UiTableCell>
                  <UiTableCell>
                    <div class="flex justify-end gap-2">
                      <UiButton variant="ghost" size="sm" @click="editUser(user)">
                        <UiEdit class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm" @click="viewUser(user)">
                        <UiEye class="w-4 h-4" />
                      </UiButton>
                      <UiButton
                        variant="ghost"
                        size="sm"
                        class="text-destructive hover:text-destructive"
                        @click="deleteUser(user)"
                      >
                        <UiTrash2 class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </UiTableCell>
                </UiTableRow>
              </UiTableBody>
            </UiTable>
          </div>

          <!-- 空状态 -->
          <div
            v-if="filteredUsers.length === 0"
            class="flex flex-col items-center justify-center py-12 text-center"
          >
            <UiUsers class="w-12 h-12 text-muted-foreground mb-4" />
            <h3 class="text-lg font-medium">暂无数据</h3>
            <p class="text-sm text-muted-foreground mt-1">没有找到匹配的用户数据</p>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 分页控件 -->
      <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
        <div class="text-sm text-muted-foreground">
          显示第 {{ startIndex }} 到 {{ endIndex }} 条，共 {{ filteredUsers.length }} 条记录
        </div>
        <div class="flex items-center gap-2">
          <UiButton
            variant="outline"
            size="sm"
            :disabled="currentPage === 1"
            @click="currentPage--"
          >
            上一页
          </UiButton>
          <div class="flex items-center gap-1">
            <UiButton
              v-for="page in visiblePages"
              :key="page"
              variant="outline"
              size="sm"
              :class="currentPage === page ? 'bg-primary text-primary-foreground' : ''"
              @click="currentPage = page"
            >
              {{ page }}
            </UiButton>
          </div>
          <UiButton
            variant="outline"
            size="sm"
            :disabled="currentPage === totalPages"
            @click="currentPage++"
          >
            下一页
          </UiButton>
        </div>
      </div>
    </div>

    <!-- 批量操作栏 -->
    <div
      v-if="selectedUsers.length > 0"
      class="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-background border rounded-lg shadow-lg p-4"
    >
      <div class="flex items-center gap-4">
        <span class="text-sm font-medium"> 已选择 {{ selectedUsers.length }} 个用户 </span>
        <div class="flex gap-2">
          <UiButton variant="outline" size="sm" @click="batchActivate"> 激活 </UiButton>
          <UiButton variant="outline" size="sm" @click="batchDeactivate"> 禁用 </UiButton>
          <UiButton variant="destructive" size="sm" @click="batchDelete"> 删除 </UiButton>
          <UiButton variant="ghost" size="sm" @click="clearSelection"> 取消选择 </UiButton>
        </div>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'

interface User {
  id: number
  name: string
  email: string
  status: 'active' | 'inactive' | 'banned'
  role: string
  createdAt: string
}

// 模拟数据
const mockUsers: User[] = [
  {
    id: 1,
    name: '张三',
    email: 'zhangsan@example.com',
    status: 'active',
    role: 'admin',
    createdAt: '2024-01-15T08:00:00Z',
  },
  {
    id: 2,
    name: '李四',
    email: 'lisi@example.com',
    status: 'active',
    role: 'user',
    createdAt: '2024-01-16T09:30:00Z',
  },
  {
    id: 3,
    name: '王五',
    email: 'wangwu@example.com',
    status: 'inactive',
    role: 'user',
    createdAt: '2024-01-17T10:15:00Z',
  },
  {
    id: 4,
    name: '赵六',
    email: 'zhaoliu@example.com',
    status: 'banned',
    role: 'user',
    createdAt: '2024-01-18T11:45:00Z',
  },
  {
    id: 5,
    name: '钱七',
    email: 'qianqi@example.com',
    status: 'active',
    role: 'moderator',
    createdAt: '2024-01-19T14:20:00Z',
  },
  {
    id: 6,
    name: '孙八',
    email: 'sunba@example.com',
    status: 'inactive',
    role: 'user',
    createdAt: '2024-01-20T16:10:00Z',
  },
  {
    id: 7,
    name: '周九',
    email: 'zhoujiu@example.com',
    status: 'active',
    role: 'user',
    createdAt: '2024-01-21T17:30:00Z',
  },
  {
    id: 8,
    name: '吴十',
    email: 'wushi@example.com',
    status: 'active',
    role: 'user',
    createdAt: '2024-01-22T18:45:00Z',
  },
]

// 响应式数据
const users = ref<User[]>([])
const searchQuery = ref('')
const statusFilter = ref('all')
const selectedUsers = ref<number[]>([])
const currentPage = ref(1)
const pageSize = 5
const showAddDialog = ref(false)

// 计算属性
const filteredUsers = computed(() => {
  let filtered = users.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (user) => user.name.toLowerCase().includes(query) || user.email.toLowerCase().includes(query),
    )
  }

  // 状态过滤
  if (statusFilter.value !== 'all') {
    filtered = filtered.filter((user) => user.status === statusFilter.value)
  }

  return filtered
})

const paginatedUsers = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredUsers.value.slice(start, end)
})

const totalPages = computed(() => Math.ceil(filteredUsers.value.length / pageSize))

const selectAll = computed(() => {
  if (paginatedUsers.value.length === 0) return false
  return paginatedUsers.value.every((user) => selectedUsers.value.includes(user.id))
})

const startIndex = computed(() => (currentPage.value - 1) * pageSize + 1)
const endIndex = computed(() => Math.min(currentPage.value * pageSize, filteredUsers.value.length))

const visiblePages = computed(() => {
  const pages = []
  const maxVisiblePages = 5
  let startPage = Math.max(1, currentPage.value - Math.floor(maxVisiblePages / 2))
  const endPage = Math.min(totalPages.value, startPage + maxVisiblePages - 1)

  if (endPage - startPage + 1 < maxVisiblePages) {
    startPage = Math.max(1, endPage - maxVisiblePages + 1)
  }

  for (let i = startPage; i <= endPage; i++) {
    pages.push(i)
  }
  return pages
})

// 方法
const getStatusVariant = (status: string) => {
  switch (status) {
    case 'active':
      return 'default'
    case 'inactive':
      return 'secondary'
    case 'banned':
      return 'destructive'
    default:
      return 'outline'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'active':
      return '活跃'
    case 'inactive':
      return '非活跃'
    case 'banned':
      return '已禁用'
    default:
      return status
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('zh-CN')
}

const toggleSelectAll = (checked: boolean) => {
  if (checked) {
    selectedUsers.value = paginatedUsers.value.map((user) => user.id)
  } else {
    selectedUsers.value = selectedUsers.value.filter(
      (id) => !paginatedUsers.value.some((user) => user.id === id),
    )
  }
}

const toggleSelectUser = (userId: number, checked: boolean) => {
  if (checked) {
    selectedUsers.value.push(userId)
  } else {
    const index = selectedUsers.value.indexOf(userId)
    if (index > -1) {
      selectedUsers.value.splice(index, 1)
    }
  }
}

const clearSelection = () => {
  selectedUsers.value = []
}

const editUser = (user: User) => {
  console.log('编辑用户:', user)
  // 这里可以打开编辑对话框
}

const viewUser = (user: User) => {
  console.log('查看用户:', user)
  // 这里可以打开查看详情对话框
}

const deleteUser = (user: User) => {
  console.log('删除用户:', user)
  // 这里可以显示确认删除对话框
}

const exportData = () => {
  console.log('导出数据')
  // 这里可以实现数据导出功能
}

const batchActivate = () => {
  console.log('批量激活:', selectedUsers.value)
  // 这里可以实现批量激活功能
}

const batchDeactivate = () => {
  console.log('批量禁用:', selectedUsers.value)
  // 这里可以实现批量禁用功能
}

const batchDelete = () => {
  console.log('批量删除:', selectedUsers.value)
  // 这里可以实现批量删除功能
}

// 生命周期
onMounted(() => {
  // 模拟从API获取数据
  setTimeout(() => {
    users.value = mockUsers
  }, 500)
})

// 监听搜索和筛选变化，重置页码
watch([searchQuery, statusFilter], () => {
  currentPage.value = 1
})
</script>

<style scoped></style>
