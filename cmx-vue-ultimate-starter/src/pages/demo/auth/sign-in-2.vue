<script setup lang="ts">
import LoginForm from './components/login-form.vue'
import SignUp from './sign-up.vue'
import { ref, onMounted } from 'vue'
import logo from '@/assets/app-icon.png'
// 新增：导入粒子配置
const particleOptions = {
  background: {
    color: 'transparent',
  },
  fpsLimit: 60,
  interactivity: {
    events: {
      onHover: {
        enable: true,
        mode: 'grab',
      },
    },
    modes: {
      grab: {
        distance: 140,
        duration: 0.4,
      },
    },
  },
  particles: {
    color: {
      value: ['#3b82f6', '#8b5cf6', '#ec4899'], // 蓝紫粉色系粒子
    },
    links: {
      color: '#a5b4fc',
      distance: 120,
      enable: true,
      opacity: 0.4,
      width: 1,
    },
    move: {
      enable: true,
      speed: 1.5,
      direction: 'none',
      outModes: {
        default: 'bounce',
      },
    },
    number: {
      value: 60,
    },
    opacity: {
      value: 0.7,
    },
    shape: {
      type: 'circle',
    },
    size: {
      value: { min: 1, max: 3 },
    },
  },
  detectRetina: true,
}

// 登录场景的欢迎文案
const loginWelcomeMessages = [
  '欢迎回到普联软件!',
  '很高兴再次见到您',
  '继续您的精彩旅程',
  '海内存知己，天涯若比邻',
  '技术为舟，文化为帆',
  '感谢您的信任与支持',
]

// 注册场景的欢迎文案
const signupWelcomeMessages = [
  '欢迎加入普联软件!',
  '我们的名字源自两句古诗...',
  '海内存知己，天涯若比邻',
  '鸢飞戾天者，望峰息心',
  '海纳百川，象征包容与汇聚',
  '鸢飞戾天，代表自由与高度',
  '开启您的创新之旅',
  '期待与您共创未来',
]

const currentMessage = ref('')
const currentIndex = ref(0)
const isTyping = ref(true)
const isLoginOnLeft = ref(true)

// 控制显示哪个组件：'login' 或 'signup'
const currentView = ref<'login' | 'signup'>('login')
const showComponent = ref(false)

// 定时器引用，用于清理
let typingInterval: ReturnType<typeof setInterval> | null = null
let deletingInterval: ReturnType<typeof setInterval> | null = null

// 清理所有定时器
const clearAllIntervals = () => {
  if (typingInterval) {
    clearInterval(typingInterval)
    typingInterval = null
  }
  if (deletingInterval) {
    clearInterval(deletingInterval)
    deletingInterval = null
  }
}

// 根据当前视图获取对应的欢迎文案
const getCurrentMessages = () => {
  return currentView.value === 'login' ? loginWelcomeMessages : signupWelcomeMessages
}

// 打字机效果
const typeWriter = () => {
  clearAllIntervals() // 清理之前的定时器
  const messages = getCurrentMessages()
  const message = messages[currentIndex.value] ?? ''
  let charIndex = 0

  typingInterval = setInterval(() => {
    if (charIndex < message.length) {
      currentMessage.value = message.substring(0, charIndex + 1)
      charIndex++
    } else {
      if (typingInterval) clearInterval(typingInterval)
      isTyping.value = false
      setTimeout(() => {
        deleteText()
      }, 1500)
    }
  }, 100)
}

// 删除文本效果
const deleteText = () => {
  clearAllIntervals() // 清理之前的定时器
  isTyping.value = true
  let charIndex = currentMessage.value.length

  deletingInterval = setInterval(() => {
    if (charIndex > 0) {
      currentMessage.value = currentMessage.value.substring(0, charIndex - 1)
      charIndex--
    } else {
      if (deletingInterval) clearInterval(deletingInterval)
      isTyping.value = false
      const messages = getCurrentMessages()
      currentIndex.value = (currentIndex.value + 1) % messages.length
      setTimeout(() => {
        typeWriter()
      }, 800)
    }
  }, 50)
}

const toggleLayout = () => {
  // 立即停止所有计时器和动画效果
  clearAllIntervals()

  // 立即停止打字状态
  isTyping.value = false

  // 立即清空当前文案，避免闪动
  currentMessage.value = ''

  // 先隐藏当前组件（透明度动画会与平移同步进行）
  showComponent.value = false

  // 同步触发左右位置切换
  isLoginOnLeft.value = !isLoginOnLeft.value

  // 等待300ms后再开始显示新组件和启动动画（更快响应）
  setTimeout(() => {
    // 交替切换登录和注册
    currentView.value = currentView.value === 'login' ? 'signup' : 'login'

    // 重置文案索引
    currentIndex.value = 0

    // 重新启动打字机效果
    isTyping.value = true
    typeWriter()

    // 显示新组件（淡入动画会与平移同步进行）
    requestAnimationFrame(() => {
      showComponent.value = true
    })
  }, 300)
}

onMounted(() => {
  typeWriter()
  // 初始显示登录表单
  setTimeout(() => {
    showComponent.value = true
  }, 300)
})
</script>

<template>
  <div class="w-full h-[calc(100vh-3rem)] flex overflow-hidden bg-background">
    <!-- 新增：粒子背景（全屏覆盖） -->
    <vue-particles id="tsparticles-bg" class="absolute inset-0 z-20" :options="particleOptions" />

    <!-- 登录和注册表单区域 -->
    <div
      class="flex-1 flex items-center justify-center transition-all duration-500 ease-[cubic-bezier(0.65,0.05,0.36,1)] z-10 overflow-hidden"
      :style="{
        transform: isLoginOnLeft ? 'translateX(0)' : 'translateX(100%)',
        borderRadius: isLoginOnLeft ? '0 120px 120px 0' : '120px 0 0 120px',
      }"
    >
      <div class="w-[400px] max-w-full p-6 relative">
        <!-- 根据 currentView 显示对应组件 -->
        <Transition name="form-slide" mode="out-in">
          <div
            :key="currentView"
            class="transition-all duration-500 ease-[cubic-bezier(0.34,1.56,0.64,1)]"
            :style="{
              transform: showComponent
                ? 'translateX(0) translateZ(0px) scale(1)'
                : 'translateZ(-30px) scale(0.95)',
              opacity: showComponent ? 1 : 0,
            }"
          >
            <!-- LoginForm -->
            <LoginForm v-if="currentView === 'login'" :toggle-layout="toggleLayout" />

            <!-- SignUp -->
            <SignUp v-else-if="currentView === 'signup'" :toggle-layout="toggleLayout" />
          </div>
        </Transition>
      </div>
    </div>

    <!-- 欢迎区域 -->
    <div
      class="flex-1 flex items-center mt-2 mb-2 justify-center bg-muted transition-all duration-700 ease-[cubic-bezier(0.65,0.05,0.36,1)] z-10 overflow-hidden"
      :style="{
        transform: isLoginOnLeft ? 'translateX(0)' : 'translateX(-100%)',
        borderRadius: isLoginOnLeft ? '120px 0 0 120px' : '0 120px 120px 0',
      }"
    >
      <div class="w-full max-w-md px-8 py-12 text-center">
        <h2 class="text-3xl font-bold text-primary min-h-[48px] mb-6">
          {{ currentMessage }}<span class="typing-cursor" :class="{ blink: isTyping }">|</span>
        </h2>
        <div class="w-48 h-48 mx-auto mb-8">
          <img
            :src="logo"
            alt="App Icon"
            class="w-full h-full object-contain animate-float"
          />
        </div>
        <p class="text-muted-foreground">
          {{ currentView === 'login' ? '普联软件 - 连接创造价值' : '普联软件 - 开启创新之旅' }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.typing-cursor {
  display: inline-block;
  margin-left: 2px;
  color: currentColor;
}

.blink {
  animation: blink 0.7s infinite;
}

@keyframes blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0;
  }
}

.animate-float {
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

/* 在您的style标签中添加 */
#tsparticles-bg {
  pointer-events: none;
}

/* 3D 变换效果 */
.transition-all {
  transform-style: preserve-3d;
  perspective: 1000px;
}

/* 表单平移+3D过渡效果 */
.form-slide-enter-active {
  transition: all 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.form-slide-leave-active {
  transition: all 0.3s cubic-bezier(0.65, 0.05, 0.36, 1);
}

.form-slide-enter-from {
  opacity: 0;
  transform: translateX(60px) translateZ(-30px) scale(0.95);
}

.form-slide-leave-to {
  opacity: 0;
  transform: translateX(-60px) translateZ(-30px) scale(0.95);
}
</style>

<route lang="yaml">
meta:
  layout: default-base
</route>
