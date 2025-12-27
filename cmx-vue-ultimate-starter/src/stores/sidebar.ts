import { ref, computed, watch } from 'vue'
import { defineStore } from 'pinia'
import { sidebarData, getCurrentTeamSidebar } from '@/components/app-sidebar/data/sidebar-data'
import type { Team, NavGroup } from '@/components/app-sidebar/types'

export const useSidebarStore = defineStore('sidebar', () => {
  // 持久化的团队名称（只存储名称，因为 logo 组件无法序列化）
  const persistedTeamName = ref<string | null>(null)

  // 根据持久化名称或默认值获取初始团队
  function getInitialTeam(): Team | null {
    if (persistedTeamName.value) {
      const found = sidebarData.teams.find((t) => t.name === persistedTeamName.value)
      if (found) return found
    }
    return sidebarData.teams.length > 0 ? sidebarData.teams[0]! : null
  }

  const initialTeam = getInitialTeam()
  const { navMain: initialNavMain, otherPages: initialOtherPages } = getCurrentTeamSidebar(
    initialTeam?.name || '演示系统',
  )

  const activeTeam = ref<Team | null>(initialTeam)

  // 当前团队的导航数据
  const navMain = ref<NavGroup[]>(initialNavMain)

  // 当前团队的其他页面数据
  const otherPages = ref<NavGroup[]>(initialOtherPages)

  // 监听持久化数据恢复，hydration 后同步 activeTeam
  watch(
    persistedTeamName,
    (teamName) => {
      if (teamName && activeTeam.value?.name !== teamName) {
        const found = sidebarData.teams.find((t) => t.name === teamName)
        if (found) {
          activeTeam.value = found
          const { navMain: newNavMain, otherPages: newOtherPages } = getCurrentTeamSidebar(teamName)
          navMain.value = newNavMain
          otherPages.value = newOtherPages
        }
      }
    },
  )

  // 获取当前团队的第一个可访问路径
  function getFirstAvailablePath(navGroups: NavGroup[]): string {
    for (const group of navGroups) {
      for (const item of group.items) {
        if (item.url) {
          return item.url
        }
        if (item.items && item.items.length > 0) {
          const firstSubItem = item.items.find((subItem) => subItem.url)
          if (firstSubItem?.url) {
            return firstSubItem.url
          }
        }
      }
    }
    return '/system/users/' // 默认路径
  }

  // 初始化所有团队的默认激活项
  function initializeTeamActiveItems(): Record<string, string> {
    const items: Record<string, string> = {}
    for (const team of sidebarData.teams) {
      const { navMain: teamNavMain } = getCurrentTeamSidebar(team.name)
      items[team.name] = getFirstAvailablePath(teamNavMain)
    }
    return items
  }

  // 记录每个团队的激活项
  const teamActiveItems = ref<Record<string, string>>(initializeTeamActiveItems())

  // 当前团队名称
  const activeTeamName = computed(() => activeTeam.value?.name || '演示系统')

  // 当前激活的item路径
  const activeItemPath = computed(() => {
    return teamActiveItems.value[activeTeamName.value] || ''
  })

  // 切换团队
  function switchTeam(teamName: string) {
    const targetTeam = sidebarData.teams.find((team) => team.name === teamName)
    if (targetTeam) {
      const { navMain: newNavMain, otherPages: newOtherPages } = getCurrentTeamSidebar(
        targetTeam.name,
      )

      // 先确保该团队有激活项记录（在更新 activeTeam 之前）
      if (!teamActiveItems.value[teamName]) {
        const firstPath = getFirstAvailablePath(newNavMain)
        teamActiveItems.value[teamName] = firstPath
      }

      // 再更新团队和导航数据
      activeTeam.value = targetTeam
      persistedTeamName.value = teamName // 持久化团队名称
      navMain.value = newNavMain
      otherPages.value = newOtherPages
    }
  }

  // 设置当前团队的激活项
  function setActiveItem(teamName: string, itemPath: string) {
    teamActiveItems.value[teamName] = itemPath
  }

  // 根据团队名称切换团队
  function switchTeamByName(teamName: string) {
    switchTeam(teamName)
  }

  // 获取当前团队的导航数据
  function getCurrentNavMain(): NavGroup[] {
    return navMain.value
  }

  // 重置到默认团队
  function resetToDefaultTeam() {
    const defaultTeam = sidebarData.teams.find((team) => team.name === '演示系统')
    if (defaultTeam) {
      activeTeam.value = defaultTeam
      const { navMain: newNavMain, otherPages: newOtherPages } = getCurrentTeamSidebar(
        defaultTeam.name,
      )
      navMain.value = newNavMain
      otherPages.value = newOtherPages

      // 重置激活项
      const firstPath = getFirstAvailablePath(newNavMain)
      teamActiveItems.value[defaultTeam.name] = firstPath
    }
  }

  // 获取指定团队的激活项
  function getTeamActiveItem(teamName: string): string {
    return teamActiveItems.value[teamName] || ''
  }

  // 更新当前团队的激活项
  function updateCurrentTeamActiveItem(itemPath: string) {
    teamActiveItems.value[activeTeamName.value] = itemPath
  }

  // 重新初始化所有团队的激活项
  function reinitializeTeamActiveItems() {
    teamActiveItems.value = initializeTeamActiveItems()
  }

  return {
    activeTeam,
    navMain,
    otherPages,
    teamActiveItems,
    activeTeamName,
    activeItemPath,
    switchTeam,
    setActiveItem,
    switchTeamByName,
    getCurrentNavMain,
    resetToDefaultTeam,
    getTeamActiveItem,
    updateCurrentTeamActiveItem,
    getFirstAvailablePath,
    reinitializeTeamActiveItems,
    persistedTeamName,
  }
}, {
  persist: {
    pick: ['persistedTeamName', 'teamActiveItems'],
  },
})
