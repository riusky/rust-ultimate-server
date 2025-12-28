<script setup lang="ts">
import type { Table } from '@tanstack/vue-table'
import type { UserWithInfo } from '@/services/types/user/index'

import { useQueryClient } from '@tanstack/vue-query'
import { toast } from 'vue-sonner'

import { deleteUserInfo } from '@/services/api/rpc/userinfo/user-info'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'

interface Props {
  table: Table<UserWithInfo>
}

const props = defineProps<Props>()
const open = defineModel<boolean>('open', { required: true })

const queryClient = useQueryClient()
const isLoading = ref(false)

const selectedRows = computed(() => props.table.getFilteredSelectedRowModel().rows)
const deletableRows = computed(() =>
  selectedRows.value.filter((row) => row.original.user_info_id)
)

async function handleDelete() {
  isLoading.value = true
  try {
    const ids = deletableRows.value.map((row) => Number(row.original.user_info_id))
    await Promise.all(ids.map((id) => deleteUserInfo({ id })))
    toast.success(`已删除 ${ids.length} 个用户信息`)
    props.table.resetRowSelection()
    queryClient.invalidateQueries({ queryKey: ['users'] })
    open.value = false
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <AlertDialog v-model:open="open">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>确认批量删除</AlertDialogTitle>
        <AlertDialogDescription>
          确定要删除选中的 {{ deletableRows.length }} 个用户信息吗？此操作无法撤销。
          <span v-if="selectedRows.length !== deletableRows.length" class="block mt-2 text-warning">
            注意: 有 {{ selectedRows.length - deletableRows.length }} 个用户没有信息记录，将被跳过。
          </span>
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>取消</AlertDialogCancel>
        <AlertDialogAction
          class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
          :disabled="isLoading || deletableRows.length === 0"
          @click="handleDelete"
        >
          {{ isLoading ? '删除中...' : '删除' }}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
