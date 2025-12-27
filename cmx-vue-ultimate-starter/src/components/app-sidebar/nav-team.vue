<script lang="ts" setup>
import { ChevronRight } from 'lucide-vue-next'
import { useSidebar } from '@/components/ui/sidebar'
import { useSidebarStore } from '@/stores/sidebar'
import type { NavGroup, NavItem } from './types'

const { navMain, otherPages } = defineProps<{
  navMain: NavGroup[]
  otherPages: NavGroup[]
}>()

const route = useRoute()
const sidebarStore = useSidebarStore()
const { state, isMobile } = useSidebar()

// 检查菜单项是否展开（用于可折叠菜单）
function isCollapsed(menu: NavItem): boolean {
  const pathname = route.path
  // 检查当前菜单项的子项中是否有匹配当前路由的路径
  return !!menu.items?.some((item) => item.url === pathname)
}

// 检查菜单项是否激活
function isActive(menu: NavItem): boolean {
  const pathname = route.path
  const activeItemPath = sidebarStore.activeItemPath

  // 如果菜单项有 URL，检查是否匹配当前路由或激活项
  if (menu.url) {
    return pathname === menu.url || activeItemPath === menu.url
  }

  // 对于有子项的菜单，检查子项是否匹配当前路由或激活项
  return !!menu.items?.some((item) => {
    return item.url === pathname || item.url === activeItemPath
  })
}

// 处理导航项点击
function handleNavItemClick(itemPath: string) {
  sidebarStore.updateCurrentTeamActiveItem(itemPath)
}

// 处理子导航项点击
function handleSubNavItemClick(itemPath: string) {
  sidebarStore.updateCurrentTeamActiveItem(itemPath)
}
</script>

<template>
  <UiSidebarGroup v-for="group in navMain" :key="group.title">
    <UiSidebarGroupLabel>{{ group.title }}</UiSidebarGroupLabel>
    <UiSidebarMenu>
      <template v-for="menu in group.items" :key="menu.title">
        <UiSidebarMenuItem v-if="!menu.items">
          <UiSidebarMenuButton
            as-child
            :is-active="isActive(menu)"
            :tooltip="menu.title"
            @click="handleNavItemClick(menu.url || '')"
            :class="{ '!bg-primary !text-primary-foreground': isActive(menu) }"
          >
            <router-link :to="menu.url">
              <component :is="menu.icon" />
              <span>{{ menu.title }}</span>
            </router-link>
          </UiSidebarMenuButton>
        </UiSidebarMenuItem>

        <UiSidebarMenuItem v-else>
          <!-- sidebar expanded -->
          <UiCollapsible
            v-if="state !== 'collapsed' || isMobile"
            as-child
            :default-open="isCollapsed(menu)"
            class="group/collapsible"
          >
            <UiSidebarMenuItem>
              <UiCollapsibleTrigger as-child>
                <UiSidebarMenuButton :tooltip="menu.title">
                  <component :is="menu.icon" v-if="menu.icon" />
                  <span>{{ menu.title }}</span>
                  <ChevronRight
                    class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                  />
                </UiSidebarMenuButton>
              </UiCollapsibleTrigger>
            </UiSidebarMenuItem>
            <UiCollapsibleContent>
              <UiSidebarMenuSub>
                <UiSidebarMenuSubItem v-for="subItem in menu.items" :key="subItem.title">
                  <UiSidebarMenuSubButton
                    as-child
                    :is-active="isActive(subItem as NavItem)"
                    @click="handleSubNavItemClick(subItem?.url || '')"
                    :class="{
                      '!bg-primary !text-primary-foreground': isActive(subItem as NavItem),
                    }"
                  >
                    <router-link :to="subItem?.url || '/'">
                      <component :is="subItem.icon" v-if="subItem.icon" />
                      <span>{{ subItem.title }}</span>
                    </router-link>
                  </UiSidebarMenuSubButton>
                </UiSidebarMenuSubItem>
              </UiSidebarMenuSub>
            </UiCollapsibleContent>
          </UiCollapsible>

          <!-- sidebar collapsed -->
          <UiDropdownMenu v-else>
            <UiDropdownMenuTrigger as-child>
              <UiSidebarMenuButton :tooltip="menu.title">
                <component :is="menu.icon" v-if="menu.icon" />
                <span>{{ menu.title }}</span>
              </UiSidebarMenuButton>
            </UiDropdownMenuTrigger>
            <UiDropdownMenuContent align="start" side="right">
              <UiDropdownMenuLabel>{{ menu.title }}</UiDropdownMenuLabel>
              <UiDropdownMenuSeparator />
              <UiDropdownMenuItem
                v-for="subItem in menu.items"
                :key="subItem.title"
                as-child
                @click="handleSubNavItemClick(subItem?.url || '')"
              >
                <router-link :to="subItem?.url || '/'">
                  <component :is="subItem.icon" v-if="subItem.icon" />
                  <span>{{ subItem.title }}</span>
                </router-link>
              </UiDropdownMenuItem>
            </UiDropdownMenuContent>
          </UiDropdownMenu>
        </UiSidebarMenuItem>
      </template>
    </UiSidebarMenu>
  </UiSidebarGroup>

  <UiSidebarGroup v-for="group in otherPages" :key="group.title">
    <UiSidebarGroupLabel>{{ group.title }}</UiSidebarGroupLabel>
    <UiSidebarMenu>
      <template v-for="menu in group.items" :key="menu.title">
        <UiSidebarMenuItem v-if="!menu.items">
          <UiSidebarMenuButton
            as-child
            :is-active="isActive(menu)"
            :tooltip="menu.title"
            @click="handleNavItemClick(menu.url || '')"
            :class="{ '!bg-primary !text-primary-foreground': isActive(menu) }"
          >
            <router-link :to="menu.url">
              <component :is="menu.icon" />
              <span>{{ menu.title }}</span>
            </router-link>
          </UiSidebarMenuButton>
        </UiSidebarMenuItem>

        <UiSidebarMenuItem v-else>
          <!-- sidebar expanded -->
          <UiCollapsible
            v-if="state !== 'collapsed' || isMobile"
            as-child
            :default-open="isCollapsed(menu)"
            class="group/collapsible"
          >
            <UiSidebarMenuItem>
              <UiCollapsibleTrigger as-child>
                <UiSidebarMenuButton :tooltip="menu.title">
                  <component :is="menu.icon" v-if="menu.icon" />
                  <span>{{ menu.title }}</span>
                  <ChevronRight
                    class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                  />
                </UiSidebarMenuButton>
              </UiCollapsibleTrigger>
            </UiSidebarMenuItem>
            <UiCollapsibleContent>
              <UiSidebarMenuSub>
                <UiSidebarMenuSubItem v-for="subItem in menu.items" :key="subItem.title">
                  <UiSidebarMenuSubButton
                    as-child
                    :is-active="isActive(subItem as NavItem)"
                    @click="handleSubNavItemClick(subItem?.url || '')"
                    :class="{
                      '!bg-primary !text-primary-foreground': isActive(subItem as NavItem),
                    }"
                  >
                    <router-link :to="subItem?.url || '/'">
                      <component :is="subItem.icon" v-if="subItem.icon" />
                      <span>{{ subItem.title }}</span>
                    </router-link>
                  </UiSidebarMenuSubButton>
                </UiSidebarMenuSubItem>
              </UiSidebarMenuSub>
            </UiCollapsibleContent>
          </UiCollapsible>

          <!-- sidebar collapsed -->
          <UiDropdownMenu v-else>
            <UiDropdownMenuTrigger as-child>
              <UiSidebarMenuButton :tooltip="menu.title">
                <component :is="menu.icon" v-if="menu.icon" />
                <span>{{ menu.title }}</span>
              </UiSidebarMenuButton>
            </UiDropdownMenuTrigger>
            <UiDropdownMenuContent align="start" side="right">
              <UiDropdownMenuLabel>{{ menu.title }}</UiDropdownMenuLabel>
              <UiDropdownMenuSeparator />
              <UiDropdownMenuItem
                v-for="subItem in menu.items"
                :key="subItem.title"
                as-child
                @click="handleSubNavItemClick(subItem?.url || '')"
              >
                <router-link :to="subItem?.url || '/'">
                  <component :is="subItem.icon" v-if="subItem.icon" />
                  <span>{{ subItem.title }}</span>
                </router-link>
              </UiDropdownMenuItem>
            </UiDropdownMenuContent>
          </UiDropdownMenu>
        </UiSidebarMenuItem>
      </template>
    </UiSidebarMenu>
  </UiSidebarGroup>
</template>
