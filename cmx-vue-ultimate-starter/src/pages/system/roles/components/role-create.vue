<script setup lang="ts">
import { Plus } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useQueryClient } from '@tanstack/vue-query'

import { createRole } from '@/services/api/rpc/role'

const open = defineModel<boolean>('open', { default: false })

const loading = ref(false)
const queryClient = useQueryClient()

const form = reactive({
  name: '',
  display_name: '',
  description: '',
})

async function onSubmit() {
  if (!form.name.trim()) {
    toast.error('角色名称不能为空')
    return
  }

  loading.value = true
  try {
    await createRole({
      name: form.name.trim(),
      display_name: form.display_name.trim() || null,
      description: form.description.trim() || null,
    })

    toast.success('角色创建成功')

    // Reset form
    form.name = ''
    form.display_name = ''
    form.description = ''
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
        <UiDialogTitle>新建角色</UiDialogTitle>
        <UiDialogDescription>
          创建一个新的系统角色
        </UiDialogDescription>
      </UiDialogHeader>

      <form @submit.prevent="onSubmit" class="space-y-4">
        <div class="space-y-2">
          <UiLabel for="name">角色名称 *</UiLabel>
          <UiInput
            id="name"
            v-model="form.name"
            placeholder="例如：admin, editor"
            required
          />
        </div>

        <div class="space-y-2">
          <UiLabel for="display_name">显示名称</UiLabel>
          <UiInput
            id="display_name"
            v-model="form.display_name"
            placeholder="例如：管理员"
          />
        </div>

        <div class="space-y-2">
          <UiLabel for="description">描述</UiLabel>
          <UiTextarea
            id="description"
            v-model="form.description"
            placeholder="角色描述..."
            rows="3"
          />
        </div>

        <UiDialogFooter>
          <UiButton type="button" variant="outline" @click="open = false" :disabled="loading">
            取消
          </UiButton>
          <UiButton type="submit" :disabled="loading">
            <span v-if="loading">创建中...</span>
            <span v-else>创建</span>
          </UiButton>
        </UiDialogFooter>
      </form>
    </UiDialogContent>
  </UiDialog>
</template>
