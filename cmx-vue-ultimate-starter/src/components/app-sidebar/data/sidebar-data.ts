import {
  Command,
  GalleryVerticalEnd,
} from 'lucide-vue-next'

import { useDemoSidebar } from './use-demo-sidebar'
import { useSystemSidebar } from './use-system-sidebar'

import type { SidebarData, Team, User, NavGroup } from '../types'

const user: User = {
  name: 'shadcn',
  email: 'm@example.com',
  avatar: '/avatars/shadcn.jpg',
}

const teams: Team[] = [
    {
    name: '系统管理',
    nameEn: 'System Management',
    logo: Command,
    plan: 'Enterprise',
  },
  {
    name: '演示系统',
    nameEn: 'Demo System',
    logo: GalleryVerticalEnd,
    plan: 'Demo',
  }
]

// 团队对应的侧边栏数据映射
const teamSidebarMap: Record<
  string,
  () => { navData: Ref<NavGroup[] | undefined>; otherPages: Ref<NavGroup[]> }
> = {
  演示系统: useDemoSidebar,
  系统管理: useSystemSidebar,
}

// 获取当前团队的侧边栏数据
export function getCurrentTeamSidebar(teamName: string): {
  navMain: NavGroup[]
  otherPages: NavGroup[]
} {
  const sidebarFunction = teamSidebarMap[teamName] || useDemoSidebar
  const { navData, otherPages } = sidebarFunction()
  return {
    navMain: navData.value || [],
    otherPages: otherPages.value,
  }
}

// 默认使用Demo系统的侧边栏
const { navMain, otherPages } = getCurrentTeamSidebar('演示系统')

export const sidebarData: SidebarData = {
  user,
  teams,
  navMain,
  otherPages,
}
