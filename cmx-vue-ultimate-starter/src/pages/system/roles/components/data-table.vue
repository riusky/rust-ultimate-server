<script setup lang="ts">
import type { VisibilityState } from '@tanstack/vue-table'
import { Trash2Icon } from 'lucide-vue-next'

import type { DataTableProps } from '@/components/data-table/types'
import type { Role } from '@/services/types/acs/Role'

import BulkActions from '@/components/data-table/bulk-actions.vue'
import DataTable from '@/components/data-table/data-table.vue'
import { generateVueTable } from '@/components/data-table/use-generate-vue-table'

import DataTableToolbar from './data-table-toolbar.vue'
import RoleDeleteBatch from './role-delete-batch.vue'

interface Props extends DataTableProps<Role> {}

const props = defineProps<Props>()

// Default hidden columns
const initialColumnVisibility: VisibilityState = {
  ctime: false,
  mtime: false,
}

// Create a reactive wrapper that preserves reactivity through getters
const reactiveProps = {
  get data() { return props.data },
  get columns() { return props.columns },
  get loading() { return props.loading },
  initialColumnVisibility,
}

const table = generateVueTable<Role>(reactiveProps)

const deleteBatchOpen = ref(false)
</script>

<template>
  <BulkActions entity-name="role" :table>
    <UiTooltip>
      <UiTooltipTrigger as-child>
        <UiButton
          variant="destructive"
          size="icon"
          class="size-8"
          aria-label="删除选中角色"
          title="删除选中角色"
          @click="deleteBatchOpen = true"
        >
          <Trash2Icon />
          <span class="sr-only">删除选中角色</span>
        </UiButton>
      </UiTooltipTrigger>
      <UiTooltipContent>
        <p>删除选中角色</p>
      </UiTooltipContent>
    </UiTooltip>

    <RoleDeleteBatch v-model:open="deleteBatchOpen" :table />
  </BulkActions>

  <DataTable :columns :table :data :loading>
    <template #toolbar>
      <DataTableToolbar :table="table" class="w-full overflow-x-auto" />
    </template>
  </DataTable>
</template>
