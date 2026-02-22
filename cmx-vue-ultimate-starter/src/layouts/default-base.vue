<script setup lang="ts">
import ThemePopover from '@/components/custom-theme/theme-popover.vue'
import ToggleTheme from '@/components/toggle-theme.vue'
import LangToggle from '@/components/language-change.vue'
import LocalTime from '@/components/local-time.vue'
// import WindowControls from '@/components/window-controls.vue'
import logo from '@/assets/app-icon.png'
</script>

<template>
  <header
    data-tauri-drag-region
    class="sticky top-0 z-50 flex items-center gap-3 sm:gap-4 h-12 p-4 pr-2 shrink-0 transition-[width,height] ease-linear bg-sidebar text-sidebar-foreground"
  >
    <!-- 添加的图标和文字部分 -->
    <div class="flex items-center gap-2 mr-auto">
      <img :src="logo" alt="Logo" class="h-8 w-8 object-contain" />
      <span class="text-lg font-semibold hidden sm:inline-block">Riusky软件</span>
    </div>

    <div class="flex items-center gap-1">
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
  <div>
    <router-view />
  </div>
</template>
