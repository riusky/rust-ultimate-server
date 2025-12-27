<script lang="ts" setup>
import { Check, ChevronsUpDown, Plus } from 'lucide-vue-next'

import { useSidebar } from '@/components/ui/sidebar'

import type { Team } from './types'

// 定义事件
const emit = defineEmits<{
  teamChange: [team: Team]
}>()

const { teams, activeTeam: externalActiveTeam } = defineProps<{
  teams: Team[]
  activeTeam?: Team | null
}>()

// 内部状态
const internalActiveTeam = ref<Team | null>(teams.length > 0 ? teams[0]! : null)

// 响应外部 activeTeam 变化
watch(
  () => externalActiveTeam,
  (newTeam) => {
    if (newTeam) {
      internalActiveTeam.value = newTeam
    }
  },
  { immediate: true },
)

// 提供团队切换功能给父组件
function setActiveTeam(team: Team) {
  internalActiveTeam.value = team
  emit('teamChange', team)
}

defineExpose({
  setActiveTeam,
})

const { isMobile, open } = useSidebar()

const isOpen = ref(false)

const showComponent = shallowRef<Component | null>(null)
type TComponent = 'team-add'

function handleSelect(command: TComponent) {
  switch (command) {
    case 'team-add':
      showComponent.value = defineAsyncComponent(() => import('./nav-team-add.vue'))
      break
  }
}

// 计算当前显示的团队
const displayedTeam = computed(() => {
  if (externalActiveTeam !== undefined) {
    return externalActiveTeam
  }
  return internalActiveTeam.value
})
</script>

<template>
  <UiSidebarMenu>
    <UiSidebarMenuItem>
      <UiDialog v-model:open="isOpen">
        <UiDropdownMenu>
          <UiDropdownMenuTrigger as-child>
            <UiSidebarMenuButton
              size="lg"
              class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <div
                v-if="displayedTeam"
                class="flex items-center justify-center rounded-lg aspect-square size-8 bg-sidebar-primary text-sidebar-primary-foreground"
              >
                <component :is="displayedTeam.logo" class="size-4" />
              </div>
              <div v-if="displayedTeam" class="grid flex-1 text-sm leading-tight text-left">
                <span class="font-semibold truncate">{{ displayedTeam.name }}</span>
                <span class="text-xs truncate">{{ displayedTeam.plan }}</span>
              </div>
              <ChevronsUpDown class="ml-auto" />
            </UiSidebarMenuButton>
          </UiDropdownMenuTrigger>
          <UiDropdownMenuContent
            class="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
            align="start"
            :side="isMobile || open ? 'bottom' : 'right'"
            :side-offset="4"
          >
            <UiDropdownMenuLabel class="text-xs text-muted-foreground"> Teams </UiDropdownMenuLabel>
            <UiDropdownMenuItem
              v-for="(team, index) in teams"
              :key="team.name"
              class="gap-2 p-2"
              @click="setActiveTeam(team)"
            >
              <div class="flex items-center justify-center border rounded-sm size-6">
                <component :is="team.logo" class="size-4 shrink-0" />
              </div>
              {{ team.name }}
              <Check
                v-if="displayedTeam?.name === team.name"
                class="ml-auto size-4 text-primary"
              />
              <UiDropdownMenuShortcut v-else>⌘{{ index + 1 }}</UiDropdownMenuShortcut>
            </UiDropdownMenuItem>
            <UiDropdownMenuSeparator />

            <UiDialogTrigger as-child>
              <UiDropdownMenuItem class="gap-2 p-2" @click.stop="handleSelect('team-add')">
                <div
                  class="flex items-center justify-center border rounded-md size-6 bg-background"
                >
                  <Plus class="size-4" />
                </div>
                <div class="font-medium text-muted-foreground">Add team</div>
              </UiDropdownMenuItem>
            </UiDialogTrigger>
          </UiDropdownMenuContent>
        </UiDropdownMenu>

        <UiDialogContent>
          <component :is="showComponent" @close="isOpen = false" />
        </UiDialogContent>
      </UiDialog>
    </UiSidebarMenuItem>
  </UiSidebarMenu>
</template>
