<script lang="ts" setup>
import { sidebarData } from './data/sidebar-data'
import NavFooter from './nav-footer.vue'
import NavTeam from './nav-team.vue'
import TeamSwitcher from './team-switcher.vue'
import { useSidebarStore } from '@/stores/sidebar'
import { storeToRefs } from 'pinia'

import type { Team } from './types'

// 使用 Pinia store 管理状态
const sidebarStore = useSidebarStore()
const { activeTeam, navMain, otherPages } = storeToRefs(sidebarStore)

// 团队切换器引用
const teamSwitcherRef = ref()

// 处理团队切换
function handleTeamChange(team: Team) {
  sidebarStore.switchTeam(team.name)
}
</script>

<template>
  <UiSidebar collapsible="icon" class="z-50">
    <UiSidebarHeader>
      <TeamSwitcher
        ref="teamSwitcherRef"
        :teams="sidebarData.teams"
        :active-team="activeTeam"
        @team-change="handleTeamChange"
      />
    </UiSidebarHeader>

    <UiSidebarContent>
      <NavTeam :nav-main="navMain" :other-pages="otherPages" />
    </UiSidebarContent>

    <UiSidebarFooter>
      <NavFooter :user="sidebarData.user" />
    </UiSidebarFooter>

    <UiSidebarRail />
  </UiSidebar>
</template>

<style></style>
