<script lang="ts" setup>
import { ref } from 'vue'

const isOpen = ref(false)
</script>

<template>
  <UiDialog v-model:open="isOpen">
    <UiDialogTrigger as-child>
      <UiButton
        variant="link"
        class="px-0 font-normal underline text-muted-foreground hover:text-primary"
      >
        隐私政策
      </UiButton>
    </UiDialogTrigger>
    <UiDialogContent class="max-w-2xl max-h-[80vh]">
      <UiDialogHeader>
        <UiDialogTitle class="text-2xl font-bold">隐私政策</UiDialogTitle>
        <UiDialogDescription> 我们重视您的隐私，请仔细阅读我们的隐私政策 </UiDialogDescription>
      </UiDialogHeader>

      <UiScrollArea class="h-[60vh] pr-4">
        <div class="space-y-4 text-sm">
          <section>
            <h3 class="font-semibold text-base mb-2">引言</h3>
            <p class="text-muted-foreground leading-relaxed">
              欢迎您使用Riusky软件的产品和服务。我们深知个人信息对您的重要性，因此制定本隐私政策来说明我们如何收集、使用、存储和保护您的个人信息。请您仔细阅读并充分理解本隐私政策。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">1. 我们收集的信息</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              <strong>1.1 您主动提供的信息</strong>
            </p>
            <ul class="list-disc list-inside space-y-1 text-muted-foreground ml-4 mb-3">
              <li>注册信息：昵称、手机号、邮箱地址、密码等</li>
              <li>身份信息：头像、性别、生日等个人资料</li>
              <li>联系信息：地址、电话号码等</li>
              <li>支付信息：银行卡号、支付账号等</li>
            </ul>
            <p class="text-muted-foreground leading-relaxed mb-2">
              <strong>1.2 我们自动收集的信息</strong>
            </p>
            <ul class="list-disc list-inside space-y-1 text-muted-foreground ml-4">
              <li>设备信息：设备型号、操作系统、唯一设备标识符等</li>
              <li>日志信息：IP地址、浏览器类型、访问时间、访问页面等</li>
              <li>位置信息：基于GPS、IP地址的位置信息（需经您授权）</li>
              <li>Cookie和类似技术收集的信息</li>
            </ul>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">2. 我们如何使用您的信息</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              我们会将收集的信息用于以下目的：
            </p>
            <ul class="list-disc list-inside space-y-1 text-muted-foreground ml-4">
              <li>提供、维护和改进我们的产品和服务</li>
              <li>验证您的身份，防止欺诈和未经授权的访问</li>
              <li>处理您的订单和支付</li>
              <li>与您沟通，包括发送服务通知、更新和安全警告</li>
              <li>个性化推荐内容和广告（需经您同意）</li>
              <li>进行数据分析和研究，以了解用户行为和偏好</li>
              <li>遵守法律法规和监管要求</li>
            </ul>
          </section>

          <div class="mt-6 pt-4 border-t text-xs text-muted-foreground">
            <p>最后更新时间：2025年10月21日</p>
            <p class="mt-1">版本号：v1.0</p>
          </div>
        </div>
      </UiScrollArea>

      <UiDialogFooter>
        <UiButton @click="isOpen = false" class="w-full sm:w-auto"> 我已阅读并同意 </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
