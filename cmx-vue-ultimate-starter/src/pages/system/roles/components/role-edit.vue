<script setup lang="ts">
import { toast } from 'vue-sonner'
import { useQueryClient } from '@tanstack/vue-query'

import type { Role } from '@/services/types/acs/Role'
import { updateRole } from '@/services/api/rpc/role'

interface RoleEditProps {
  role: Role
}

const props = defineProps<RoleEditProps>()
const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const queryClient = useQueryClient()

const form = reactive({
  name: '',
  display_name: '',
  description: '',
})

watch(() => props.role, (newRole) => {
  if (newRole) {
    form.name = newRole.name
    form.display_name = newRole.display_name || ''
    form.description = newRole.description || ''
  }
}, { immediate: true })

async function onSubmit() {
  if (!form.name.trim()) {
    toast.error('角色名称不能为空')
    return
  }

  loading.value = true
  try {
    await updateRole({
      id: props.role.id,
      data: {
        name: form.name.trim(),
        display_name: form.display_name.trim() || null,
        description: form.description.trim() || null,
      },
    })

    toast.success('角色更新成功')
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
    <UiDialogContent class="sm:max-w-[500px]">
      <UiDialogHeader>
        <UiDialogTitle>编辑角色</UiDialogTitle>
        <UiDialogDescription>
          修改角色信息
        </UiDialogDescription>
      </UiDialogHeader>

      <form @submit.prevent="onSubmit" class="space-y-4">
        <div class="space-y-2">
          <UiLabel for="edit-name">角色名称 *</UiLabel>
          <UiInput
            id="edit-name"
            v-model="form.name"
            required
          />
        </div>

        <div class="space-y-2">
          <UiLabel for="edit-display_name">显示名称</UiLabel>
          <UiInput
            id="edit-display_name"
            v-model="form.display_name"
          />
        </div>

        <div class="space-y-2">
          <UiLabel for="edit-description">描述</UiLabel>
          <UiTextarea
            id="edit-description"
            v-model="form.description"
            rows="3"
          />
        </div>

        <UiDialogFooter>
          <UiButton type="button" variant="outline" @click="open = false" :disabled="loading">
            取消
          </UiButton>
          <UiButton type="submit" :disabled="loading">
            <span v-if="loading">更新中...</span>
            <span v-else>更新</span>
          </UiButton>
        </UiDialogFooter>
      </form>
    </UiDialogContent>
  </UiDialog>
</template>
