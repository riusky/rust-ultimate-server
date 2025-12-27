<route lang="yaml">
meta:
  layout: default-base
</route>

<script setup lang="ts">
import { ref } from 'vue'
import PrivacyPolicyButton from './components/privacy-policy-button.vue'
import TermsOfServiceButton from './components/terms-of-service-button.vue'

defineProps<{
  toggleLayout: () => void
}>()

const emit = defineEmits(['show-login'])

// 表单数据
const formData = ref({
  phone: '',
  password: '',
  confirmPassword: '',
  verifyCode: '',
})

const handleToggle = (toggleLayout?: () => void) => {
  if (toggleLayout) {
    toggleLayout()
  } else {
    emit('show-login')
  }
}

// 提交注册表单
const handleSubmit = () => {
  console.log('=== 注册表单提交 ===')
  console.log('手机号:', formData.value.phone)
  console.log('密码:', formData.value.password)
  console.log('确认密码:', formData.value.confirmPassword)
  console.log('验证码:', formData.value.verifyCode)
  console.log('表单数据:', formData.value)
  console.log('===================')
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <UiCard class="max-w-sm mx-auto">
      <UiCardHeader>
        <UiCardTitle class="text-xl"> 注册 </UiCardTitle>
        <UiCardDescription>
          填写以下信息以创建账户。
          <!-- 已有账户？
          <UiButton variant="link" class="px-0 text-muted-foreground" @click="emit('show-login')">
            登录
          </UiButton> -->
        </UiCardDescription>
      </UiCardHeader>
      <UiCardContent>
        <form @submit.prevent="handleSubmit">
          <div class="grid gap-4">
            <!-- 手机号 - 必填 -->
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

            <!-- 密码 - 必填 -->
            <div class="grid gap-2">
              <UiLabel for="password"> 密码 <span class="text-destructive">*</span> </UiLabel>
              <UiInput
                id="password"
                v-model="formData.password"
                type="password"
                placeholder="请输入密码"
                required
              />
            </div>

            <!-- 确认密码 - 必填 -->
            <div class="grid gap-2">
              <UiLabel for="confirmPassword">
                确认密码 <span class="text-destructive">*</span>
              </UiLabel>
              <UiInput
                id="confirmPassword"
                v-model="formData.confirmPassword"
                type="password"
                placeholder="请再次输入密码"
                required
              />
            </div>

            <!-- 验证码 - 必填 -->
            <div class="grid gap-2">
              <UiLabel for="verifyCode"> 验证码 <span class="text-destructive">*</span> </UiLabel>
              <div class="flex gap-2">
                <UiInput
                  id="verifyCode"
                  v-model="formData.verifyCode"
                  placeholder="请输入验证码"
                  required
                />
                <UiButton type="button" variant="outline" class="whitespace-nowrap">
                  获取验证码
                </UiButton>
              </div>
            </div>

            <UiButton
              type="submit"
              class="w-full text-muted bg-muted-foreground hover:text-primary-foreground hover:bg-primary"
            >
              创建账户
            </UiButton>

            <UiButton
              @click="handleToggle(toggleLayout)"
              class="w-full text-muted bg-muted-foreground hover:text-primary-foreground hover:bg-primary"
            >
              返回登录
            </UiButton>

            <UiSeparator label="或使用以下方式继续" />

            <!-- <div class="flex flex-col items-center justify-between gap-4">
              <GitHubButton />
              <GoogleButton />
            </div> -->

            <UiCardDescription>
              创建账户即表示您同意我们的
              <TermsOfServiceButton />
              和
              <PrivacyPolicyButton />
            </UiCardDescription>
          </div>
        </form>
      </UiCardContent>
    </UiCard>
  </div>
</template>
