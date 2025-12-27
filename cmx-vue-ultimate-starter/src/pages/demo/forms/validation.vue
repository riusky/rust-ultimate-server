<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="表单验证" description="这是一个表单验证示例，展示各种表单验证规则和错误提示">
    <div class="max-w-2xl mx-auto space-y-6">
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>表单验证示例</UiCardTitle>
          <UiCardDescription>
            包含必填验证、邮箱验证、手机号验证、密码强度验证等
          </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <form @submit.prevent="handleSubmit" class="space-y-4">
            <!-- 姓名验证 -->
            <div class="space-y-2">
              <UiLabel for="name" class="required">姓名</UiLabel>
              <UiInput
                id="name"
                v-model="form.name"
                placeholder="请输入您的姓名"
                :class="{ 'border-destructive': errors.name }"
              />
              <p v-if="errors.name" class="text-sm text-destructive">{{ errors.name }}</p>
            </div>

            <!-- 邮箱验证 -->
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

            <!-- 手机号验证 -->
            <div class="space-y-2">
              <UiLabel for="phone" class="required">手机号码</UiLabel>
              <UiInput
                id="phone"
                v-model="form.phone"
                type="tel"
                placeholder="请输入手机号码"
                :class="{ 'border-destructive': errors.phone }"
              />
              <p v-if="errors.phone" class="text-sm text-destructive">{{ errors.phone }}</p>
            </div>

            <!-- 密码验证 -->
            <div class="space-y-2">
              <UiLabel for="password" class="required">密码</UiLabel>
              <UiInput
                id="password"
                v-model="form.password"
                type="password"
                placeholder="请输入密码"
                :class="{ 'border-destructive': errors.password }"
              />
              <div class="flex items-center gap-2 mt-1">
                <div
                  class="h-1 flex-1 rounded-full"
                  :class="passwordStrength >= 1 ? 'bg-destructive' : 'bg-muted'"
                ></div>
                <div
                  class="h-1 flex-1 rounded-full"
                  :class="passwordStrength >= 2 ? 'bg-warning' : 'bg-muted'"
                ></div>
                <div
                  class="h-1 flex-1 rounded-full"
                  :class="passwordStrength >= 3 ? 'bg-success' : 'bg-muted'"
                ></div>
              </div>
              <p v-if="errors.password" class="text-sm text-destructive">{{ errors.password }}</p>
              <p v-else class="text-sm text-muted-foreground">
                密码强度:
                <span
                  :class="{
                    'text-destructive': passwordStrength === 1,
                    'text-warning': passwordStrength === 2,
                    'text-success': passwordStrength === 3,
                  }"
                >
                  {{ getPasswordStrengthText() }}
                </span>
              </p>
            </div>

            <!-- 确认密码 -->
            <div class="space-y-2">
              <UiLabel for="confirmPassword" class="required">确认密码</UiLabel>
              <UiInput
                id="confirmPassword"
                v-model="form.confirmPassword"
                type="password"
                placeholder="请再次输入密码"
                :class="{ 'border-destructive': errors.confirmPassword }"
              />
              <p v-if="errors.confirmPassword" class="text-sm text-destructive">
                {{ errors.confirmPassword }}
              </p>
            </div>

            <!-- 年龄验证 -->
            <div class="space-y-2">
              <UiLabel for="age" class="required">年龄</UiLabel>
              <UiInput
                id="age"
                v-model="form.age"
                type="number"
                placeholder="请输入年龄"
                :class="{ 'border-destructive': errors.age }"
              />
              <p v-if="errors.age" class="text-sm text-destructive">{{ errors.age }}</p>
            </div>

            <!-- 网址验证 -->
            <div class="space-y-2">
              <UiLabel for="website">个人网站</UiLabel>
              <UiInput
                id="website"
                v-model="form.website"
                placeholder="请输入网址 (可选)"
                :class="{ 'border-destructive': errors.website }"
              />
              <p v-if="errors.website" class="text-sm text-destructive">{{ errors.website }}</p>
            </div>

            <!-- 协议同意 -->
            <div class="flex items-center space-x-2">
              <UiCheckbox
                id="agree"
                v-model="form.agree"
                :class="{ 'border-destructive': errors.agree }"
              />
              <UiLabel for="agree" class="required">我同意相关条款和协议</UiLabel>
            </div>
            <p v-if="errors.agree" class="text-sm text-destructive">{{ errors.agree }}</p>

            <!-- 提交按钮 -->
            <div class="flex gap-2 pt-4">
              <UiButton type="submit" :disabled="isSubmitting">
                <UiLoader2 v-if="isSubmitting" class="w-4 h-4 mr-2 animate-spin" />
                {{ isSubmitting ? '提交中...' : '提交表单' }}
              </UiButton>
              <UiButton variant="outline" type="button" @click="resetForm">重置</UiButton>
            </div>
          </form>
        </UiCardContent>
      </UiCard>

      <!-- 验证成功提示 -->
      <UiAlert v-if="submitSuccess" variant="default" class="bg-success/10 border-success">
        <UiCheckCircle class="w-4 h-4 text-success" />
        <UiAlertTitle>提交成功！</UiAlertTitle>
        <UiAlertDescription> 表单验证通过，数据已成功提交。 </UiAlertDescription>
      </UiAlert>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref, computed, watch } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'

interface FormData {
  name: string
  email: string
  phone: string
  password: string
  confirmPassword: string
  age: string
  website: string
  agree: boolean
}

interface Errors {
  name?: string
  email?: string
  phone?: string
  password?: string
  confirmPassword?: string
  age?: string
  website?: string
  agree?: string
}

const form = ref<FormData>({
  name: '',
  email: '',
  phone: '',
  password: '',
  confirmPassword: '',
  age: '',
  website: '',
  agree: false,
})

const errors = ref<Errors>({})
const isSubmitting = ref(false)
const submitSuccess = ref(false)

// 密码强度计算
const passwordStrength = computed(() => {
  const password = form.value.password
  if (!password) return 0

  let strength = 0
  if (password.length >= 8) strength++
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) strength++
  if (/[0-9]/.test(password) && /[^a-zA-Z0-9]/.test(password)) strength++

  return strength
})

// 密码强度文本
const getPasswordStrengthText = () => {
  switch (passwordStrength.value) {
    case 1:
      return '弱'
    case 2:
      return '中'
    case 3:
      return '强'
    default:
      return '无'
  }
}

// 验证规则
const validateForm = (): boolean => {
  const newErrors: Errors = {}

  // 姓名验证
  if (!form.value.name.trim()) {
    newErrors.name = '姓名不能为空'
  } else if (form.value.name.trim().length < 2) {
    newErrors.name = '姓名至少需要2个字符'
  }

  // 邮箱验证
  if (!form.value.email.trim()) {
    newErrors.email = '邮箱不能为空'
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.value.email)) {
    newErrors.email = '请输入有效的邮箱地址'
  }

  // 手机号验证
  if (!form.value.phone.trim()) {
    newErrors.phone = '手机号码不能为空'
  } else if (!/^1[3-9]\d{9}$/.test(form.value.phone)) {
    newErrors.phone = '请输入有效的手机号码'
  }

  // 密码验证
  if (!form.value.password) {
    newErrors.password = '密码不能为空'
  } else if (form.value.password.length < 8) {
    newErrors.password = '密码至少需要8个字符'
  } else if (!/(?=.*[a-z])(?=.*[A-Z])/.test(form.value.password)) {
    newErrors.password = '密码必须包含大小写字母'
  } else if (!/(?=.*\d)(?=.*[^a-zA-Z0-9])/.test(form.value.password)) {
    newErrors.password = '密码必须包含数字和特殊字符'
  }

  // 确认密码验证
  if (!form.value.confirmPassword) {
    newErrors.confirmPassword = '请确认密码'
  } else if (form.value.password !== form.value.confirmPassword) {
    newErrors.confirmPassword = '两次输入的密码不一致'
  }

  // 年龄验证
  if (!form.value.age) {
    newErrors.age = '年龄不能为空'
  } else {
    const age = parseInt(form.value.age)
    if (isNaN(age) || age < 1 || age > 150) {
      newErrors.age = '请输入有效的年龄 (1-150)'
    }
  }

  // 网址验证 (可选)
  if (form.value.website && !/^https?:\/\/.+\..+/.test(form.value.website)) {
    newErrors.website = '请输入有效的网址'
  }

  // 协议同意验证
  if (!form.value.agree) {
    newErrors.agree = '请同意相关条款和协议'
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

  // 模拟API调用
  try {
    await new Promise((resolve) => setTimeout(resolve, 2000))
    submitSuccess.value = true
    resetForm()
  } catch (error) {
    console.error('提交失败:', error)
  } finally {
    isSubmitting.value = false
  }
}

// 重置表单
const resetForm = () => {
  form.value = {
    name: '',
    email: '',
    phone: '',
    password: '',
    confirmPassword: '',
    age: '',
    website: '',
    agree: false,
  }
  errors.value = {}
  submitSuccess.value = false
}

// 实时验证
watch(
  () => form.value,
  () => {
    if (Object.keys(errors.value).length > 0) {
      validateForm()
    }
  },
  { deep: true },
)
</script>

<style scoped>
.required::after {
  content: ' *';
  color: rgb(239, 68, 68);
}
</style>
