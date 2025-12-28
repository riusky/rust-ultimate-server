<script setup lang="ts">
import { useQuery } from '@tanstack/vue-query'

import type { ServerPagination } from '@/components/data-table/types'
import Page from '@/components/global-layout/basic-page.vue'
import { listUsers, type ListUsersParams } from '@/services/api/rpc/user/user'

import { columns } from './components/columns'
import DataTable from './components/data-table.vue'

// Pagination state
const pageSize = ref(20)
const pageNumber = ref(1)

const { data, isLoading } = useQuery({
  queryKey: computed(() => ['users', { page_size: pageSize.value, page_number: pageNumber.value }]),
  queryFn: () => listUsers({ page_size: pageSize.value, page_number: pageNumber.value }),
  refetchOnMount: 'always',
})

const users = computed(() => data.value?.data ?? [])
const total = computed(() => data.value?.page_info.total ?? 0)

const serverPagination = computed<ServerPagination>(() => ({
  page: pageNumber.value,
  pageSize: pageSize.value,
  total: total.value,
  onPageChange: (page: number) => {
    pageNumber.value = page
  },
  onPageSizeChange: (size: number) => {
    pageSize.value = size
    pageNumber.value = 1 // Reset to first page
  },
}))
</script>

<template>
  <Page title="用户管理" description="管理系统用户信息" sticky>
    <div class="overflow-x-auto">
      <DataTable
        :data="users"
        :columns="columns"
        :loading="isLoading"
        :server-pagination="serverPagination"
      />
    </div>
  </Page>
</template>

<route lang="yaml">
meta:
  auth: true
</route>
