<script lang="ts" setup>
import { ref } from 'vue'
import PrivacyPolicyButton from './privacy-policy-button.vue'
import TermsOfServiceButton from './terms-of-service-button.vue'
import ToForgotPasswordLink from './to-forgot-password-link.vue'
import logo from '@/assets/app-icon.png'

defineProps<{
  toggleLayout: () => void
}>()

// 手机号登录表单数据
const phoneFormData = ref({
  phone: '',
  smsCode: '',
})

// 邮箱登录表单数据
const emailFormData = ref({
  email: '',
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

// 邮箱登录
const handleEmailLogin = () => {
  console.log('=== 邮箱登录 ===')
  console.log('邮箱:', emailFormData.value.email)
  console.log('密码:', emailFormData.value.password)
  console.log('表单数据:', emailFormData.value)
  console.log('================')
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
        <UiTabs default-value="phone" class="w-full">
          <UiTabsList class="grid w-full grid-cols-3">
            <UiTabsTrigger value="phone"> 手机号 </UiTabsTrigger>
            <UiTabsTrigger value="email"> 邮箱 </UiTabsTrigger>
            <UiTabsTrigger value="wechat"> 微信扫码 </UiTabsTrigger>
          </UiTabsList>

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

          <!-- 邮箱登录 -->
          <UiTabsContent value="email" class="grid gap-4">
            <form @submit.prevent="handleEmailLogin">
              <div class="grid gap-4">
                <div class="grid gap-2">
                  <UiLabel for="email"> 邮箱 </UiLabel>
                  <UiInput
                    id="email"
                    v-model="emailFormData.email"
                    type="email"
                    placeholder="例如：m@example.com"
                    required
                  />
                </div>
                <div class="grid gap-2">
                  <div class="flex items-center justify-between">
                    <UiLabel for="password"> 密码 </UiLabel>
                    <ToForgotPasswordLink />
                  </div>
                  <UiInput
                    id="password"
                    v-model="emailFormData.password"
                    type="password"
                    required
                    placeholder="*********"
                  />
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
