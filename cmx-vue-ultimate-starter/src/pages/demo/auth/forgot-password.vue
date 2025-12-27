<route lang="yaml">
meta:
  layout: default-base
</route>

<script setup lang="ts">
import { ref } from 'vue'
// 表单数据
const formData = ref({
  phone: '',
  verifyCode: '',
  newPassword: '',
  confirmPassword: '',
})

// 验证状态
const isVerified = ref(false)
const isCodeSent = ref(false)
const countdown = ref(0)

// Demo数据：模拟验证码
const DEMO_VERIFY_CODE = '123456'
const DEMO_PHONE = '13800138000'

// 发送验证码
const handleSendCode = () => {
  if (!formData.value.phone) {
    console.log('请输入手机号')
    return
  }

  console.log('=== 发送验证码 ===')
  console.log('手机号:', formData.value.phone)
  console.log('验证码已发送（Demo）:', DEMO_VERIFY_CODE)
  console.log('===================')

  isCodeSent.value = true
  countdown.value = 60

  // 倒计时
  const timer = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(timer)
      isCodeSent.value = false
    }
  }, 1000)
}

// 验证手机号和验证码
const handleVerify = () => {
  console.log('=== 验证手机号和验证码 ===')
  console.log('输入的手机号:', formData.value.phone)
  console.log('输入的验证码:', formData.value.verifyCode)

  // Demo验证逻辑
  if (formData.value.phone === DEMO_PHONE && formData.value.verifyCode === DEMO_VERIFY_CODE) {
    console.log('✅ 验证成功')
    isVerified.value = true
  } else {
    console.log('❌ 验证失败')
    console.log('提示：Demo手机号为', DEMO_PHONE, '验证码为', DEMO_VERIFY_CODE)
  }
  console.log('===================')
}

// 重置密码
const handleResetPassword = () => {
  console.log('=== 重置密码 ===')
  console.log('手机号:', formData.value.phone)
  console.log('新密码:', formData.value.newPassword)
  console.log('确认密码:', formData.value.confirmPassword)

  if (formData.value.newPassword !== formData.value.confirmPassword) {
    console.log('❌ 两次密码输入不一致')
    return
  }

  console.log('✅ 密码重置成功')
  console.log('===================')
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4 min-w-screen">
    <main class="flex flex-col gap-4">
      <UiCard class="max-w-md">
        <UiCardHeader>
          <UiCardTitle class="text-2xl"> 忘记密码 </UiCardTitle>
          <UiCardDescription>
            {{ isVerified ? '请设置新密码' : '请输入注册时的手机号，我们将发送验证码进行身份验证' }}
          </UiCardDescription>
        </UiCardHeader>
        <UiCardContent class="grid gap-4">
          <!-- 手机号验证阶段 -->
          <template v-if="!isVerified">
            <div class="grid gap-2">
              <UiLabel for="phone"> 手机号 <span class="text-destructive">*</span> </UiLabel>
              <UiInput
                id="phone"
                v-model="formData.phone"
                type="tel"
                placeholder="请输入手机号"
                required
              />
            </div>

            <div class="grid gap-2">
              <UiLabel for="verifyCode"> 验证码 <span class="text-destructive">*</span> </UiLabel>
              <div class="flex gap-2">
                <UiInput
                  id="verifyCode"
                  v-model="formData.verifyCode"
                  placeholder="请输入验证码"
                  required
                />
                <UiButton
                  type="button"
                  variant="outline"
                  class="whitespace-nowrap"
                  :disabled="isCodeSent"
                  @click="handleSendCode"
                >
                  {{ isCodeSent ? `${countdown}秒后重试` : '获取验证码' }}
                </UiButton>
              </div>
              <p class="text-xs text-muted-foreground">
                Demo提示：手机号 {{ DEMO_PHONE }}，验证码 {{ DEMO_VERIFY_CODE }}
              </p>
            </div>
          </template>

          <!-- 重置密码阶段 -->
          <template v-else>
            <div class="grid gap-2">
              <UiLabel for="newPassword"> 新密码 <span class="text-destructive">*</span> </UiLabel>
              <UiInput
                id="newPassword"
                v-model="formData.newPassword"
                type="password"
                placeholder="请输入新密码"
                required
              />
            </div>

            <div class="grid gap-2">
              <UiLabel for="confirmPassword">
                确认新密码 <span class="text-destructive">*</span>
              </UiLabel>
              <UiInput
                id="confirmPassword"
                v-model="formData.confirmPassword"
                type="password"
                placeholder="请再次输入新密码"
                required
              />
            </div>
          </template>
        </UiCardContent>
        <UiCardFooter class="flex flex-col gap-2">
          <UiButton v-if="!isVerified" class="w-full" @click="handleVerify"> 验证身份 </UiButton>
          <UiButton v-else class="w-full" @click="handleResetPassword"> 重置密码 </UiButton>
        </UiCardFooter>
      </UiCard>
    </main>
  </div>
</template>
