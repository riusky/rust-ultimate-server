<script setup lang="ts">
import { useQuery } from '@tanstack/vue-query'

import Page from '@/components/global-layout/basic-page.vue'
import { listRoles } from '@/services/api/rpc/role'

import { columns } from './components/columns'
import DataTable from './components/data-table.vue'

const { data, isLoading } = useQuery({
  queryKey: ['roles'],
  queryFn: () => listRoles(),
  refetchOnMount: 'always',
})

const roles = computed(() => data.value ?? [])
</script>

<template>
  <Page title="角色管理" description="管理系统角色和权限" sticky>
    <div class="overflow-x-auto">
      <DataTable
        :data="roles"
        :columns="columns"
        :loading="isLoading"
      />
    </div>
  </Page>
</template>

<style scoped></style>

<route lang="yaml">
meta:
  auth: true
</route>
