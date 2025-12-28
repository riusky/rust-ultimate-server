import type { ColumnDef } from '@tanstack/vue-table'
import type { Role } from '@/services/types/acs/Role'

import { h } from 'vue'

import DataTableColumnHeader from '@/components/data-table/column-header.vue'
import { SelectColumn } from '@/components/data-table/table-columns'

import DataTableRowActions from './data-table-row-actions.vue'

export const columns: ColumnDef<Role>[] = [
  SelectColumn as ColumnDef<Role>,
  {
    accessorKey: 'id',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: 'ID' }),
    cell: ({ row }) => h('div', { class: 'w-16' }, String(row.getValue('id'))),
    enableSorting: true,
    enableHiding: false,
  },
  {
    accessorKey: 'name',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: '角色名称' }),
    cell: ({ row }) => {
      const name = row.getValue('name') as string
      return h('div', { class: 'max-w-[150px] truncate font-medium' }, name)
    },
    enableSorting: true,
  },
  {
    accessorKey: 'display_name',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: '显示名称' }),
    cell: ({ row }) => {
      const displayName = row.getValue('display_name') as string | null
      return h('div', { class: 'max-w-[200px] truncate' }, displayName || '-')
    },
    enableSorting: true,
  },
  {
    accessorKey: 'description',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: '描述' }),
    cell: ({ row }) => {
      const description = row.getValue('description') as string | null
      return h('div', { class: 'max-w-[300px] truncate text-muted-foreground' }, description || '-')
    },
    enableSorting: false,
  },
  {
    accessorKey: 'ctime',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: '创建时间' }),
    cell: ({ row }) => {
      const ctime = row.getValue('ctime') as string
      const date = new Date(ctime)
      return h('span', { class: 'text-sm text-muted-foreground' }, date.toLocaleString('zh-CN'))
    },
    enableHiding: true,
  },
  {
    accessorKey: 'mtime',
    header: ({ column }) => h(DataTableColumnHeader<Role>, { column, title: '更新时间' }),
    cell: ({ row }) => {
      const mtime = row.getValue('mtime') as string
      const date = new Date(mtime)
      return h('span', { class: 'text-sm text-muted-foreground' }, date.toLocaleString('zh-CN'))
    },
    enableHiding: true,
  },
  {
    id: 'actions',
    cell: ({ row }) => h(DataTableRowActions, { row }),
  },
]
