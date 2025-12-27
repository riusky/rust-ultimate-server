<script setup lang="ts">
import { useCookies } from '@vueuse/integrations/useCookies'
import { storeToRefs } from 'pinia'
import { useRoute, useRouter } from 'vue-router'

import AppSidebar from '@/components/app-sidebar/index.vue'
import CommandMenuPanel from '@/components/command-menu-panel/index.vue'
import ThemePopover from '@/components/custom-theme/theme-popover.vue'
import ToggleTheme from '@/components/toggle-theme.vue'
import LangToggle from '@/components/language-change.vue'
import LocalTime from '@/components/local-time.vue'
import WindowControls from '@/components/window-controls.vue'
import { cn } from '@/lib/utils'
import { useThemeStore } from '@/stores/theme'
import { useSidebarStore } from '@/stores/sidebar'
import { SIDEBAR_COOKIE_NAME } from '@/components/ui/sidebar/utils'

const defaultOpen = useCookies([SIDEBAR_COOKIE_NAME])
const themeStore = useThemeStore()
const sidebarStore = useSidebarStore()
const { contentLayout } = storeToRefs(themeStore)
const { activeTeamName, activeItemPath } = storeToRefs(sidebarStore)
const route = useRoute()
const router = useRouter()

// 当前路由路径，用于跟踪是否发生跨团队路由变化
const currentRoutePath = ref('')

// 路由路径到团队名称的映射
const routeTeamMap: Record<string, string> = {
  '/system': '系统管理',
  '/demo': '演示系统',
}

// 获取当前路由对应的团队名称
function getTeamFromPath(path: string): string {
  for (const [pathPrefix, teamName] of Object.entries(routeTeamMap)) {
    if (path.startsWith(pathPrefix)) {
      return teamName
    }
  }
  return '系统管理' // 默认团队
}

// 监听路由变化，只在首次加载或跨团队路由变化时切换团队
watch(
  () => route.path,
  (newPath: string) => {
    const targetTeam = getTeamFromPath(newPath)

    // 只有当目标团队与当前团队不同时才切换
    if (targetTeam !== activeTeamName.value) {
      sidebarStore.switchTeam(targetTeam)
    }

    // 确保设置当前激活项
    sidebarStore.updateCurrentTeamActiveItem(newPath)

    currentRoutePath.value = newPath
  },
  { immediate: true },
)

// 监听团队切换，自动导航到该团队的激活项
watch(
  () => activeTeamName.value,
  (newTeamName: any, oldTeamName: any) => {
    if (newTeamName !== oldTeamName) {
      const activePath = sidebarStore.getTeamActiveItem(newTeamName)
      if (activePath && activePath !== route.path) {
        router.push(activePath)
      }
    }
  },
)

// 监听激活项变化，确保路由同步
watch(
  () => activeItemPath.value,
  (newPath: any) => {
    if (newPath && newPath !== route.path) {
      router.push(newPath)
    }
  },
)
</script>

<template>
  <UiSidebarProvider :default-open="defaultOpen.get(SIDEBAR_COOKIE_NAME)">
    <AppSidebar />
    <UiSidebarInset
      class="w-full max-w-full peer-data-[state=collapsed]:w-[calc(100%-var(--sidebar-width-icon)-1rem)] peer-data-[state=expanded]:w-[calc(100%-var(--sidebar-width))]"
    >
      <header
        data-tauri-drag-region
        class="sticky top-0 z-50 flex items-center gap-3 sm:gap-4 h-12 p-4 pr-2 shrink-0 transition-[width,height] ease-linear bg-sidebar text-sidebar-foreground"
      >
        <UiSidebarTrigger class="-ml-1" />
        <UiSeparator orientation="vertical" class="h-6" />
        <CommandMenuPanel />
        <div class="ml-auto flex items-center gap-1">
          <LocalTime
            date-style="full"
            time-style="full"
            :showWeekday="true"
            :showSeconds="true"
            :hour12="true"
          />
          <LangToggle />
          <ToggleTheme />
          <ThemePopover />
          <!-- 非pc项目禁用窗口控制器 -->
          <!-- <WindowControls /> -->
        </div>
      </header>
      <div :class="cn('pl-4 pr-4 pb-4 grow', contentLayout === 'centered' ? 'container mx-auto ' : '')">
        <router-view />
      </div>
    </UiSidebarInset>
  </UiSidebarProvider>
</template>
