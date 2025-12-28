<script setup lang="ts">
import { Shield } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useQueryClient, useQuery } from '@tanstack/vue-query'

import type { Role } from '@/services/types/acs/Role'
import type { Permission } from '@/services/types/acs/Permission'

import {
  listPermissionsForRole,
  setPermissionsForRole,
  listAllPermissions,
} from '@/services/api/rpc/role/role-permission'

interface RolePermissionsProps {
  role: Role
}

const props = defineProps<RolePermissionsProps>()
const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const selectedPermissionIds = ref<bigint[]>([])
const queryClient = useQueryClient()

// 获取所有权限列表
const { data: allPermissions, isLoading: permissionsLoading } = useQuery({
  queryKey: ['permissions'],
  queryFn: () => listAllPermissions(),
})

// 按组名分组权限
const groupedPermissions = computed(() => {
  if (!allPermissions.value) return {}

  const groups: Record<string, Permission[]> = {}
  for (const perm of allPermissions.value) {
    const groupName = perm.group_name || '未分组'
    if (!groups[groupName]) {
      groups[groupName] = []
    }
    groups[groupName].push(perm)
  }
  return groups
})

// 当弹窗打开时，获取角色当前的权限
watch(open, async (isOpen) => {
  if (isOpen) {
    try {
      const rolePermissions = await listPermissionsForRole({ role_id: props.role.id })
      selectedPermissionIds.value = rolePermissions.map(p => p.id)
    }
    catch (error) {
      // Error handled by RPC client
    }
  }
})

function isPermissionSelected(permissionId: bigint): boolean {
  return selectedPermissionIds.value.includes(permissionId)
}

function togglePermission(permissionId: bigint) {
  if (isPermissionSelected(permissionId)) {
    selectedPermissionIds.value = selectedPermissionIds.value.filter(id => id !== permissionId)
  }
  else {
    selectedPermissionIds.value = [...selectedPermissionIds.value, permissionId]
  }
}

// 全选/取消全选某个分组
function toggleGroup(groupName: string) {
  const groupPerms = groupedPermissions.value[groupName] || []
  const groupIds = groupPerms.map(p => p.id)
  const allSelected = groupIds.every(id => isPermissionSelected(id))

  if (allSelected) {
    // 取消全选
    selectedPermissionIds.value = selectedPermissionIds.value.filter(
      id => !groupIds.includes(id)
    )
  }
  else {
    // 全选
    const newIds = new Set([...selectedPermissionIds.value, ...groupIds])
    selectedPermissionIds.value = Array.from(newIds)
  }
}

function isGroupAllSelected(groupName: string): boolean {
  const groupPerms = groupedPermissions.value[groupName] || []
  return groupPerms.length > 0 && groupPerms.every(p => isPermissionSelected(p.id))
}

function isGroupPartiallySelected(groupName: string): boolean {
  const groupPerms = groupedPermissions.value[groupName] || []
  const selectedCount = groupPerms.filter(p => isPermissionSelected(p.id)).length
  return selectedCount > 0 && selectedCount < groupPerms.length
}

async function onSubmit() {
  loading.value = true
  try {
    await setPermissionsForRole({
      role_id: props.role.id,
      permission_ids: selectedPermissionIds.value,
    })

    toast.success('角色权限更新成功')
    open.value = false
    queryClient.invalidateQueries({ queryKey: ['roles'] })
  }
  catch (error: any) {
    // Error handled by RPC client
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <UiDialog v-model:open="open">
    <UiDialogContent class="sm:max-w-[700px] max-h-[80vh] flex flex-col">
      <UiDialogHeader>
        <UiDialogTitle class="flex items-center gap-2">
          <Shield class="size-5" />
          管理权限
        </UiDialogTitle>
        <UiDialogDescription>
          为角色 "{{ props.role.display_name || props.role.name }}" 分配权限
        </UiDialogDescription>
      </UiDialogHeader>

      <div class="py-4 flex-1 overflow-y-auto">
        <div v-if="permissionsLoading" class="flex items-center justify-center py-8">
          <UiLoading />
        </div>
        <div v-else-if="Object.keys(groupedPermissions).length > 0" class="space-y-4">
          <!-- 按组显示权限 -->
          <div
            v-for="(permissions, groupName) in groupedPermissions"
            :key="groupName"
            class="border rounded-lg"
          >
            <!-- 组标题 -->
            <div
              class="flex items-center space-x-3 p-3 bg-muted/50 rounded-t-lg cursor-pointer hover:bg-muted transition-colors"
              @click="toggleGroup(groupName as string)"
            >
              <UiCheckbox
                :model-value="isGroupAllSelected(groupName as string)"
                :indeterminate="isGroupPartiallySelected(groupName as string)"
                @update:model-value="toggleGroup(groupName as string)"
              />
              <span class="font-medium text-sm">{{ groupName }}</span>
              <span class="text-xs text-muted-foreground">
                ({{ permissions.filter(p => isPermissionSelected(p.id)).length }}/{{ permissions.length }})
              </span>
            </div>

            <!-- 组内权限列表 -->
            <div class="p-3 grid grid-cols-1 md:grid-cols-2 gap-2">
              <div
                v-for="permission in permissions"
                :key="String(permission.id)"
                class="flex items-center space-x-2 p-2 rounded cursor-pointer hover:bg-muted/30 transition-colors"
                :class="{ 'bg-primary/5': isPermissionSelected(permission.id) }"
                @click="togglePermission(permission.id)"
              >
                <UiCheckbox
                  :model-value="isPermissionSelected(permission.id)"
                  @update:model-value="togglePermission(permission.id)"
                />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">
                    {{ permission.display_name || permission.key }}
                  </p>
                  <p v-if="permission.description" class="text-xs text-muted-foreground truncate">
                    {{ permission.description }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="text-center py-8 text-muted-foreground">
          暂无可用权限
        </div>
      </div>

      <UiDialogFooter class="border-t pt-4">
        <div class="flex-1 text-sm text-muted-foreground">
          已选择 {{ selectedPermissionIds.length }} 个权限
        </div>
        <UiButton variant="outline" :disabled="loading" @click="open = false">
          取消
        </UiButton>
        <UiButton :disabled="loading || permissionsLoading" @click="onSubmit">
          <template v-if="loading">
            保存中...
          </template>
          <template v-else>
            保存
          </template>
        </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
