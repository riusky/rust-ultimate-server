<script setup lang="ts">
import type { Row } from '@tanstack/vue-table'
import type { UserWithInfo } from '@/services/types/user/index'

import { KeyRound, MoreHorizontal, Pencil, Shield, Trash2 } from 'lucide-vue-next'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

import UserDelete from './user-delete.vue'
import UserEdit from './user-edit.vue'
import UserResetPwd from './user-reset-pwd.vue'
import UserRoles from './user-roles.vue'

interface DataTableRowActionsProps {
  row: Row<UserWithInfo>
}

const props = defineProps<DataTableRowActionsProps>()

const editOpen = ref(false)
const deleteOpen = ref(false)
const resetPwdOpen = ref(false)
const rolesOpen = ref(false)
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="ghost" class="flex size-8 p-0 data-[state=open]:bg-muted">
        <MoreHorizontal class="size-4" />
        <span class="sr-only">打开菜单</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end" class="w-[160px]">
      <DropdownMenuItem @click="editOpen = true">
        <Pencil class="mr-2 size-4" />
        编辑信息
      </DropdownMenuItem>
      <DropdownMenuItem @click="rolesOpen = true">
        <Shield class="mr-2 size-4" />
        分配角色
      </DropdownMenuItem>
      <DropdownMenuItem @click="resetPwdOpen = true">
        <KeyRound class="mr-2 size-4" />
        重置密码
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem class="text-destructive" @click="deleteOpen = true">
        <Trash2 class="mr-2 size-4" />
        删除
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>

  <UserEdit v-model:open="editOpen" :user="props.row.original" />
  <UserRoles v-model:open="rolesOpen" :user="props.row.original" />
  <UserResetPwd v-model:open="resetPwdOpen" :user="props.row.original" />
  <UserDelete v-model:open="deleteOpen" :user="props.row.original" />
</template>
