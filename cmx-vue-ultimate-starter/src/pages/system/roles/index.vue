<template>
  <Page title="角色管理" description="管理系统角色和权限分配" sticky>
    <template #actions>
      <UiButton>
        <UserPlus class="w-4 h-4 mr-2" />
        添加角色
      </UiButton>
    </template>

    <div class="space-y-4">
      <!-- 角色统计 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">总角色数</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">8</div>
            <p class="text-xs text-muted-foreground">系统中共有8个角色</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">活跃角色</UiCardTitle>
            <UserCheck class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">6</div>
            <p class="text-xs text-muted-foreground">6个角色正在使用</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">用户数</UiCardTitle>
            <Users class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">150</div>
            <p class="text-xs text-muted-foreground">分配了角色的用户</p>
          </UiCardContent>
        </UiCard>
        <UiCard>
          <UiCardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
            <UiCardTitle class="text-sm font-medium">权限数</UiCardTitle>
            <Shield class="h-4 w-4 text-muted-foreground" />
          </UiCardHeader>
          <UiCardContent>
            <div class="text-2xl font-bold">42</div>
            <p class="text-xs text-muted-foreground">系统总权限数量</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 角色列表 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>角色列表</UiCardTitle>
          <UiCardDescription>管理系统中的所有角色及其权限</UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="p-0">
          <div class="relative overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs uppercase bg-muted">
                <tr>
                  <th scope="col" class="px-6 py-3">角色名称</th>
                  <th scope="col" class="px-6 py-3">描述</th>
                  <th scope="col" class="px-6 py-3">用户数</th>
                  <th scope="col" class="px-6 py-3">权限数</th>
                  <th scope="col" class="px-6 py-3">状态</th>
                  <th scope="col" class="px-6 py-3">创建时间</th>
                  <th scope="col" class="px-6 py-3">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="role in roles" :key="role.id" class="border-b hover:bg-muted/50">
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center"
                      >
                        <Shield class="w-4 h-4 text-primary" />
                      </div>
                      <div>
                        <div class="font-medium">{{ role.name }}</div>
                        <div class="text-sm text-muted-foreground">{{ role.code }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-4">{{ role.description }}</td>
                  <td class="px-6 py-4">
                    <span class="font-medium">{{ role.userCount }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <span class="font-medium">{{ role.permissionCount }}</span>
                  </td>
                  <td class="px-6 py-4">
                    <UiBadge :variant="role.status === '启用' ? 'default' : 'outline'">
                      {{ role.status }}
                    </UiBadge>
                  </td>
                  <td class="px-6 py-4">{{ role.createdAt }}</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center space-x-2">
                      <UiButton variant="ghost" size="sm">
                        <Edit class="w-4 h-4" />
                      </UiButton>
                      <UiButton variant="ghost" size="sm">
                        <Trash2 class="w-4 h-4" />
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

      <!-- 权限分配示例 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>权限分配示例</UiCardTitle>
          <UiCardDescription>管理员角色的权限配置</UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="space-y-2">
              <h4 class="font-medium">用户管理</h4>
              <div class="space-y-1">
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">查看用户</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">创建用户</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">编辑用户</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">删除用户</span>
                </div>
              </div>
            </div>
            <div class="space-y-2">
              <h4 class="font-medium">角色管理</h4>
              <div class="space-y-1">
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">查看角色</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">创建角色</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">编辑角色</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">删除角色</span>
                </div>
              </div>
            </div>
            <div class="space-y-2">
              <h4 class="font-medium">系统设置</h4>
              <div class="space-y-1">
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">查看设置</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">修改设置</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox />
                  <span class="text-sm">系统重启</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox />
                  <span class="text-sm">系统备份</span>
                </div>
              </div>
            </div>
            <div class="space-y-2">
              <h4 class="font-medium">监控管理</h4>
              <div class="space-y-1">
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">查看监控</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox checked />
                  <span class="text-sm">告警设置</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox />
                  <span class="text-sm">日志管理</span>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox />
                  <span class="text-sm">性能分析</span>
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
import { UserPlus, Users, UserCheck, Shield, Edit, Trash2, Settings } from 'lucide-vue-next'

const roles = ref([
  {
    id: 1,
    name: '超级管理员',
    code: 'super_admin',
    description: '拥有系统所有权限',
    userCount: 2,
    permissionCount: 42,
    status: '启用',
    createdAt: '2023-01-15',
  },
  {
    id: 2,
    name: '系统管理员',
    code: 'admin',
    description: '系统管理权限',
    userCount: 5,
    permissionCount: 35,
    status: '启用',
    createdAt: '2023-02-20',
  },
  {
    id: 3,
    name: '内容编辑',
    code: 'editor',
    description: '内容编辑和管理权限',
    userCount: 25,
    permissionCount: 18,
    status: '启用',
    createdAt: '2023-03-10',
  },
  {
    id: 4,
    name: '查看者',
    code: 'viewer',
    description: '只读查看权限',
    userCount: 118,
    permissionCount: 12,
    status: '启用',
    createdAt: '2023-04-05',
  },
  {
    id: 5,
    name: '审核员',
    code: 'auditor',
    description: '内容审核权限',
    userCount: 8,
    permissionCount: 15,
    status: '启用',
    createdAt: '2023-05-12',
  },
  {
    id: 6,
    name: '运维人员',
    code: 'operator',
    description: '系统运维权限',
    userCount: 3,
    permissionCount: 28,
    status: '启用',
    createdAt: '2023-06-18',
  },
  {
    id: 7,
    name: '测试角色',
    code: 'tester',
    description: '测试环境使用',
    userCount: 0,
    permissionCount: 8,
    status: '禁用',
    createdAt: '2023-07-22',
  },
  {
    id: 8,
    name: '访客',
    code: 'guest',
    description: '访客权限',
    userCount: 0,
    permissionCount: 5,
    status: '禁用',
    createdAt: '2023-08-30',
  },
])
</script>

<style scoped></style>
