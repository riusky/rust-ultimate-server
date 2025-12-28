<script lang="ts" setup>
import { ref } from 'vue'
import { useAuth } from '@/composables/use-auth'
import PrivacyPolicyButton from './privacy-policy-button.vue'
import TermsOfServiceButton from './terms-of-service-button.vue'
import ToForgotPasswordLink from './to-forgot-password-link.vue'
import logo from '@/assets/app-icon.png'

defineProps<{
  toggleLayout: () => void
}>()

const { login, loading, error } = useAuth()

// 手机号登录表单数据
const phoneFormData = ref({
  phone: '',
  smsCode: '',
})

// 账号密码登录表单数据
const accountFormData = ref({
  username: '',
  password: '',
})

// 手机号登录
const handlePhoneLogin = () => {
  console.log('=== 手机号登录 ===')
  console.log('手机号:', phoneFormData.value.phone)
  console.log('验证码:', phoneFormData.value.smsCode)
  console.log('表单数据:', phoneFormData.value)
  console.log('==================')
}

// 账号密码登录
const handleAccountLogin = async () => {
  try {
    await login(accountFormData.value.username, accountFormData.value.password)
  } catch (e) {
    // 错误已在 useAuth 中处理
    console.error('Login failed:', e)
  }
}

// 获取验证码
const handleGetSmsCode = () => {
  console.log('=== 获取验证码 ===')
  console.log('手机号:', phoneFormData.value.phone)
  console.log('==================')
}
</script>

<template>
  <div class="relative">
    <UiCard class="w-full">
      <UiCardHeader>
        <UiCardTitle class="text-2xl"> 登录 </UiCardTitle>
        <UiCardDescription>
          <!-- 选择登录方式。还没有账户？ -->
        </UiCardDescription>
      </UiCardHeader>
      <UiCardContent class="grid gap-4">
        <UiTabs default-value="account" class="w-full">
          <UiTabsList class="grid w-full grid-cols-3">
            <UiTabsTrigger value="account"> 账号 </UiTabsTrigger>
            <UiTabsTrigger value="phone"> 手机号 </UiTabsTrigger>
            <UiTabsTrigger value="wechat"> 微信扫码 </UiTabsTrigger>
          </UiTabsList>

          <!-- 账号密码登录 -->
          <UiTabsContent value="account" class="grid gap-4">
            <form @submit.prevent="handleAccountLogin">
              <div class="grid gap-4">
                <!-- 错误提示 -->
                <div v-if="error" class="text-sm text-destructive bg-destructive/10 p-3 rounded-md">
                  {{ error }}
                </div>
                <div class="grid gap-2">
                  <UiLabel for="username"> 用户名 </UiLabel>
                  <UiInput
                    id="username"
                    v-model="accountFormData.username"
                    type="text"
                    placeholder="请输入用户名"
                    required
                    :disabled="loading"
                  />
                </div>
                <div class="grid gap-2">
                  <div class="flex items-center justify-between">
                    <UiLabel for="password"> 密码 </UiLabel>
                    <ToForgotPasswordLink />
                  </div>
                  <UiInput
                    id="password"
                    v-model="accountFormData.password"
                    type="password"
                    required
                    placeholder="*********"
                    :disabled="loading"
                  />
                </div>
                <UiButton
                  type="submit"
                  class="w-full text-muted bg-muted-foreground hover:text-primary-foreground hover:bg-primary"
                  :disabled="loading"
                >
                  <span v-if="loading" class="mr-2">
                    <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                  </span>
                  {{ loading ? '登录中...' : '登录' }}
                </UiButton>
              </div>
            </form>
          </UiTabsContent>

          <!-- 手机号登录 -->
          <UiTabsContent value="phone" class="grid gap-4">
            <form @submit.prevent="handlePhoneLogin">
              <div class="grid gap-4">
                <div class="grid gap-2">
                  <UiLabel for="phone"> 手机号 </UiLabel>
                  <UiInput
                    id="phone"
                    v-model="phoneFormData.phone"
                    type="tel"
                    placeholder="例如：13800138000"
                    required
                  />
                </div>
                <div class="grid gap-2">
                  <div class="flex items-center justify-between">
                    <UiLabel for="sms-code"> 验证码 </UiLabel>
                    <ToForgotPasswordLink />
                  </div>
                  <div class="flex gap-2">
                    <UiInput
                      id="sms-code"
                      v-model="phoneFormData.smsCode"
                      type="text"
                      required
                      placeholder="请输入验证码"
                    />
                    <UiButton
                      type="button"
                      variant="outline"
                      @click="handleGetSmsCode"
                      class="whitespace-nowrap hover:text-primary-foreground hover:bg-primary hover:ring-2 hover:ring-primary"
                    >
                      获取验证码
                    </UiButton>
                  </div>
                </div>
                <UiButton
                  type="submit"
                  class="w-full text-muted bg-muted-foreground hover:text-primary-foreground hover:bg-primary"
                >
                  登录
                </UiButton>
              </div>
            </form>
          </UiTabsContent>

          <!-- 微信扫码登录 -->
          <UiTabsContent value="wechat" class="grid gap-4">
            <div class="flex flex-col items-center justify-center gap-4 py-8">
              <div class="bg-gray-100 dark:bg-gray-800 p-4 rounded-lg">
                <!-- 这里可以替换为实际的微信二维码组件 -->
                <div class="w-48 h-48 flex items-center justify-center">
                  <img :src="logo" alt="Logo" />
                </div>
              </div>
              <p class="text-sm text-muted-foreground text-center">使用微信扫描二维码登录</p>
            </div>
          </UiTabsContent>
        </UiTabs>

        <UiButton
          @click="toggleLayout"
          class="w-full text-muted bg-muted-foreground hover:text-primary-foreground hover:bg-primary"
        >
          账号注册
        </UiButton>
        <UiCardDescription>
          点击登录即表示您同意我们的
          <TermsOfServiceButton />
          和
          <PrivacyPolicyButton />
        </UiCardDescription>
      </UiCardContent>
    </UiCard>
  </div>
</template>

<style scoped></style>
