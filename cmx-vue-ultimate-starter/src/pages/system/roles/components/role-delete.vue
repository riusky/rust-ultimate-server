<script setup lang="ts">
import { AlertTriangle } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useQueryClient } from '@tanstack/vue-query'

import type { Role } from '@/services/types/acs/Role'
import { deleteRole } from '@/services/api/rpc/role'

interface RoleDeleteProps {
  role: Role
}

const props = defineProps<RoleDeleteProps>()
const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const queryClient = useQueryClient()

async function onConfirm() {
  loading.value = true
  try {
    await deleteRole({ id: props.role.id })
    toast.success('角色删除成功')
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
    <UiDialogContent class="sm:max-w-[425px]">
      <UiDialogHeader>
        <div class="flex items-center gap-2">
          <AlertTriangle class="h-5 w-5 text-destructive" />
          <UiDialogTitle>删除角色</UiDialogTitle>
        </div>
        <UiDialogDescription>
          确定要删除角色 "{{ props.role.display_name || props.role.name }}" 吗？此操作无法撤销。
        </UiDialogDescription>
      </UiDialogHeader>

      <UiDialogFooter>
        <UiButton variant="outline" @click="open = false" :disabled="loading">
          取消
        </UiButton>
        <UiButton variant="destructive" @click="onConfirm" :disabled="loading">
          <span v-if="loading">删除中...</span>
          <span v-else>删除</span>
        </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
