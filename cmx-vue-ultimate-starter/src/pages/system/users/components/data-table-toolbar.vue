<script setup lang="ts">
import type { Table } from '@tanstack/vue-table'
import type { UserWithInfo } from '@/services/types/user/index'

import { Plus, X } from 'lucide-vue-next'

import DataTableFacetedFilter from '@/components/data-table/faceted-filter.vue'
import DataTableViewOptions from '@/components/data-table/view-options.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

import { genders, statuses } from './data'
import UserCreate from './user-create.vue'

interface DataTableToolbarProps {
  table: Table<UserWithInfo>
}

const props = defineProps<DataTableToolbarProps>()

const isFiltered = computed(() => props.table.getState().columnFilters.length > 0)
const createOpen = ref(false)
</script>

<template>
  <div class="flex items-center justify-between">
    <div class="flex flex-1 items-center space-x-2">
      <Input
        placeholder="Search nickname..."
        :model-value="(table.getColumn('nickname')?.getFilterValue() as string) ?? ''"
        class="h-8 w-[150px] lg:w-[250px]"
        @input="table.getColumn('nickname')?.setFilterValue($event.target.value)"
      />
      <DataTableFacetedFilter
        v-if="table.getColumn('status')"
        :column="table.getColumn('status')"
        title="Status"
        :options="statuses"
      />
      <DataTableFacetedFilter
        v-if="table.getColumn('gender')"
        :column="table.getColumn('gender')"
        title="Gender"
        :options="genders"
      />
      <Button
        v-if="isFiltered"
        variant="ghost"
        class="h-8 px-2 lg:px-3"
        @click="table.resetColumnFilters()"
      >
        Reset
        <X class="ml-2 size-4" />
      </Button>
    </div>
    <div class="flex items-center space-x-2">
      <Button size="sm" class="h-8" @click="createOpen = true">
        <Plus class="mr-2 size-4" />
        Add User
      </Button>
      <DataTableViewOptions :table="table" />
    </div>
  </div>

  <UserCreate v-model:open="createOpen" />
</template>
