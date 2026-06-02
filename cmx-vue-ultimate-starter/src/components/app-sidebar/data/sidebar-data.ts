import { Command } from 'lucide-vue-next'

import { useSystemSidebar } from './use-system-sidebar'

import type { SidebarData, Team, User, NavGroup } from '../types'

export const DEFAULT_TEAM_NAME = '系统管理'

const user: User = {
  name: 'shadcn',
  email: 'm@example.com',
  avatar: '/avatars/shadcn.jpg',
}

const teams: Team[] = [
  {
    name: DEFAULT_TEAM_NAME,
    nameEn: 'System Management',
    logo: Command,
    plan: 'Enterprise',
  },
]

const teamSidebarMap: Record<
  string,
  () => { navData: Ref<NavGroup[] | undefined>; otherPages: Ref<NavGroup[]> }
> = {
  [DEFAULT_TEAM_NAME]: useSystemSidebar,
}

export function getCurrentTeamSidebar(teamName: string): {
  navMain: NavGroup[]
  otherPages: NavGroup[]
} {
  const sidebarFunction = teamSidebarMap[teamName] || useSystemSidebar
  const { navData, otherPages } = sidebarFunction()
  return {
    navMain: navData.value || [],
    otherPages: otherPages.value,
  }
}

const { navMain, otherPages } = getCurrentTeamSidebar(DEFAULT_TEAM_NAME)

export const sidebarData: SidebarData = {
  user,
  teams,
  navMain,
  otherPages,
}
