<script setup lang="ts">
import type { Table } from '@tanstack/vue-table'
import type { Role } from '@/services/types/acs/Role'

import { Plus, X } from 'lucide-vue-next'

import DataTableViewOptions from '@/components/data-table/view-options.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

import RoleCreate from './role-create.vue'

interface DataTableToolbarProps {
  table: Table<Role>
}

const props = defineProps<DataTableToolbarProps>()

const isFiltered = computed(() => props.table.getState().columnFilters.length > 0)
const createOpen = ref(false)
</script>

<template>
  <div class="flex items-center justify-between">
    <div class="flex flex-1 items-center space-x-2">
      <Input
        placeholder="搜索角色名称..."
        :model-value="(table.getColumn('name')?.getFilterValue() as string) ?? ''"
        class="h-8 w-[150px] lg:w-[250px]"
        @input="table.getColumn('name')?.setFilterValue($event.target.value)"
      />
      <Button
        v-if="isFiltered"
        variant="ghost"
        class="h-8 px-2 lg:px-3"
        @click="table.resetColumnFilters()"
      >
        重置
        <X class="ml-2 size-4" />
      </Button>
    </div>
    <div class="flex items-center space-x-2">
      <Button size="sm" class="h-8" @click="createOpen = true">
        <Plus class="mr-2 size-4" />
        新建角色
      </Button>
      <DataTableViewOptions :table="table" />
    </div>
  </div>

  <RoleCreate v-model:open="createOpen" />
</template>
