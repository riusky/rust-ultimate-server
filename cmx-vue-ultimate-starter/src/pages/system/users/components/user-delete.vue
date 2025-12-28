<script setup lang="ts">
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
  user: UserWithInfo
}

const props = defineProps<Props>()
const open = defineModel<boolean>('open', { required: true })

const queryClient = useQueryClient()
const isLoading = ref(false)

async function handleDelete() {
  if (!props.user.user_info_id) {
    toast.error('该用户没有信息记录')
    return
  }
  isLoading.value = true
  try {
    await deleteUserInfo({ id: Number(props.user.user_info_id) })
    toast.success('用户信息已删除')
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
        <AlertDialogTitle>确认删除</AlertDialogTitle>
        <AlertDialogDescription>
          确定要删除用户 "{{ props.user.nickname || props.user.username }}" 的信息吗？此操作无法撤销。
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>取消</AlertDialogCancel>
        <AlertDialogAction
          class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
          :disabled="isLoading || !props.user.user_info_id"
          @click="handleDelete"
        >
          {{ isLoading ? '删除中...' : '删除' }}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
