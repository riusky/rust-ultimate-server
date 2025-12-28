<script setup lang="ts">
import { Shield } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useQueryClient, useQuery } from '@tanstack/vue-query'

import type { UserWithInfo } from '@/services/types/user/index'
import type { Role } from '@/services/types/acs/Role'

import { listRolesForUser, setRolesForUser } from '@/services/api/rpc/user/user-role'
import { listRoles } from '@/services/api/rpc/role'

interface UserRolesProps {
  user: UserWithInfo
}

const props = defineProps<UserRolesProps>()
const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const selectedRoleIds = ref<bigint[]>([])
const queryClient = useQueryClient()

// 获取所有角色列表
const { data: allRoles, isLoading: rolesLoading } = useQuery({
  queryKey: ['roles'],
  queryFn: () => listRoles(),
})

// 当弹窗打开时，获取用户当前的角色
watch(open, async (isOpen) => {
  if (isOpen) {
    try {
      const userRoles = await listRolesForUser({ user_id: props.user.id })
      selectedRoleIds.value = userRoles.map(r => r.id)
    }
    catch (error) {
      // Error handled by RPC client
    }
  }
})

function isRoleSelected(roleId: bigint): boolean {
  return selectedRoleIds.value.includes(roleId)
}

function toggleRole(roleId: bigint) {
  if (isRoleSelected(roleId)) {
    selectedRoleIds.value = selectedRoleIds.value.filter(id => id !== roleId)
  }
  else {
    selectedRoleIds.value = [...selectedRoleIds.value, roleId]
  }
}

async function onSubmit() {
  loading.value = true
  try {
    await setRolesForUser({
      user_id: props.user.id,
      role_ids: selectedRoleIds.value,
    })

    toast.success('用户角色更新成功')
    open.value = false
    queryClient.invalidateQueries({ queryKey: ['users'] })
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
    <UiDialogContent class="sm:max-w-[500px]">
      <UiDialogHeader>
        <UiDialogTitle class="flex items-center gap-2">
          <Shield class="size-5" />
          分配角色
        </UiDialogTitle>
        <UiDialogDescription>
          为用户 "{{ props.user.username }}" 分配角色
        </UiDialogDescription>
      </UiDialogHeader>

      <div class="py-4">
        <div v-if="rolesLoading" class="flex items-center justify-center py-8">
          <UiLoading />
        </div>
        <div v-else-if="allRoles && allRoles.length > 0" class="space-y-2">
          <div
            v-for="role in allRoles"
            :key="String(role.id)"
            class="flex items-center space-x-3 rounded-lg border p-3 cursor-pointer hover:bg-muted/50 transition-colors"
            :class="{ 'bg-primary/10 border-primary': isRoleSelected(role.id) }"
            @click="toggleRole(role.id)"
          >
            <UiCheckbox
              :model-value="isRoleSelected(role.id)"
              @update:model-value="toggleRole(role.id)"
            />
            <div class="flex-1 space-y-1">
              <p class="text-sm font-medium leading-none">
                {{ role.display_name || role.name }}
              </p>
              <p v-if="role.description" class="text-xs text-muted-foreground">
                {{ role.description }}
              </p>
            </div>
          </div>
        </div>
        <div v-else class="text-center py-8 text-muted-foreground">
          暂无可用角色
        </div>
      </div>

      <UiDialogFooter>
        <UiButton variant="outline" :disabled="loading" @click="open = false">
          取消
        </UiButton>
        <UiButton :disabled="loading || rolesLoading" @click="onSubmit">
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
