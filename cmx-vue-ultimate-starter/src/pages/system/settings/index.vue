<template>
  <Page title="系统设置" description="配置系统参数和全局设置" sticky>
    <UiTabs default-value="general" class="w-full">
      <UiTabsList class="grid w-full grid-cols-4">
        <UiTabsTrigger value="general">常规设置</UiTabsTrigger>
        <UiTabsTrigger value="security">安全设置</UiTabsTrigger>
        <UiTabsTrigger value="notification">通知设置</UiTabsTrigger>
        <UiTabsTrigger value="advanced">高级设置</UiTabsTrigger>
      </UiTabsList>

      <!-- 常规设置 -->
      <UiTabsContent value="general" class="space-y-6">
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>基本设置</UiCardTitle>
            <UiCardDescription>配置系统基本参数</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="system-name">系统名称</UiLabel>
                <UiInput id="system-name" placeholder="请输入系统名称" value="管理系统" />
              </div>
              <div class="space-y-2">
                <UiLabel for="system-version">系统版本</UiLabel>
                <UiInput id="system-version" placeholder="系统版本" value="v2.1.0" disabled />
              </div>
            </div>
            <div class="space-y-2">
              <UiLabel for="system-description">系统描述</UiLabel>
              <UiTextarea
                id="system-description"
                placeholder="请输入系统描述"
                value="这是一个企业级管理系统，用于管理用户、权限和系统配置。"
              />
            </div>
            <div class="space-y-2">
              <UiLabel for="timezone">时区设置</UiLabel>
              <UiSelect default-value="beijing">
                <UiSelectTrigger>
                  <UiSelectValue placeholder="选择时区" />
                </UiSelectTrigger>
                <UiSelectContent>
                  <UiSelectItem value="beijing">北京时间 (UTC+8)</UiSelectItem>
                  <UiSelectItem value="tokyo">东京时间 (UTC+9)</UiSelectItem>
                  <UiSelectItem value="new-york">纽约时间 (UTC-5)</UiSelectItem>
                  <UiSelectItem value="london">伦敦时间 (UTC+0)</UiSelectItem>
                </UiSelectContent>
              </UiSelect>
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard>
          <UiCardHeader>
            <UiCardTitle>界面设置</UiCardTitle>
            <UiCardDescription>配置系统界面显示</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="theme">主题设置</UiLabel>
                <UiSelect default-value="light">
                  <UiSelectTrigger>
                    <UiSelectValue placeholder="选择主题" />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="light">浅色主题</UiSelectItem>
                    <UiSelectItem value="dark">深色主题</UiSelectItem>
                    <UiSelectItem value="auto">跟随系统</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
              </div>
              <div class="space-y-2">
                <UiLabel for="language">语言设置</UiLabel>
                <UiSelect default-value="zh-cn">
                  <UiSelectTrigger>
                    <UiSelectValue placeholder="选择语言" />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="zh-cn">简体中文</UiSelectItem>
                    <UiSelectItem value="en-us">English</UiSelectItem>
                    <UiSelectItem value="ja-jp">日本語</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="sidebar-collapse" checked />
                <UiLabel for="sidebar-collapse">默认折叠侧边栏</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="breadcrumb" checked />
                <UiLabel for="breadcrumb">显示面包屑导航</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="animation" />
                <UiLabel for="animation">启用页面动画</UiLabel>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </UiTabsContent>

      <!-- 安全设置 -->
      <UiTabsContent value="security" class="space-y-6">
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>密码策略</UiCardTitle>
            <UiCardDescription>配置用户密码安全策略</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="min-length">最小密码长度</UiLabel>
                <UiInput id="min-length" type="number" value="8" />
              </div>
              <div class="space-y-2">
                <UiLabel for="expire-days">密码过期天数</UiLabel>
                <UiInput id="expire-days" type="number" value="90" />
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="complexity" checked />
                <UiLabel for="complexity">要求密码复杂度</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="history" checked />
                <UiLabel for="history">禁止使用历史密码</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="lockout" checked />
                <UiLabel for="lockout">启用账户锁定</UiLabel>
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard>
          <UiCardHeader>
            <UiCardTitle>会话管理</UiCardTitle>
            <UiCardDescription>配置用户会话安全设置</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="session-timeout">会话超时时间(分钟)</UiLabel>
                <UiInput id="session-timeout" type="number" value="30" />
              </div>
              <div class="space-y-2">
                <UiLabel for="max-sessions">最大并发会话数</UiLabel>
                <UiInput id="max-sessions" type="number" value="3" />
              </div>
            </div>
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="single-sign-on" />
                <UiLabel for="single-sign-on">启用单点登录(SSO)</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="force-logout" checked />
                <UiLabel for="force-logout">强制登出过期会话</UiLabel>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </UiTabsContent>

      <!-- 通知设置 -->
      <UiTabsContent value="notification" class="space-y-6">
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>邮件通知</UiCardTitle>
            <UiCardDescription>配置系统邮件通知设置</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="space-y-2">
              <UiLabel for="smtp-host">SMTP服务器</UiLabel>
              <UiInput id="smtp-host" placeholder="smtp.example.com" value="smtp.company.com" />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="smtp-port">SMTP端口</UiLabel>
                <UiInput id="smtp-port" type="number" value="587" />
              </div>
              <div class="space-y-2">
                <UiLabel for="smtp-ssl">SSL/TLS</UiLabel>
                <UiSelect default-value="tls">
                  <UiSelectTrigger>
                    <UiSelectValue placeholder="选择加密方式" />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="none">无加密</UiSelectItem>
                    <UiSelectItem value="ssl">SSL</UiSelectItem>
                    <UiSelectItem value="tls">TLS</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
              </div>
            </div>
            <div class="space-y-2">
              <UiLabel for="from-email">发件人邮箱</UiLabel>
              <UiInput
                id="from-email"
                placeholder="noreply@example.com"
                value="noreply@company.com"
              />
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard>
          <UiCardHeader>
            <UiCardTitle>通知类型</UiCardTitle>
            <UiCardDescription>配置需要接收的通知类型</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <UiCheckbox id="system-alerts" checked />
                <UiLabel for="system-alerts">系统告警通知</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="user-activities" checked />
                <UiLabel for="user-activities">用户活动通知</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="security-events" checked />
                <UiLabel for="security-events">安全事件通知</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="backup-reports" />
                <UiLabel for="backup-reports">备份报告通知</UiLabel>
              </div>
              <div class="flex items-center space-x-2">
                <UiCheckbox id="performance-reports" />
                <UiLabel for="performance-reports">性能报告通知</UiLabel>
              </div>
            </div>
          </UiCardContent>
        </UiCard>
      </UiTabsContent>

      <!-- 高级设置 -->
      <UiTabsContent value="advanced" class="space-y-6">
        <UiCard>
          <UiCardHeader>
            <UiCardTitle>数据库设置</UiCardTitle>
            <UiCardDescription>配置数据库连接和性能参数</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="db-host">数据库主机</UiLabel>
                <UiInput id="db-host" value="localhost" />
              </div>
              <div class="space-y-2">
                <UiLabel for="db-port">数据库端口</UiLabel>
                <UiInput id="db-port" type="number" value="3306" />
              </div>
              <div class="space-y-2">
                <UiLabel for="db-name">数据库名称</UiLabel>
                <UiInput id="db-name" value="management_system" />
              </div>
              <div class="space-y-2">
                <UiLabel for="db-pool">连接池大小</UiLabel>
                <UiInput id="db-pool" type="number" value="20" />
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <UiCard>
          <UiCardHeader>
            <UiCardTitle>缓存设置</UiCardTitle>
            <UiCardDescription>配置系统缓存策略</UiCardDescription>
          </UiCardHeader>
          <UiCardContent class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <UiLabel for="cache-type">缓存类型</UiLabel>
                <UiSelect default-value="redis">
                  <UiSelectTrigger>
                    <UiSelectValue placeholder="选择缓存类型" />
                  </UiSelectTrigger>
                  <UiSelectContent>
                    <UiSelectItem value="memory">内存缓存</UiSelectItem>
                    <UiSelectItem value="redis">Redis</UiSelectItem>
                    <UiSelectItem value="memcached">Memcached</UiSelectItem>
                  </UiSelectContent>
                </UiSelect>
              </div>
              <div class="space-y-2">
                <UiLabel for="cache-ttl">缓存过期时间(秒)</UiLabel>
                <UiInput id="cache-ttl" type="number" value="3600" />
              </div>
            </div>
          </UiCardContent>
        </UiCard>

        <div class="flex justify-end space-x-2">
          <UiButton variant="outline">重置设置</UiButton>
          <UiButton>保存设置</UiButton>
        </div>
      </UiTabsContent>
    </UiTabs>
  </Page>
</template>

<script lang="ts" setup>
import Page from '@/components/global-layout/basic-page.vue'
</script>

<style scoped></style>
