import type { ColumnDef } from '@tanstack/vue-table'
import type { UserWithInfo } from '@/services/types/user/index'

import { h } from 'vue'

import DataTableColumnHeader from '@/components/data-table/column-header.vue'
import { SelectColumn } from '@/components/data-table/table-columns'
import { Badge } from '@/components/ui/badge'

import { genders, statuses, userTypes } from './data'
import DataTableRowActions from './data-table-row-actions.vue'

export const columns: ColumnDef<UserWithInfo>[] = [
  SelectColumn as ColumnDef<UserWithInfo>,
  {
    accessorKey: 'id',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: 'ID' }),
    cell: ({ row }) => h('div', { class: 'w-16' }, String(row.getValue('id'))),
    enableSorting: true,
    enableHiding: false,
  },
  {
    accessorKey: 'username',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '用户名' }),
    cell: ({ row }) => {
      const username = row.getValue('username') as string
      return h('div', { class: 'max-w-[150px] truncate font-medium' }, username)
    },
  },
  {
    accessorKey: 'typ',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '用户类型' }),
    cell: ({ row }) => {
      const typ = userTypes.find((t) => t.value === row.getValue('typ'))
      if (!typ) return h('span', '-')
      return h('div', { class: 'flex items-center' }, [
        typ.icon && h(typ.icon, { class: 'mr-2 h-4 w-4 text-muted-foreground' }),
        h('span', typ.label),
      ])
    },
    filterFn: (row, id, value) => value.includes(row.getValue(id)),
  },
  {
    accessorKey: 'nickname',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '昵称' }),
    cell: ({ row }) => {
      const nickname = row.getValue('nickname') as string | null
      return h('div', { class: 'max-w-[200px] truncate' }, nickname || '-')
    },
  },
  {
    accessorKey: 'email',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '邮箱' }),
    cell: ({ row }) => {
      const email = row.getValue('email') as string | null
      const verified = row.original.email_verified
      return h('div', { class: 'flex items-center gap-1' }, [
        h('span', { class: 'max-w-[200px] truncate' }, email || '-'),
        verified ? h(Badge, { variant: 'secondary', class: 'text-xs' }, () => '已验证') : null,
      ])
    },
  },
  {
    accessorKey: 'phone',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '手机' }),
    cell: ({ row }) => {
      const phone = row.getValue('phone') as string | null
      const verified = row.original.phone_verified
      return h('div', { class: 'flex items-center gap-1' }, [
        h('span', phone || '-'),
        verified ? h(Badge, { variant: 'secondary', class: 'text-xs' }, () => '已验证') : null,
      ])
    },
  },
  {
    accessorKey: 'gender',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '性别' }),
    cell: ({ row }) => {
      const gender = genders.find((g) => g.value === row.getValue('gender'))
      if (!gender) return h('span', '-')
      return h('div', { class: 'flex items-center' }, [
        gender.icon && h(gender.icon, { class: 'mr-2 h-4 w-4 text-muted-foreground' }),
        h('span', gender.label),
      ])
    },
    filterFn: (row, id, value) => value.includes(row.getValue(id)),
  },
  {
    accessorKey: 'status',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '状态' }),
    cell: ({ row }) => {
      const status = statuses.find((s) => s.value === row.getValue('status'))
      if (!status) return h('span', '-')
      return h('div', { class: 'flex w-[100px] items-center' }, [
        status.icon && h(status.icon, { class: 'mr-2 h-4 w-4 text-muted-foreground' }),
        h('span', status.label),
      ])
    },
    filterFn: (row, id, value) => value.includes(row.getValue(id)),
  },
  // Hidden by default columns
  {
    accessorKey: 'bio',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '简介' }),
    cell: ({ row }) => {
      const bio = row.getValue('bio') as string | null
      return h('div', { class: 'max-w-[200px] truncate' }, bio || '-')
    },
    enableHiding: true,
  },
  {
    accessorKey: 'country',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '国家' }),
    cell: ({ row }) => h('span', (row.getValue('country') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'province',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '省份' }),
    cell: ({ row }) => h('span', (row.getValue('province') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'city',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '城市' }),
    cell: ({ row }) => h('span', (row.getValue('city') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'timezone',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '时区' }),
    cell: ({ row }) => h('span', (row.getValue('timezone') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'locale',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '语言' }),
    cell: ({ row }) => h('span', (row.getValue('locale') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'theme',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '主题' }),
    cell: ({ row }) => h('span', (row.getValue('theme') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'login_count',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '登录次数' }),
    cell: ({ row }) => {
      const count = row.getValue('login_count') as number | null
      return h('span', count !== null ? String(count) : '-')
    },
    enableHiding: true,
  },
  {
    accessorKey: 'last_login_at',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '最后登录' }),
    cell: ({ row }) => {
      const lastLogin = row.getValue('last_login_at') as string | null
      if (!lastLogin) return h('span', '-')
      const date = new Date(lastLogin)
      return h('span', { class: 'text-sm text-muted-foreground' }, date.toLocaleString('zh-CN'))
    },
    enableHiding: true,
  },
  {
    accessorKey: 'last_login_ip',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '最后登录IP' }),
    cell: ({ row }) => h('span', (row.getValue('last_login_ip') as string) || '-'),
    enableHiding: true,
  },
  {
    accessorKey: 'ctime',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '创建时间' }),
    cell: ({ row }) => {
      const ctime = row.getValue('ctime') as string
      const date = new Date(ctime)
      return h('span', { class: 'text-sm text-muted-foreground' }, date.toLocaleString('zh-CN'))
    },
  },
  {
    accessorKey: 'mtime',
    header: ({ column }) => h(DataTableColumnHeader<UserWithInfo>, { column, title: '更新时间' }),
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
