<script setup lang="ts">
import {
  BadgeCheck,
  Bell,
  ChevronsUpDown,
  CreditCard,
  LogOut,
  Sparkles,
  UserRoundCog,
} from 'lucide-vue-next'

import { useSidebar } from '@/components/ui/sidebar'
import { getCurrentUserInfoSilent } from '@/services/api/rpc/userinfo/user-info'
import type { UserInfo } from '@/services/types/user/index'

import type { User } from './types'

const { user } = defineProps<{ user: User }>()

const { logout } = useAuth()
const { isMobile, open } = useSidebar()

// User info state
const userInfo = ref<UserInfo | null>(null)
const isLoading = ref(false)
const hasLoaded = ref(false)

// Computed display values
const displayName = computed(() => userInfo.value?.nickname || user.name)
const displayEmail = computed(() => userInfo.value?.email || user.email)
const displayAvatar = computed(() => userInfo.value?.avatar || user.avatar)

// Fetch user info when dropdown opens
async function fetchUserInfo() {
  if (hasLoaded.value || isLoading.value) return

  isLoading.value = true
  try {
    userInfo.value = await getCurrentUserInfoSilent()
    hasLoaded.value = true
  } catch (e) {
    console.error('Failed to fetch user info:', e)
  } finally {
    isLoading.value = false
  }
}

// Fetch on mount for immediate display
onMounted(() => {
  fetchUserInfo()
})
</script>

<template>
  <UiSidebarMenu>
    <UiSidebarMenuItem>
      <UiDropdownMenu>
        <UiDropdownMenuTrigger as-child>
          <UiSidebarMenuButton
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <UiAvatar class="size-8 rounded-lg">
              <UiAvatarImage :src="displayAvatar" :alt="displayName" />
              <UiAvatarFallback class="rounded-lg">
                {{ displayName?.slice(0, 2).toUpperCase() || 'CN' }}
              </UiAvatarFallback>
            </UiAvatar>
            <div class="grid flex-1 text-sm leading-tight text-left">
              <span class="font-semibold truncate">{{ displayName }}</span>
              <span class="text-xs truncate">{{ displayEmail }}</span>
            </div>
            <ChevronsUpDown class="ml-auto size-4" />
          </UiSidebarMenuButton>
        </UiDropdownMenuTrigger>
        <UiDropdownMenuContent
          class="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
          :side="isMobile || open ? 'bottom' : 'right'"
          align="start"
          :side-offset="4"
        >
          <UiDropdownMenuLabel class="p-0 font-normal">
            <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
              <UiAvatar class="size-8 rounded-lg">
                <UiAvatarImage :src="displayAvatar" :alt="displayName" />
                <UiAvatarFallback class="rounded-lg">
                  {{ displayName?.slice(0, 2).toUpperCase() || 'CN' }}
                </UiAvatarFallback>
              </UiAvatar>
              <div class="grid flex-1 text-sm leading-tight text-left">
                <span class="font-semibold truncate">{{ displayName }}</span>
                <span class="text-xs truncate">{{ displayEmail }}</span>
              </div>
            </div>
          </UiDropdownMenuLabel>

          <UiDropdownMenuSeparator />
          <UiDropdownMenuGroup>
            <UiDropdownMenuItem @click="$router.push('/demo/billing/')">
              <Sparkles />
              Upgrade to Pro
            </UiDropdownMenuItem>
          </UiDropdownMenuGroup>

          <UiDropdownMenuSeparator />
          <UiDropdownMenuGroup>
            <UiDropdownMenuItem @click="$router.push('/demo/billing?type=billing')">
              <CreditCard />
              Billing
            </UiDropdownMenuItem>
          </UiDropdownMenuGroup>

          <UiDropdownMenuSeparator />
          <UiDropdownMenuGroup>
            <UiDropdownMenuItem @click="$router.push('/demo/settings/')">
              <UserRoundCog />
              Profile
            </UiDropdownMenuItem>
            <UiDropdownMenuItem @click="$router.push('/demo/settings/account')">
              <BadgeCheck />
              Account
            </UiDropdownMenuItem>
            <UiDropdownMenuItem @click="$router.push('/demo/settings/notifications')">
              <Bell />
              Notifications
            </UiDropdownMenuItem>
          </UiDropdownMenuGroup>

          <UiDropdownMenuSeparator />
          <UiDropdownMenuItem @click="logout">
            <LogOut />
            Log out
          </UiDropdownMenuItem>
        </UiDropdownMenuContent>
      </UiDropdownMenu>
    </UiSidebarMenuItem>
  </UiSidebarMenu>
</template>
