<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="基础表单" description="这是一个基础表单示例，包含常见的表单元素">
    <div class="max-w-2xl mx-auto space-y-6">
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>基础表单示例</UiCardTitle>
          <UiCardDescription> 包含输入框、选择框、复选框、单选框等基础表单元素 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <form class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="firstName">名字</UiLabel>
                <UiInput id="firstName" placeholder="请输入名字" />
              </div>
              <div class="space-y-2">
                <UiLabel for="lastName">姓氏</UiLabel>
                <UiInput id="lastName" placeholder="请输入姓氏" />
              </div>
            </div>

            <div class="space-y-2">
              <UiLabel for="email">邮箱</UiLabel>
              <UiInput id="email" type="email" placeholder="请输入邮箱地址" />
            </div>

            <div class="space-y-2">
              <UiLabel for="phone">手机号码</UiLabel>
              <UiInput id="phone" type="tel" placeholder="请输入手机号码" />
            </div>

            <div class="space-y-2">
              <UiLabel for="gender">性别</UiLabel>
              <UiSelect>
                <UiSelectTrigger>
                  <UiSelectValue placeholder="请选择性别" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="male">男</UiSelectItem>
                  <UiSelectItem value="female">女</UiSelectItem>
                  <UiSelectItem value="other">其他</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>

            <div class="space-y-2">
              <UiLabel>兴趣爱好</UiLabel>
              <div class="flex flex-wrap gap-4">
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="reading" />
                  <UiLabel for="reading">阅读</UiLabel>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="sports" />
                  <UiLabel for="sports">运动</UiLabel>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="music" />
                  <UiLabel for="music">音乐</UiLabel>
                </div>
                <div class="flex items-center space-x-2">
                  <UiCheckbox id="travel" />
                  <UiLabel for="travel">旅行</UiLabel>
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <UiLabel>通知方式</UiLabel>
              <div class="space-y-2">
                <div class="flex items-center space-x-2">
                  <UiRadioGroup value="email">
                    <div class="flex items-center space-x-2">
                      <UiRadioGroupItem value="email" id="email-notify" />
                      <UiLabel for="email-notify">邮件通知</UiLabel>
                    </div>
                    <div class="flex items-center space-x-2">
                      <UiRadioGroupItem value="sms" id="sms-notify" />
                      <UiLabel for="sms-notify">短信通知</UiLabel>
                    </div>
                    <div class="flex items-center space-x-2">
                      <UiRadioGroupItem value="both" id="both-notify" />
                      <UiLabel for="both-notify">两者都通知</UiLabel>
                    </div>
                  </UiRadioGroup>
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <UiLabel for="message">留言</UiLabel>
              <UiTextarea id="message" placeholder="请输入您的留言..." rows="4" />
            </div>

            <div class="flex items-center space-x-2">
              <UiCheckbox id="agree" />
              <UiLabel for="agree">我同意相关条款和协议</UiLabel>
            </div>

            <div class="flex gap-2 pt-4">
              <UiButton type="submit">提交表单</UiButton>
              <UiButton variant="outline" type="button">取消</UiButton>
            </div>
          </form>
        </UiCardContent>
      </UiCard>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
</script>

<style scoped></style>
