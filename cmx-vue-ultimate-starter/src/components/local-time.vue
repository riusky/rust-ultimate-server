<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { currentLocale } from '@/plugins/i18n'

// 定义可用的显示模式
type DisplayMode = 'full' | 'date' | 'time' | 'compact'

const props = defineProps({
  // 初始显示模式
  initialMode: {
    type: String as () => DisplayMode,
    default: 'full',
  },
  // 是否允许点击切换
  clickable: {
    type: Boolean,
    default: true,
  },
})

const now = ref(new Date())
const currentMode = ref<DisplayMode>(props.initialMode)

// 定义每种模式的格式选项
const formatOptions = computed(() => {
  const options: Record<DisplayMode, Intl.DateTimeFormatOptions> = {
    full: {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    },
    date: {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    },
    time: {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    },
    compact: {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    },
  }
  return options[currentMode.value]
})

// 格式化时间
const formattedTime = computed(() => {
  try {
    return new Intl.DateTimeFormat(currentLocale.value, formatOptions.value).format(now.value)
  } catch (e) {
    console.error('DateTimeFormat error:', e)
    return now.value.toLocaleString(currentLocale.value)
  }
})

// 点击切换显示模式
const toggleDisplayMode = () => {
  if (!props.clickable) return

  const modes: DisplayMode[] = ['full', 'date', 'time', 'compact']
  const currentIndex = modes.indexOf(currentMode.value)
  const nextMode = modes[(currentIndex + 1) % modes.length]
  if (nextMode) {
    currentMode.value = nextMode
  }
}

// 自动更新时间
let timer: number
onMounted(() => {
  timer = window.setInterval(() => {
    now.value = new Date()
  }, 1000)
})

onUnmounted(() => {
  clearInterval(timer)
})
</script>

<template>
  <div
    class="font-mono select-none cursor-pointer hover:opacity-80 transition-opacity"
    :class="{ 'cursor-default': !clickable }"
    @click="toggleDisplayMode"
  >
    {{ formattedTime }}
  </div>
</template>
