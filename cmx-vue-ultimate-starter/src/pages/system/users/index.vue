<template>
  <Page title="用户管理" description="管理系统用户账户和权限" sticky>
    <template #actions>
      <UiButton>
        <UserPlus class="w-4 h-4 mr-2" />
        添加用户
      </UiButton>
    </template>

    <div class="space-y-4">
      <!-- 搜索和筛选 -->
      <div class="flex flex-col sm:flex-row gap-4">
        <div class="flex-1">
          <UiInput placeholder="搜索用户..." class="max-w-sm">
            <template #prefix>
              <Search class="w-4 h-4" />
            </template>
          </UiInput>
        </div>
        <UiSelect default-value="all">
          <UiSelectTrigger class="w-[180px]">
            <UiSelectValue placeholder="状态筛选" />
          </UiSelectTrigger>
          <UiSelectContent>
            <UiSelectItem value="all">全部状态</UiSelectItem>
            <UiSelectItem value="active">活跃</UiSelectItem>
            <UiSelectItem value="inactive">未激活</UiSelectItem>
            <UiSelectItem value="locked">已锁定</UiSelectItem>
          </UiSelectContent>
        </UiSelect>
      </div>

      <!-- 用户列表 -->
      <UiCard>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">用户</th>
                  <th scope="col" class="px-6 py-3">邮箱</th>
                  <th scope="col" class="px-6 py-3">角色</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">最后登录</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in users" :key="user.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <UiAvatar class="h-8 w-8">
                        <UiAvatarImage :src="user.avatar" />
                        <UiAvatarFallback>{{ user.name.charAt(0) }}</UiAvatarFallback>
                      </UiAvatar>
                      <div>
                        <div class="font-medium">{{ user.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ user.username }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">{{ user.email }}</td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getRoleVariant(user.role)">
                      {{ user.role }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="getStatusVariant(user.status)">
                      {{ user.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ user.lastLogin }}</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Edit class="w-4 h-4" />
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

      <!-- 分页 -->
      <div class="flex items-center justify-between">
        <div class="text-sm text-muted-foreground">显示 1-10 条，共 150 条记录</div>
        <div class="flex items-center space-x-2">
          <UiButton variant="outline" size="sm" disabled> 上一页 </UiButton>
          <UiButton variant="outline" size="sm"> 1 </UiButton>
          <UiButton variant="outline" size="sm"> 2 </UiButton>
          <UiButton variant="outline" size="sm"> 3 </UiButton>
          <UiButton variant="outline" size="sm"> 下一页 </UiButton>
        </div>
      </div>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
import { UserPlus, Search, Edit, Trash2 } from 'lucide-vue-next'

const users = ref([
  {
    id: 1,
    name: '张三',
    username: 'zhangsan',
    email: 'zhangsan@example.com',
    avatar: '',
    role: '管理员',
    status: '活跃',
    lastLogin: '2024-01-15 14:30',
  },
  {
    id: 2,
    name: '李四',
    username: 'lisi',
    email: 'lisi@example.com',
    avatar: '',
    role: '编辑',
    status: '活跃',
    lastLogin: '2024-01-14 09:15',
  },
  {
    id: 3,
    name: '王五',
    username: 'wangwu',
    email: 'wangwu@example.com',
    avatar: '',
    role: '查看者',
    status: '未激活',
    lastLogin: '2024-01-10 16:45',
  },
  {
    id: 4,
    name: '赵六',
    username: 'zhaoliu',
    email: 'zhaoliu@example.com',
    avatar: '',
    role: '管理员',
    status: '活跃',
    lastLogin: '2024-01-15 10:20',
  },
  {
    id: 5,
    name: '钱七',
    username: 'qianqi',
    email: 'qianqi@example.com',
    avatar: '',
    role: '编辑',
    status: '已锁定',
    lastLogin: '2024-01-08 11:30',
  },
])

function getRoleVariant(role: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    管理员: 'destructive',
    编辑: 'default',
    查看者: 'secondary',
  }
  return variants[role] || 'outline'
}

function getStatusVariant(status: string) {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    活跃: 'default',
    未激活: 'outline',
    已锁定: 'destructive',
  }
  return variants[status] || 'outline'
}
</script>

<style scoped></style>

<route lang="yaml">
meta:
  auth: true
</route>
