<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="高级表单" description="这是一个高级表单示例，包含复杂的表单布局和交互">
    <div class="max-w-4xl mx-auto space-y-6">
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>高级表单示例</UiCardTitle>
          <UiCardDescription>
            包含复杂布局、文件上传、日期选择、多步骤表单等高级功能
          </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <form class="space-y-6">
            <!-- 个人信息 -->
            <div class="space-y-4">
              <h3 class="text-lg font-medium">个人信息</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                  <UiLabel for="fullName">姓名</UiLabel>
                  <UiInput id="fullName" placeholder="请输入您的姓名" />
                </div>
                <div class="space-y-2">
                  <UiLabel for="birthDate">出生日期</UiLabel>
                  <UiInput id="birthDate" type="date" />
                </div>
                <div class="space-y-2">
                  <UiLabel for="country">国家/地区</UiLabel>
                  <UiSelect>
                    <UiSelectTrigger>
                      <UiSelectValue placeholder="请选择国家/地区" />
                    </UiSelectTrigger>
                    <UiSelectContent>
                      <UiSelectItem value="china">中国</UiSelectItem>
                      <UiSelectItem value="usa">美国</UiSelectItem>
                      <UiSelectItem value="japan">日本</UiSelectItem>
                      <UiSelectItem value="korea">韩国</UiSelectItem>
                      <UiSelectItem value="other">其他</UiSelectItem>
                    </UiSelectContent>
                  </UiSelect>
                </div>
                <div class="space-y-2">
                  <UiLabel for="city">城市</UiLabel>
                  <UiInput id="city" placeholder="请输入城市" />
                </div>
              </div>
            </div>

            <!-- 工作信息 -->
            <div class="space-y-4">
              <h3 class="text-lg font-medium">工作信息</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                  <UiLabel for="company">公司名称</UiLabel>
                  <UiInput id="company" placeholder="请输入公司名称" />
                </div>
                <div class="space-y-2">
                  <UiLabel for="position">职位</UiLabel>
                  <UiInput id="position" placeholder="请输入职位" />
                </div>
                <div class="space-y-2">
                  <UiLabel for="salary">期望薪资范围</UiLabel>
                  <UiSelect>
                    <UiSelectTrigger>
                      <UiSelectValue placeholder="请选择薪资范围" />
                    </UiSelectTrigger>
                    <UiSelectContent>
                      <UiSelectItem value="0-10k">0-10K</UiSelectItem>
                      <UiSelectItem value="10k-20k">10K-20K</UiSelectItem>
                      <UiSelectItem value="20k-30k">20K-30K</UiSelectItem>
                      <UiSelectItem value="30k+">30K+</UiSelectItem>
                    </UiSelectContent>
                  </UiSelect>
                </div>
                <div class="space-y-2">
                  <UiLabel for="experience">工作经验</UiLabel>
                  <UiSelect>
                    <UiSelectTrigger>
                      <UiSelectValue placeholder="请选择工作经验" />
                    </UiSelectTrigger>
                    <UiSelectContent>
                      <UiSelectItem value="0-1">0-1年</UiSelectItem>
                      <UiSelectItem value="1-3">1-3年</UiSelectItem>
                      <UiSelectItem value="3-5">3-5年</UiSelectItem>
                      <UiSelectItem value="5+">5年以上</UiSelectItem>
                    </UiSelectContent>
                  </UiSelect>
                </div>
              </div>
            </div>

            <!-- 技能标签 -->
            <div class="space-y-4">
              <h3 class="text-lg font-medium">技能标签</h3>
              <div class="space-y-2">
                <UiLabel>选择您的技能</UiLabel>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="javascript" />
                    <UiLabel for="javascript">JavaScript</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="vue" />
                    <UiLabel for="vue">Vue.js</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="react" />
                    <UiLabel for="react">React</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="angular" />
                    <UiLabel for="angular">Angular</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="nodejs" />
                    <UiLabel for="nodejs">Node.js</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="python" />
                    <UiLabel for="python">Python</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="java" />
                    <UiLabel for="java">Java</UiLabel>
                  </div>
                  <div class="flex items-center space-x-2">
                    <UiCheckbox id="go" />
                    <UiLabel for="go">Go</UiLabel>
                  </div>
                </div>
              </div>
            </div>

            <!-- 文件上传 -->
            <div class="space-y-4">
              <h3 class="text-lg font-medium">文件上传</h3>
              <div class="space-y-2">
                <UiLabel for="resume">上传简历</UiLabel>
                <div class="flex items-center gap-4">
                  <UiInput id="resume" type="file" class="flex-1" />
                  <UiButton variant="outline" type="button">选择文件</UiButton>
                </div>
                <p class="text-sm text-muted-foreground">
                  支持 PDF、DOC、DOCX 格式，文件大小不超过 5MB
                </p>
              </div>
              <div class="space-y-2">
                <UiLabel for="portfolio">作品集</UiLabel>
                <div class="flex items-center gap-4">
                  <UiInput id="portfolio" type="file" class="flex-1" multiple />
                  <UiButton variant="outline" type="button">选择文件</UiButton>
                </div>
                <p class="text-sm text-muted-foreground">可上传多个文件，每个文件不超过 10MB</p>
              </div>
            </div>

            <!-- 提交按钮 -->
            <div class="flex gap-2 pt-4">
              <UiButton type="submit" class="flex-1">提交申请</UiButton>
              <UiButton variant="outline" type="button">保存草稿</UiButton>
              <UiButton variant="ghost" type="button">重置表单</UiButton>
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
