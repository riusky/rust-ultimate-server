<script setup lang="ts">
// import { Minus, Square, X, Pin, PinOff } from 'lucide-vue-next'
// import { Window } from '@tauri-apps/api/window'
// import { ref } from 'vue'

// const appWindow = Window.getCurrent()
// const isMaximized = ref(false)
// const isAlwaysOnTop = ref(false)

// const minimizeWindow = () => appWindow.minimize()

// const toggleMaximizeWindow = async () => {
//   await appWindow.toggleMaximize()
//   isMaximized.value = await appWindow.isMaximized()
// }

// const closeWindow = () => appWindow.close()

// const toggleAlwaysOnTop = async () => {
//   isAlwaysOnTop.value = !isAlwaysOnTop.value
//   await appWindow.setAlwaysOnTop(isAlwaysOnTop.value)
// }

/*

  <div class="window-controls flex items-center gap-1">
    <!-- 窗口置顶按钮 -->
    <UiButton
      variant="outline"
      size="icon"
      @click="toggleAlwaysOnTop"
      aria-label="Toggle always on top"
      class="hover:!bg-accent hover:text-accent-foreground"
      :class="{ '!bg-accent text-accent-foreground': isAlwaysOnTop }"
    >
      <component :is="isAlwaysOnTop ? Pin : PinOff" class="h-3.5 w-3.5" />
    </UiButton>

    <!-- 最小化按钮 -->
    <UiButton
      variant="outline"
      size="icon"
      class="hover:!bg-accent hover:text-accent-foreground"
      @click="minimizeWindow"
      aria-label="Minimize window"
    >
      <Minus class="h-3.5 w-3.5" />
    </UiButton>

    <!-- 最大化/还原按钮 -->
    <UiButton
      variant="outline"
      size="icon"
      class="hover:!bg-accent hover:text-accent-foreground"
      @click="toggleMaximizeWindow"
      aria-label="Toggle maximize"
    >
      <component
        :is="isMaximized ? 'svg' : Square"
        class="h-3.5 w-3.5"
        v-if="isMaximized"
        viewBox="0 0 14 14"
        fill="none"
        stroke="currentColor"
      >
        <path d="M4 4H10V10H4V4Z" stroke-width="1.5" />
        <path d="M7 7V10H10" stroke-width="1.5" />
        <path d="M7 7H4V4" stroke-width="1.5" />
      </component>
      <Square v-else class="h-3.5 w-3.5" />
    </UiButton>

    <!-- 关闭按钮 -->
    <UiButton
      variant="outline"
      size="icon"
      @click="closeWindow"
      aria-label="Close window"
      class="hover:!bg-destructive/10 hover:text-destructive"
    >
      <X class="h-3.5 w-3.5" />
    </UiButton>
  </div>

*/


</script>

<template>



</template>
