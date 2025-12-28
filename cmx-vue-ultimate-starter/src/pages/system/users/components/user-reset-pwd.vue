<script setup lang="ts">
import type { UserWithInfo } from '@/services/types/user/index'

import { useQueryClient } from '@tanstack/vue-query'
import { toast } from 'vue-sonner'

import { resetUserPwd } from '@/services/api/rpc/user/user'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

interface Props {
  user: UserWithInfo
}

const props = defineProps<Props>()
const open = defineModel<boolean>('open', { required: true })

const queryClient = useQueryClient()
const isLoading = ref(false)
const newPassword = ref('')
const confirmPassword = ref('')

watch(open, (val) => {
  if (!val) {
    newPassword.value = ''
    confirmPassword.value = ''
  }
})

async function handleSubmit() {
  if (newPassword.value !== confirmPassword.value) {
    toast.error('两次输入的密码不一致')
    return
  }
  if (newPassword.value.length < 6) {
    toast.error('密码长度至少6位')
    return
  }

  isLoading.value = true
  try {
    await resetUserPwd({
      user_id: Number(props.user.id),
      new_password: newPassword.value,
    })
    toast.success('密码已重置')
    queryClient.invalidateQueries({ queryKey: ['users'] })
    open.value = false
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>重置密码</DialogTitle>
        <DialogDescription>
          为用户 "{{ props.user.nickname || props.user.username }}" 设置新密码。
        </DialogDescription>
      </DialogHeader>

      <form class="grid gap-4 py-4" @submit.prevent="handleSubmit">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="newPassword" class="text-right">新密码</Label>
          <Input
            id="newPassword"
            v-model="newPassword"
            type="password"
            class="col-span-3"
            placeholder="请输入新密码"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="confirmPassword" class="text-right">确认密码</Label>
          <Input
            id="confirmPassword"
            v-model="confirmPassword"
            type="password"
            class="col-span-3"
            placeholder="请再次输入密码"
          />
        </div>

        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">取消</Button>
          <Button type="submit" :disabled="isLoading">
            {{ isLoading ? '重置中...' : '重置密码' }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
