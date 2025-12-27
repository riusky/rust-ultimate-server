<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="帮助中心" description="在这里找到您需要的帮助和支持信息">
    <div class="max-w-4xl mx-auto space-y-8">
      <!-- 搜索区域 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">帮助中心</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          查找常见问题的答案，了解如何使用我们的产品和服务
        </p>
        <div class="relative max-w-2xl mx-auto">
          <UiInput
            placeholder="搜索帮助内容..."
            class="pl-10 pr-4 py-2 text-base"
            v-model="searchQuery"
          />
          <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
            <UiSearch class="w-5 h-5 text-muted-foreground" />
          </div>
        </div>
      </div>

      <!-- 分类导航 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <UiCard
          v-for="category in categories"
          :key="category.id"
          class="cursor-pointer hover:shadow-md transition-shadow"
          @click="selectedCategory = category.id"
        >
          <UiCardContent class="p-6 text-center">
            <div
              class="w-12 h-12 mx-auto mb-3 rounded-lg bg-primary/10 flex items-center justify-center"
            >
              <component :is="category.icon" class="w-6 h-6 text-primary" />
            </div>
            <h3 class="font-semibold mb-1">{{ category.title }}</h3>
            <p class="text-sm text-muted-foreground">{{ category.description }}</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 热门问题 -->
      <div class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold">热门问题</h2>
          <UiButton variant="outline" @click="showAllFAQs = !showAllFAQs">
            {{ showAllFAQs ? '收起' : '查看全部' }}
          </UiButton>
        </div>

        <div class="space-y-3">
          <UiCard
            v-for="faq in visibleFAQs"
            :key="faq.id"
            class="cursor-pointer"
            @click="toggleFAQ(faq.id)"
          >
            <UiCardContent class="p-4">
              <div class="flex items-center justify-between">
                <h3 class="font-medium">{{ faq.question }}</h3>
                <UiChevronDown
                  class="w-5 h-5 text-muted-foreground transition-transform"
                  :class="{ 'rotate-180': openFAQs.includes(faq.id) }"
                />
              </div>
              <div
                v-if="openFAQs.includes(faq.id)"
                class="mt-3 pt-3 border-t text-muted-foreground"
              >
                <p>{{ faq.answer }}</p>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 联系支持 -->
      <UiCard class="bg-muted/50">
        <UiCardContent class="p-6">
          <div class="text-center space-y-4">
            <h2 class="text-2xl font-bold">需要更多帮助？</h2>
            <p class="text-muted-foreground">如果找不到您需要的答案，我们的支持团队随时为您服务</p>
            <div class="flex flex-col sm:flex-row gap-3 justify-center">
              <UiButton @click="contactSupport">
                <UiMessageCircle class="w-4 h-4 mr-2" />
                联系支持
              </UiButton>
              <UiButton variant="outline" @click="openLiveChat">
                <UiHeadphones class="w-4 h-4 mr-2" />
                在线客服
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import { BookOpen, Settings, CreditCard, User } from 'lucide-vue-next'

// 分类数据
const categories = [
  {
    id: 'getting-started',
    title: '入门指南',
    description: '新用户快速上手',
    icon: BookOpen,
  },
  {
    id: 'account',
    title: '账户管理',
    description: '账户设置和安全',
    icon: User,
  },
  {
    id: 'billing',
    title: '账单支付',
    description: '订阅和付款问题',
    icon: CreditCard,
  },
  {
    id: 'technical',
    title: '技术支持',
    description: '技术问题和故障排除',
    icon: Settings,
  },
]

// FAQ数据
const faqs = [
  {
    id: 1,
    category: 'getting-started',
    question: '如何开始使用这个系统？',
    answer:
      '要开始使用系统，请先完成账户注册，然后按照引导完成初始设置。您可以在仪表板中找到快速入门指南，或者查看我们的详细教程文档。',
  },
  {
    id: 2,
    category: 'account',
    question: '如何重置密码？',
    answer:
      '如果您忘记了密码，可以在登录页面点击"忘记密码"链接，按照提示输入您的注册邮箱，系统将发送重置密码的链接到您的邮箱。',
  },
  {
    id: 3,
    category: 'billing',
    question: '如何升级我的订阅计划？',
    answer:
      '您可以在账户设置中的"订阅管理"页面升级您的计划。选择您想要的新计划，系统将自动计算差价并引导您完成支付流程。',
  },
  {
    id: 4,
    category: 'technical',
    question: '系统出现错误代码怎么办？',
    answer:
      '如果遇到系统错误代码，请先记录下具体的错误代码和出现问题的操作步骤。然后联系我们的技术支持团队，提供这些信息以便我们快速定位和解决问题。',
  },
  {
    id: 5,
    category: 'account',
    question: '如何修改个人资料信息？',
    answer:
      '登录后点击右上角的用户头像，选择"个人资料"即可进入编辑页面。您可以修改姓名、头像、联系方式等信息。',
  },
  {
    id: 6,
    category: 'billing',
    question: '发票如何获取？',
    answer:
      '所有成功的支付都会自动生成电子发票。您可以在"账单历史"页面查看和下载您的发票。如果需要纸质发票，请联系客服申请。',
  },
  {
    id: 7,
    category: 'technical',
    question: '系统支持哪些浏览器？',
    answer:
      '我们推荐使用最新版本的 Chrome、Firefox、Safari 或 Edge 浏览器。系统在移动设备上的 Safari 和 Chrome 浏览器中也能良好运行。',
  },
  {
    id: 8,
    category: 'getting-started',
    question: '有没有视频教程？',
    answer:
      '是的，我们提供了完整的视频教程系列，涵盖从基础操作到高级功能的所有内容。您可以在帮助中心的"视频教程"部分找到这些资源。',
  },
]

// 响应式数据
const searchQuery = ref('')
const selectedCategory = ref('all')
const openFAQs = ref<number[]>([])
const showAllFAQs = ref(false)

// 计算属性
const filteredFAQs = computed(() => {
  let filtered = faqs

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (faq) =>
        faq.question.toLowerCase().includes(query) || faq.answer.toLowerCase().includes(query),
    )
  }

  // 分类过滤
  if (selectedCategory.value !== 'all') {
    filtered = filtered.filter((faq) => faq.category === selectedCategory.value)
  }

  return filtered
})

const visibleFAQs = computed(() => {
  return showAllFAQs.value ? filteredFAQs.value : filteredFAQs.value.slice(0, 4)
})

// 方法
const toggleFAQ = (id: number) => {
  const index = openFAQs.value.indexOf(id)
  if (index > -1) {
    openFAQs.value.splice(index, 1)
  } else {
    openFAQs.value.push(id)
  }
}

const contactSupport = () => {
  console.log('联系支持')
  // 这里可以实现联系支持的功能
}

const openLiveChat = () => {
  console.log('打开在线客服')
  // 这里可以实现打开在线客服的功能
}
</script>

<style scoped></style>
