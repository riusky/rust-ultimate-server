<script setup lang="ts">
import type { Row } from '@tanstack/vue-table'
import { MoreHorizontal, Pen, Shield, Trash } from 'lucide-vue-next'
import { useQueryClient } from '@tanstack/vue-query'

import type { Role } from '@/services/types/acs/Role'

import RoleDelete from './role-delete.vue'
import RoleEdit from './role-edit.vue'
import RolePermissions from './role-permissions.vue'

interface DataTableRowActionsProps {
  row: Row<Role>
}

const props = defineProps<DataTableRowActionsProps>()

const editOpen = ref(false)
const deleteOpen = ref(false)
const permissionsOpen = ref(false)

const queryClient = useQueryClient()

function handleRefresh() {
  queryClient.invalidateQueries({ queryKey: ['roles'] })
}
</script>

<template>
  <UiDropdownMenu>
    <UiDropdownMenuTrigger as-child>
      <UiButton
        variant="ghost"
        class="flex h-8 w-8 p-0 data-[state=open]:bg-muted"
      >
        <MoreHorizontal class="h-4 w-4" />
        <span class="sr-only">打开菜单</span>
      </UiButton>
    </UiDropdownMenuTrigger>
    <UiDropdownMenuContent align="end" class="w-[160px]">
      <UiDropdownMenuItem @click="editOpen = true">
        <Pen class="mr-2 h-4 w-4" />
        编辑
      </UiDropdownMenuItem>
      <UiDropdownMenuItem @click="permissionsOpen = true">
        <Shield class="mr-2 h-4 w-4" />
        权限
      </UiDropdownMenuItem>
      <UiDropdownMenuSeparator />
      <UiDropdownMenuItem @click="deleteOpen = true" class="text-destructive">
        <Trash class="mr-2 h-4 w-4" />
        删除
      </UiDropdownMenuItem>
    </UiDropdownMenuContent>
  </UiDropdownMenu>

  <RoleEdit
    v-model:open="editOpen"
    :role="props.row.original"
    @updated="handleRefresh"
  />

  <RoleDelete
    v-model:open="deleteOpen"
    :role="props.row.original"
    @deleted="handleRefresh"
  />

  <RolePermissions
    v-model:open="permissionsOpen"
    :role="props.row.original"
  />
</template>
