<script setup lang="ts">
import type { VisibilityState } from '@tanstack/vue-table'
import { Trash2Icon } from 'lucide-vue-next'

import type { DataTableProps, ServerPagination } from '@/components/data-table/types'
import type { UserWithInfo } from '@/services/types/user/index'

import BulkActions from '@/components/data-table/bulk-actions.vue'
import DataTable from '@/components/data-table/data-table.vue'
import { generateVueTable } from '@/components/data-table/use-generate-vue-table'

import DataTableToolbar from './data-table-toolbar.vue'
import UserDeleteBatch from './user-delete-batch.vue'

interface Props extends DataTableProps<UserWithInfo> {
  serverPagination?: ServerPagination
}

const props = defineProps<Props>()

// Default hidden columns - only show essential columns
const initialColumnVisibility: VisibilityState = {
  typ: false,
  phone: false,
  gender: false,
  bio: false,
  country: false,
  province: false,
  city: false,
  timezone: false,
  locale: false,
  theme: false,
  login_count: false,
  last_login_at: false,
  last_login_ip: false,
  ctime: false,
  mtime: false,
}

// Create a reactive wrapper that preserves reactivity through getters
const reactiveProps = {
  get data() { return props.data },
  get columns() { return props.columns },
  get loading() { return props.loading },
  get serverPagination() { return props.serverPagination },
  initialColumnVisibility,
}

const table = generateVueTable<UserWithInfo>(reactiveProps)

const deleteBatchOpen = ref(false)
</script>

<template>
  <BulkActions entity-name="user" :table>
    <UiTooltip>
      <UiTooltipTrigger as-child>
        <UiButton
          variant="destructive"
          size="icon"
          class="size-8"
          aria-label="删除选中用户"
          title="删除选中用户"
          @click="deleteBatchOpen = true"
        >
          <Trash2Icon />
          <span class="sr-only">删除选中用户</span>
        </UiButton>
      </UiTooltipTrigger>
      <UiTooltipContent>
        <p>删除选中用户</p>
      </UiTooltipContent>
    </UiTooltip>

    <UserDeleteBatch v-model:open="deleteBatchOpen" :table />
  </BulkActions>

  <DataTable :columns :table :data :loading :server-pagination="serverPagination">
    <template #toolbar>
      <DataTableToolbar :table="table" class="w-full overflow-x-auto" />
    </template>
  </DataTable>
</template>
