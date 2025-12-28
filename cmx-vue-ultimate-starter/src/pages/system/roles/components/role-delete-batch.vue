<script setup lang="ts">
import type { Table } from '@tanstack/vue-table'
import { AlertTriangle } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useQueryClient } from '@tanstack/vue-query'

import type { Role } from '@/services/types/acs/Role'
import { deleteRole } from '@/services/api/rpc/role'

interface RoleDeleteBatchProps {
  table: Table<Role>
}

const props = defineProps<RoleDeleteBatchProps>()
const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const queryClient = useQueryClient()

const selectedRoles = computed(() => {
  return props.table.getFilteredSelectedRowModel().rows.map(row => row.original)
})

async function onConfirm() {
  loading.value = true
  try {
    // Delete all selected roles
    const deletePromises = selectedRoles.value.map(role =>
      deleteRole({ id: role.id })
    )
    await Promise.all(deletePromises)

    toast.success(`成功删除 ${selectedRoles.value.length} 个角色`)
    props.table.resetRowSelection()
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
          <UiDialogTitle>批量删除角色</UiDialogTitle>
        </div>
        <UiDialogDescription>
          确定要删除选中的 {{ selectedRoles.length }} 个角色吗？此操作无法撤销。
        </UiDialogDescription>
      </UiDialogHeader>

      <div class="max-h-[200px] overflow-y-auto py-2">
        <ul class="space-y-1 text-sm">
          <li v-for="role in selectedRoles" :key="String(role.id)" class="text-muted-foreground">
            • {{ role.display_name || role.name }}
          </li>
        </ul>
      </div>

      <UiDialogFooter>
        <UiButton variant="outline" @click="open = false" :disabled="loading">
          取消
        </UiButton>
        <UiButton variant="destructive" @click="onConfirm" :disabled="loading">
          <span v-if="loading">删除中...</span>
          <span v-else>确认删除</span>
        </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
