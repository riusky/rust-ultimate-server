<script setup lang="ts">
import type { Role } from '@/services/types/acs/Role'

interface RolePermissionsProps {
  role: Role
}

const props = defineProps<RolePermissionsProps>()
const open = defineModel<boolean>('open', { default: false })
</script>

<template>
  <UiDialog v-model:open="open">
    <UiDialogContent class="sm:max-w-[600px]">
      <UiDialogHeader>
        <UiDialogTitle>管理权限</UiDialogTitle>
        <UiDialogDescription>
          管理角色 "{{ props.role.display_name || props.role.name }}" 的权限
        </UiDialogDescription>
      </UiDialogHeader>

      <div class="py-4">
        <p class="text-sm text-muted-foreground">
          权限管理功能待实现...
        </p>
      </div>

      <UiDialogFooter>
        <UiButton variant="outline" @click="open = false">
          关闭
        </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
