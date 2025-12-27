<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="常见问题" description="在这里找到常见问题的答案和解决方案">
    <div class="max-w-4xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">常见问题</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          我们整理了用户最常遇到的问题和解决方案
        </p>
      </div>

      <!-- 搜索区域 -->
      <div class="relative">
        <UiInput
          placeholder="搜索问题..."
          class="pl-10 pr-4 py-2 text-base"
          v-model="searchQuery"
        />
        <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
          <UiSearch class="w-5 h-5 text-muted-foreground" />
        </div>
      </div>

      <!-- 分类导航 -->
      <div class="flex flex-wrap gap-2">
        <UiButton
          v-for="category in categories"
          :key="category.id"
          :variant="selectedCategory === category.id ? 'default' : 'outline'"
          size="sm"
          @click="selectedCategory = category.id"
        >
          {{ category.title }}
        </UiButton>
      </div>

      <!-- FAQ列表 -->
      <div class="space-y-4">
        <div v-for="faq in filteredFAQs" :key="faq.id" class="border rounded-lg overflow-hidden">
          <button
            class="flex items-center justify-between w-full p-4 text-left hover:bg-muted/50 transition-colors"
            @click="toggleFAQ(faq.id)"
          >
            <h3 class="font-medium text-base">{{ faq.question }}</h3>
            <UiChevronDown
              class="w-5 h-5 text-muted-foreground transition-transform shrink-0"
              :class="{ 'rotate-180': openFAQs.includes(faq.id) }"
            />
          </button>
          <div v-if="openFAQs.includes(faq.id)" class="px-4 pb-4 border-t">
            <div class="pt-4 text-muted-foreground">
              <p class="whitespace-pre-line">{{ faq.answer }}</p>
              <div v-if="faq.related" class="mt-3 pt-3 border-t">
                <p class="text-sm font-medium mb-2">相关链接：</p>
                <ul class="text-sm space-y-1">
                  <li v-for="link in faq.related" :key="link.url">
                    <a :href="link.url" class="text-primary hover:underline">{{ link.text }}</a>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div
        v-if="filteredFAQs.length === 0"
        class="flex flex-col items-center justify-center py-12 text-center border-2 border-dashed rounded-lg"
      >
        <div class="w-16 h-16 mb-4 rounded-full bg-muted flex items-center justify-center">
          <UiSearch class="w-8 h-8 text-muted-foreground" />
        </div>
        <h3 class="text-lg font-medium mb-2">没有找到相关问题</h3>
        <p class="text-muted-foreground">尝试调整搜索关键词或分类</p>
      </div>

      <!-- 联系支持 -->
      <UiCard class="bg-muted/50">
        <UiCardContent class="p-6">
          <div class="text-center space-y-4">
            <h2 class="text-2xl font-bold">没有找到您的问题？</h2>
            <p class="text-muted-foreground">如果这里没有您需要的答案，请联系我们的支持团队</p>
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

// FAQ数据
const faqs = [
  {
    id: 1,
    category: 'account',
    question: '如何注册账户？',
    answer:
      '要注册账户，请访问我们的网站首页，点击"注册"按钮。您需要提供有效的邮箱地址、设置密码，并完成邮箱验证。注册完成后，您可以完善个人资料信息。',
    related: [
      { text: '注册流程详细说明', url: '/help/tutorials' },
      { text: '邮箱验证问题', url: '/help/faq' },
    ],
  },
  {
    id: 2,
    category: 'account',
    question: '忘记密码怎么办？',
    answer:
      '如果您忘记了密码，可以在登录页面点击"忘记密码"链接。输入您的注册邮箱，系统将发送重置密码的链接到您的邮箱。点击链接后，您可以设置新的密码。\n\n注意：重置链接有效期为24小时。',
    related: [{ text: '密码安全指南', url: '/help/tutorials' }],
  },
  {
    id: 3,
    category: 'billing',
    question: '如何升级订阅计划？',
    answer:
      '您可以在账户设置中的"订阅管理"页面升级您的计划。选择您想要的新计划，系统将自动计算差价并引导您完成支付流程。升级后，新功能将立即生效。',
    related: [
      { text: '订阅计划对比', url: '/billing' },
      { text: '支付方式管理', url: '/help/faq' },
    ],
  },
  {
    id: 4,
    category: 'billing',
    question: '如何获取发票？',
    answer:
      '所有成功的支付都会自动生成电子发票。您可以在"账单历史"页面查看和下载您的发票。如果需要纸质发票，请联系客服申请，我们将在3-5个工作日内寄出。',
    related: [{ text: '发票开具说明', url: '/help/tutorials' }],
  },
  {
    id: 5,
    category: 'technical',
    question: '系统支持哪些浏览器？',
    answer:
      '我们推荐使用最新版本的 Chrome、Firefox、Safari 或 Edge 浏览器。系统在移动设备上的 Safari 和 Chrome 浏览器中也能良好运行。\n\n最低要求：\n- Chrome 90+\n- Firefox 88+\n- Safari 14+\n- Edge 90+',
    related: [{ text: '浏览器兼容性说明', url: '/help/tutorials' }],
  },
  {
    id: 6,
    category: 'technical',
    question: '遇到系统错误怎么办？',
    answer:
      '如果遇到系统错误，请先记录下具体的错误代码和出现问题的操作步骤。然后联系我们的技术支持团队，提供这些信息以便我们快速定位和解决问题。\n\n您也可以尝试以下步骤：\n1. 刷新页面重试\n2. 清除浏览器缓存\n3. 检查网络连接',
    related: [{ text: '技术支持联系方式', url: '/help/contact' }],
  },
  {
    id: 7,
    category: 'usage',
    question: '如何导入数据？',
    answer:
      '系统支持多种数据导入方式：\n\n1. CSV文件导入：在数据管理页面选择"导入"，上传CSV文件\n2. Excel导入：支持.xlsx格式文件\n3. API导入：通过REST API批量导入数据\n\n导入前请确保数据格式正确，具体格式要求请参考我们的数据导入指南。',
    related: [
      { text: '数据导入指南', url: '/help/tutorials' },
      { text: 'API文档', url: '/demo/api-docs' },
    ],
  },
  {
    id: 8,
    category: 'usage',
    question: '如何设置团队协作？',
    answer:
      '要设置团队协作，请按以下步骤操作：\n\n1. 在团队管理页面创建团队\n2. 邀请团队成员加入\n3. 设置团队成员权限\n4. 创建共享项目和任务\n\n团队成员加入后，可以共同编辑文档、分配任务和实时沟通。',
    related: [{ text: '团队协作教程', url: '/help/tutorials' }],
  },
  {
    id: 9,
    category: 'security',
    question: '如何启用双重验证？',
    answer:
      '启用双重验证可以大大提高账户安全性：\n\n1. 进入账户设置 > 安全\n2. 点击"启用双重验证"\n3. 使用身份验证应用扫描二维码\n4. 输入生成的验证码确认\n\n启用后，每次登录都需要输入密码和动态验证码。',
    related: [{ text: '账户安全设置', url: '/help/tutorials' }],
  },
  {
    id: 10,
    category: 'security',
    question: '数据如何备份？',
    answer:
      '系统会自动进行数据备份：\n\n- 每日自动备份\n- 备份保留30天\n- 支持手动导出数据\n\n您也可以在设置中配置自动备份频率，或手动导出数据到本地。',
    related: [{ text: '数据备份和恢复', url: '/help/tutorials' }],
  },
]

// 分类数据
const categories = [
  { id: 'all', title: '全部' },
  { id: 'account', title: '账户问题' },
  { id: 'billing', title: '账单支付' },
  { id: 'technical', title: '技术问题' },
  { id: 'usage', title: '使用问题' },
  { id: 'security', title: '安全设置' },
]

// 响应式数据
const searchQuery = ref('')
const selectedCategory = ref('all')
const openFAQs = ref<number[]>([])

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
