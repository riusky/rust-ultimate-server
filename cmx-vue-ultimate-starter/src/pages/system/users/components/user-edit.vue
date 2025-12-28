<script setup lang="ts">
import type { UserWithInfo } from '@/services/types/user/index'

import { useQueryClient } from '@tanstack/vue-query'
import { toast } from 'vue-sonner'

import {
  createUserInfo,
  updateUserInfo,
  type UserInfoForUpdate,
} from '@/services/api/rpc/userinfo/user-info'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

import { genders, statuses } from './data'

interface Props {
  user: UserWithInfo
}

const props = defineProps<Props>()
const open = defineModel<boolean>('open', { required: true })

const queryClient = useQueryClient()
const isLoading = ref(false)

const form = ref<UserInfoForUpdate>({
  nickname: props.user.nickname,
  email: props.user.email,
  phone: props.user.phone,
  gender: props.user.gender as 'Unknown' | 'Male' | 'Female' | undefined,
  city: props.user.city,
  status: props.user.status as 'Active' | 'Inactive' | 'Suspended' | 'Deleted' | undefined,
})

watch(
  () => props.user,
  (newVal) => {
    form.value = {
      nickname: newVal.nickname,
      email: newVal.email,
      phone: newVal.phone,
      gender: newVal.gender as 'Unknown' | 'Male' | 'Female' | undefined,
      city: newVal.city,
      status: newVal.status as 'Active' | 'Inactive' | 'Suspended' | 'Deleted' | undefined,
    }
  }
)

async function handleSubmit() {
  isLoading.value = true
  try {
    // If user has user_info_id, update it. Otherwise create new user_info
    if (props.user.user_info_id) {
      await updateUserInfo({
        id: Number(props.user.user_info_id),
        data: form.value,
      })
    } else {
      await createUserInfo({
        data: {
          user_id: Number(props.user.id),
          ...form.value,
        },
      })
    }
    toast.success('用户信息已更新')
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
        <DialogTitle>编辑用户</DialogTitle>
        <DialogDescription> 修改用户信息后点击保存。 </DialogDescription>
      </DialogHeader>

      <form class="grid gap-4 py-4" @submit.prevent="handleSubmit">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="nickname" class="text-right">昵称</Label>
          <Input id="nickname" v-model="form.nickname as string" class="col-span-3" />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="email" class="text-right">邮箱</Label>
          <Input id="email" v-model="form.email as string" type="email" class="col-span-3" />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="phone" class="text-right">手机</Label>
          <Input id="phone" v-model="form.phone as string" class="col-span-3" />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label class="text-right">性别</Label>
          <Select v-model="form.gender">
            <SelectTrigger class="col-span-3">
              <SelectValue placeholder="选择性别" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="g in genders" :key="g.value" :value="g.value">
                {{ g.label }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="city" class="text-right">城市</Label>
          <Input id="city" v-model="form.city as string" class="col-span-3" />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label class="text-right">状态</Label>
          <Select v-model="form.status">
            <SelectTrigger class="col-span-3">
              <SelectValue placeholder="选择状态" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="s in statuses" :key="s.value" :value="s.value">
                {{ s.label }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">取消</Button>
          <Button type="submit" :disabled="isLoading">
            {{ isLoading ? '保存中...' : '保存' }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
