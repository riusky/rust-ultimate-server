<template>
  <Page title="权限分配" description="管理系统权限和访问控制配置" sticky>
    <template #actions>
      <UiButton>
        <Plus class="w-4 h-4 mr-2" />
        添加权限
      </UiButton>
      <UiButton variant="outline">
        <Download class="w-4 h-4 mr-2" />
        导出权限
      </UiButton>
    </template>

    <div class="space-y-6">
      <!-- 权限概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总权限数</UiCardTitle>
            <Shield class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">42</div>
            <p class="text-xs text-muted-foreground">系统权限总数</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">已分配</UiCardTitle>
            <UserCheck class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-2">38</div>
            <p class="text-xs text-muted-foreground">已分配的权限</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">未使用</UiCardTitle>
            <UserX class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-4">4</div>
            <p class="text-xs text-muted-foreground">未使用的权限</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">权限组</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold text-chart-3">6</div>
            <p class="text-xs text-muted-foreground">权限分组数量</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 权限列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>权限列表</UiCardTitle>
          <UiCardDescription>系统所有权限配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">权限名称</th>
                  <th scope="col" class="px-6 py-3">权限代码</th>
                  <th scope="col" class="px-6 py-3">类型</th>
                  <th scope="col" class="px-6 py-3">分配角色</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">创建时间</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="permission in permissions"
                  :key="permission.id"
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
                        <div class="font-medium">{{ permission.name }}</div>
                        <div class="text-sm text-muted-foreground">
                          {{ permission.description }}
                        </div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <code class="text-xs bg-muted px-2 py-1 rounded">{{ permission.code }}</code>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge variant="outline">{{ permission.type }}</UiBadge>
                  </td>
                  <td class="px-6 py-4">
                    <span class="font-medium">{{ permission.assignedRoles }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="permission.status === '启用' ? 'default' : 'outline'">
                      {{ permission.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ permission.createdAt }}</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Edit class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Users class="w-4 h-4" />
                      </UiButton>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 权限分配 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>权限分配</UiCardTitle>
          <UiCardDescription>为角色分配系统权限</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- 角色选择 -->
            <div class="space-y-4">
              <h3 class="font-medium">选择角色</h3>
              <div class="space-y-2">
                <div
                  v-for="role in roles"
                  :key="role.id"
                  class="flex items-center space-x-3 p-3 border rounded-lg cursor-pointer hover:bg-muted/50"
                  :class="{ 'border-primary bg-primary/5': selectedRole?.id === role.id }"
                  @click="selectedRole = role"
                >
                  <div class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                    <Users class="w-4 h-4 text-primary" />
                  </div>
                  <div class="flex-1">
                    <div class="font-medium">{{ role.name }}</div>
                    <div class="text-sm text-muted-foreground">{{ role.description }}</div>
                  </div>
                  <UiBadge variant="outline">{{ role.permissionCount }} 权限</UiBadge>
                </div>
              </div>
            </div>

            <!-- 权限配置 -->
            <div class="space-y-4" v-if="selectedRole">
              <div class="flex items-center justify-between">
                <h3 class="font-medium">{{ selectedRole.name }} 权限配置</h3>
                <UiButton size="sm" @click="savePermissions">
                  <Save class="w-4 h-4 mr-2" />
                  保存配置
                </UiButton>
              </div>
              <div class="space-y-3 max-h-96 overflow-y-auto">
                <div
                  v-for="category in permissionCategories"
                  :key="category.name"
                  class="space-y-2"
                >
                  <h4 class="font-medium text-sm">{{ category.name }}</h4>
                  <div class="space-y-1">
                    <div
                      v-for="perm in category.permissions"
                      :key="perm.id"
                      class="flex items-center space-x-2 p-2 rounded hover:bg-muted/50"
                    >
                      <UiCheckbox
                        :id="`perm-${perm.id}`"
                        :checked="selectedRolePermissions.includes(perm.id)"
                        @update:checked="togglePermission(perm.id)"
                      />
                      <UiLabel :for="`perm-${perm.id}`" class="flex-1 cursor-pointer">
                        <div class="text-sm">{{ perm.name }}</div>
                        <div class="text-xs text-muted-foreground">{{ perm.description }}</div>
                      </UiLabel>
                    </div>
                  </div>
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
import { Plus, Download, Shield, UserCheck, UserX, Users, Edit, Save } from 'lucide-vue-next'

const selectedRole = ref<{
  id: string
  name: string
  description: string
  permissionCount: number
} | null>(null)
const selectedRolePermissions = ref<string[]>([])

const permissions = ref([
  {
    id: '1',
    name: '用户管理',
    code: 'user:read',
    description: '查看用户信息',
    type: '读取',
    assignedRoles: 3,
    status: '启用',
    createdAt: '2023-01-15',
  },
  {
    id: '2',
    name: '用户编辑',
    code: 'user:write',
    description: '编辑用户信息',
    type: '写入',
    assignedRoles: 2,
    status: '启用',
    createdAt: '2023-01-15',
  },
  {
    id: '3',
    name: '用户删除',
    code: 'user:delete',
    description: '删除用户账户',
    type: '删除',
    assignedRoles: 1,
    status: '启用',
    createdAt: '2023-01-15',
  },
  {
    id: '4',
    name: '角色管理',
    code: 'role:read',
    description: '查看角色信息',
    type: '读取',
    assignedRoles: 2,
    status: '启用',
    createdAt: '2023-02-20',
  },
  {
    id: '5',
    name: '角色编辑',
    code: 'role:write',
    description: '编辑角色配置',
    type: '写入',
    assignedRoles: 1,
    status: '启用',
    createdAt: '2023-02-20',
  },
  {
    id: '6',
    name: '系统设置',
    code: 'system:read',
    description: '查看系统设置',
    type: '读取',
    assignedRoles: 2,
    status: '启用',
    createdAt: '2023-03-10',
  },
])

const roles = ref([
  {
    id: '1',
    name: '超级管理员',
    description: '拥有系统所有权限',
    permissionCount: 42,
  },
  {
    id: '2',
    name: '系统管理员',
    description: '系统管理权限',
    permissionCount: 35,
  },
  {
    id: '3',
    name: '内容编辑',
    description: '内容编辑和管理权限',
    permissionCount: 18,
  },
  {
    id: '4',
    name: '查看者',
    description: '只读查看权限',
    permissionCount: 12,
  },
])

const permissionCategories = ref([
  {
    name: '用户管理',
    permissions: [
      { id: '1', name: '查看用户', description: '查看用户列表和详情' },
      { id: '2', name: '创建用户', description: '创建新用户账户' },
      { id: '3', name: '编辑用户', description: '修改用户信息' },
      { id: '4', name: '删除用户', description: '删除用户账户' },
    ],
  },
  {
    name: '角色管理',
    permissions: [
      { id: '5', name: '查看角色', description: '查看角色列表和详情' },
      { id: '6', name: '创建角色', description: '创建新角色' },
      { id: '7', name: '编辑角色', description: '修改角色配置' },
      { id: '8', name: '删除角色', description: '删除角色' },
    ],
  },
  {
    name: '系统设置',
    permissions: [
      { id: '9', name: '查看设置', description: '查看系统设置' },
      { id: '10', name: '修改设置', description: '修改系统配置' },
      { id: '11', name: '系统重启', description: '重启系统服务' },
      { id: '12', name: '系统备份', description: '执行系统备份' },
    ],
  },
])

function togglePermission(permissionId: string) {
  const index = selectedRolePermissions.value.indexOf(permissionId)
  if (index > -1) {
    selectedRolePermissions.value.splice(index, 1)
  } else {
    selectedRolePermissions.value.push(permissionId)
  }
}

function savePermissions() {
  // 模拟保存权限配置
  console.log('保存权限配置:', {
    role: selectedRole.value?.name,
    permissions: selectedRolePermissions.value,
  })
}
</script>

<style scoped></style>
