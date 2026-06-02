import { Users } from 'lucide-vue-next'

import type { NavGroup } from '@/components/app-sidebar/types'

export function useSystemSidebar() {
  const navData = ref<NavGroup[]>([
    {
      title: '系统管理',
      items: [
        {
          title: '用户权限',
          icon: Users,
          items: [
            { title: '用户管理', url: '/system/users' },
            { title: '角色管理', url: '/system/roles' },
          ],
        },
      ],
    },
  ])

  const otherPages = ref<NavGroup[]>([])

  return {
    navData,
    otherPages,
  }
}
