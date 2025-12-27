<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="联系我们" description="有任何问题或建议，欢迎随时与我们联系">
    <div class="max-w-6xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">联系我们</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          我们非常重视您的反馈，随时为您提供支持和帮助
        </p>
      </div>

      <!-- 联系信息 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <UiCard v-for="contact in contactMethods" :key="contact.title">
          <UiCardContent class="p-6 text-center">
            <div
              class="w-12 h-12 mx-auto mb-3 rounded-lg bg-primary/10 flex items-center justify-center"
            >
              <component :is="contact.icon" class="w-6 h-6 text-primary" />
            </div>
            <h3 class="font-semibold mb-1">{{ contact.title }}</h3>
            <p class="text-sm text-muted-foreground mb-2">{{ contact.description }}</p>
            <p class="text-sm font-medium">{{ contact.value }}</p>
            <p class="text-xs text-muted-foreground">{{ contact.responseTime }}</p>
          </UiCardContent>
        </UiCard>
      </div>

      <!-- 联系表单和地图 -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- 联系表单 -->
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>发送消息</UiCardTitle>
            <UiCardDescription> 填写表单，我们会尽快回复您 </UiCardDescription>
          </UiCardHeader>
          <UiCardContent>
            <form @submit.prevent="handleSubmit" class="space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                  <UiLabel for="firstName" class="required">名字</UiLabel>
                  <UiInput
                    id="firstName"
                    v-model="form.firstName"
                    placeholder="请输入名字"
                    :class="{ 'border-destructive': errors.firstName }"
                  />
                  <p v-if="errors.firstName" class="text-sm text-destructive">
                    {{ errors.firstName }}
                  </p>
                </div>
                <div class="space-y-2">
                  <UiLabel for="lastName" class="required">姓氏</UiLabel>
                  <UiInput
                    id="lastName"
                    v-model="form.lastName"
                    placeholder="请输入姓氏"
                    :class="{ 'border-destructive': errors.lastName }"
                  />
                  <p v-if="errors.lastName" class="text-sm text-destructive">
                    {{ errors.lastName }}
                  </p>
                </div>
              </div>

              <div class="space-y-2">
                <UiLabel for="email" class="required">邮箱</UiLabel>
                <UiInput
                  id="email"
                  v-model="form.email"
                  type="email"
                  placeholder="请输入邮箱地址"
                  :class="{ 'border-destructive': errors.email }"
                />
                <p v-if="errors.email" class="text-sm text-destructive">{{ errors.email }}</p>
              </div>

              <div class="space-y-2">
                <UiLabel for="phone">手机号码</UiLabel>
                <UiInput
                  id="phone"
                  v-model="form.phone"
                  type="tel"
                  placeholder="请输入手机号码（可选）"
                />
              </div>

              <div class="space-y-2">
                <UiLabel for="subject" class="required">主题</UiLabel>
                <UiSelect v-model="form.subject">
                  <UiSelectTrigger>
                    <UiSelectValue placeholder="请选择咨询主题" />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="general">一般咨询</UiSelectItem>
                    <UiSelectItem value="technical">技术问题</UiSelectItem>
                    <UiSelectItem value="billing">账单问题</UiSelectItem>
                    <UiSelectItem value="feature">功能建议</UiSelectItem>
                    <UiSelectItem value="bug">错误报告</UiSelectItem>
                    <UiSelectItem value="other">其他</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
                <p v-if="errors.subject" class="text-sm text-destructive">{{ errors.subject }}</p>
              </div>

              <div class="space-y-2">
                <UiLabel for="message" class="required">消息内容</UiLabel>
                <UiTextarea
                  id="message"
                  v-model="form.message"
                  placeholder="请详细描述您的问题或建议..."
                  rows="6"
                  :class="{ 'border-destructive': errors.message }"
                />
                <p v-if="errors.message" class="text-sm text-destructive">{{ errors.message }}</p>
              </div>

              <div class="flex items-center space-x-2">
                <UiCheckbox id="agree" v-model="form.agree" />
                <UiLabel for="agree" class="text-sm"> 我同意相关条款和隐私政策 </UiLabel>
              </div>
              <p v-if="errors.agree" class="text-sm text-destructive">{{ errors.agree }}</p>

              <UiButton type="submit" :disabled="isSubmitting" class="w-full">
                <UiLoader2 v-if="isSubmitting" class="w-4 h-4 mr-2 animate-spin" />
                {{ isSubmitting ? '发送中...' : '发送消息' }}
              </UiButton>
            </form>
          </UiCardContent>
        </UiCard>

        <!-- 联系信息和地图 -->
        <div class="space-y-6">
          <!-- 公司信息 -->
          <UiCard>
            <UiCardHeader>
              <UiCardTitle>公司信息</UiCardTitle>
            </UiCardHeader>
            <UiCardContent class="space-y-4">
              <div class="flex items-center gap-3">
                <UiMapPin class="w-5 h-5 text-muted-foreground" />
                <div>
                  <p class="font-medium">地址</p>
                  <p class="text-sm text-muted-foreground">
                    北京市朝阳区建国门外大街1号国贸大厦A座
                  </p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <UiPhone class="w-5 h-5 text-muted-foreground" />
                <div>
                  <p class="font-medium">电话</p>
                  <p class="text-sm text-muted-foreground">+86 10 1234 5678</p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <UiMail class="w-5 h-5 text-muted-foreground" />
                <div>
                  <p class="font-medium">邮箱</p>
                  <p class="text-sm text-muted-foreground">contact@example.com</p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <UiClock class="w-5 h-5 text-muted-foreground" />
                <div>
                  <p class="font-medium">工作时间</p>
                  <p class="text-sm text-muted-foreground">周一至周五 9:00-18:00</p>
                </div>
              </div>
            </UiCardContent>
          </UiCard>

          <!-- 地图占位 -->
          <UiCard>
            <UiCardHeader>
              <UiCardTitle>我们的位置</UiCardTitle>
            </UiCardHeader>
            <UiCardContent>
              <div class="h-64 bg-muted/20 rounded-lg flex items-center justify-center">
                <div class="text-center text-muted-foreground">
                  <UiMapPin class="w-12 h-12 mx-auto mb-2 opacity-50" />
                  <p>地图区域</p>
                  <p class="text-sm">这里将显示公司位置的地图</p>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 提交成功提示 -->
      <UiAlert v-if="submitSuccess" variant="default" class="bg-success/10 border-success">
        <UiCheckCircle class="w-4 h-4 text-success" />
        <UiAlertTitle>消息发送成功！</UiAlertTitle>
        <UiAlertDescription> 我们已经收到您的消息，会在24小时内回复您。 </UiAlertDescription>
      </UiAlert>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'
import { Mail, Phone, MessageCircle, Headphones } from 'lucide-vue-next'

// 联系方法数据
const contactMethods = [
  {
    title: '邮箱支持',
    description: '发送邮件给我们',
    value: 'support@example.com',
    responseTime: '24小时内回复',
    icon: Mail,
  },
  {
    title: '电话支持',
    description: '直接电话沟通',
    value: '+86 400 123 4567',
    responseTime: '工作日 9:00-18:00',
    icon: Phone,
  },
  {
    title: '在线客服',
    description: '实时在线沟通',
    value: '立即开始对话',
    responseTime: '即时响应',
    icon: MessageCircle,
  },
  {
    title: '技术支持',
    description: '专业技术问题',
    value: 'tech@example.com',
    responseTime: '12小时内回复',
    icon: Headphones,
  },
]

// 表单数据
interface FormData {
  firstName: string
  lastName: string
  email: string
  phone: string
  subject: string
  message: string
  agree: boolean
}

interface Errors {
  firstName?: string
  lastName?: string
  email?: string
  subject?: string
  message?: string
  agree?: string
}

const form = ref<FormData>({
  firstName: '',
  lastName: '',
  email: '',
  phone: '',
  subject: '',
  message: '',
  agree: false,
})

const errors = ref<Errors>({})
const isSubmitting = ref(false)
const submitSuccess = ref(false)

// 表单验证
const validateForm = (): boolean => {
  const newErrors: Errors = {}

  if (!form.value.firstName.trim()) {
    newErrors.firstName = '名字不能为空'
  }

  if (!form.value.lastName.trim()) {
    newErrors.lastName = '姓氏不能为空'
  }

  if (!form.value.email.trim()) {
    newErrors.email = '邮箱不能为空'
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.value.email)) {
    newErrors.email = '请输入有效的邮箱地址'
  }

  if (!form.value.subject) {
    newErrors.subject = '请选择咨询主题'
  }

  if (!form.value.message.trim()) {
    newErrors.message = '消息内容不能为空'
  } else if (form.value.message.trim().length < 10) {
    newErrors.message = '消息内容至少需要10个字符'
  }

  if (!form.value.agree) {
    newErrors.agree = '请同意相关条款和隐私政策'
  }

  errors.value = newErrors
  return Object.keys(newErrors).length === 0
}

// 提交表单
const handleSubmit = async () => {
  if (!validateForm()) {
    return
  }

  isSubmitting.value = true
  submitSuccess.value = false

  try {
    // 模拟API调用
    await new Promise((resolve) => setTimeout(resolve, 2000))
    submitSuccess.value = true

    // 重置表单
    form.value = {
      firstName: '',
      lastName: '',
      email: '',
      phone: '',
      subject: '',
      message: '',
      agree: false,
    }
    errors.value = {}
  } catch (error) {
    console.error('提交失败:', error)
  } finally {
    isSubmitting.value = false
  }
}
</script>

<style scoped>
.required::after {
  content: ' *';
  color: rgb(239, 68, 68);
}
</style>
