<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { toast } from 'vue-sonner'

import { createUser } from '@/services/api/rpc/user/user'
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

const open = defineModel<boolean>('open', { required: true })

const queryClient = useQueryClient()
const isLoading = ref(false)

const form = ref({
  username: '',
  password: '',
  confirmPassword: '',
})

const passwordError = computed(() => {
  if (form.value.password && form.value.confirmPassword) {
    if (form.value.password !== form.value.confirmPassword) {
      return 'Passwords do not match'
    }
  }
  return ''
})

function resetForm() {
  form.value = {
    username: '',
    password: '',
    confirmPassword: '',
  }
}

async function handleSubmit() {
  if (!form.value.username || !form.value.password) {
    toast.error('Please fill in all required fields')
    return
  }

  if (passwordError.value) {
    toast.error(passwordError.value)
    return
  }

  isLoading.value = true
  try {
    await createUser({
      username: form.value.username,
      password: form.value.password,
    })
    toast.success('User created successfully')
    queryClient.invalidateQueries({ queryKey: ['users'] })
    open.value = false
    resetForm()
  } finally {
    isLoading.value = false
  }
}

watch(open, (val) => {
  if (!val) {
    resetForm()
  }
})
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>Add User</DialogTitle>
        <DialogDescription>
          Create a new user account. User can update their profile later.
        </DialogDescription>
      </DialogHeader>

      <form class="grid gap-4 py-4" @submit.prevent="handleSubmit">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="username" class="text-right">Username</Label>
          <Input
            id="username"
            v-model="form.username"
            class="col-span-3"
            placeholder="Enter username"
            required
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="password" class="text-right">Password</Label>
          <Input
            id="password"
            v-model="form.password"
            type="password"
            class="col-span-3"
            placeholder="Enter password"
            required
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="confirmPassword" class="text-right">Confirm</Label>
          <div class="col-span-3 space-y-1">
            <Input
              id="confirmPassword"
              v-model="form.confirmPassword"
              type="password"
              placeholder="Confirm password"
              required
              :class="{ 'border-destructive': passwordError }"
            />
            <p v-if="passwordError" class="text-xs text-destructive">
              {{ passwordError }}
            </p>
          </div>
        </div>

        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">
            Cancel
          </Button>
          <Button type="submit" :disabled="isLoading || !!passwordError">
            {{ isLoading ? 'Creating...' : 'Create User' }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
