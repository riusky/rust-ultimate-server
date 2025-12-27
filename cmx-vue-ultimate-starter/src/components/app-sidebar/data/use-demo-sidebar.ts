import {
  BadgeHelp,
  BellDot,
  Boxes,
  Bug,
  Code,
  Component,
  CreditCard,
  FileText,
  LayoutDashboard,
  ListTodo,
  Palette,
  PictureInPicture2,
  Podcast,
  Settings,
  SquareUserRound,
  Star,
  Table,
  User,
  Users,
  Wrench,
  BookOpen,
} from 'lucide-vue-next'

import type { NavGroup } from '@/components/app-sidebar/types'

export function useDemoSidebar() {
  const navData = ref<NavGroup[]>([
    {
      title: '通用功能',
      items: [
        {
          title: '任务管理',
          url: '/demo/tasks',
          icon: ListTodo,
        },
        {
          title: '应用中心',
          url: '/demo/apps',
          icon: Boxes,
        },
        {
          title: '用户管理',
          url: '/demo/users',
          icon: Users,
        },
        {
          title: 'AI对话示例',
          url: '/demo/ai-talk',
          icon: Podcast,
        },
        {
          title: '古诗词欣赏',
          url: '/demo/poetry',
          icon: BookOpen,
        },
      ],
    },
    {
      title: '页面示例',
      items: [
        {
          title: '认证页面',
          icon: SquareUserRound,
          items: [
            { title: '登录', url: '/demo/auth/sign-in' },
            { title: '登录(双栏)', url: '/demo/auth/sign-in-2' },
            { title: '注册', url: '/demo/auth/sign-up' },
            { title: '忘记密码', url: '/demo/auth/forgot-password' },
            { title: '验证码', url: '/demo/auth/otp' },
          ],
        },
        {
          title: '错误页面',
          icon: Bug,
          items: [
            { title: '401 | 未授权', url: '/demo/errors/401' },
            { title: '403 | 禁止访问', url: '/demo/errors/403' },
            { title: '404 | 页面未找到', url: '/demo/errors/404' },
            { title: '500 | 服务器错误', url: '/demo/errors/500' },
            { title: '503 | 维护中', url: '/demo/errors/503' },
          ],
        },
        {
          title: '表单页面',
          icon: FileText,
          items: [
            { title: '基础表单', url: '/demo/forms/basic' },
            { title: '高级表单', url: '/demo/forms/advanced' },
            { title: '表单验证', url: '/demo/forms/validation' },
            { title: '文件上传', url: '/demo/forms/upload' },
          ],
        },
        {
          title: '数据展示',
          icon: Table,
          items: [
            { title: '数据表格', url: '/demo/data/tables' },
            { title: '数据列表', url: '/demo/data/data-lists' },
            { title: '卡片展示', url: '/demo/data/data-cards' },
            { title: '图表统计', url: '/demo/data/data-charts' },
          ],
        },
      ],
    },
    {
      title: '系统功能',
      items: [
        {
          title: '系统设置',
          icon: Settings,
          items: [
            { title: '个人资料', url: '/demo/settings/', icon: User },
            { title: '账户设置', url: '/demo/settings/account', icon: Wrench },
            { title: '外观设置', url: '/demo/settings/appearance', icon: Palette },
            { title: '通知设置', url: '/demo/settings/notifications', icon: BellDot },
            { title: '显示设置', url: '/demo/settings/display', icon: PictureInPicture2 },
          ],
        },
        {
          title: '开发工具',
          icon: Wrench,
          items: [
            { title: '组件库', url: '/demo/sva-components', icon: Component },
            { title: '图标库', url: '/demo/dev/icons', icon: Star },
            { title: '代码示例', url: '/demo/dev/examples', icon: Code },
            { title: 'API文档', url: '/demo/dev/api-docs', icon: BookOpen },
          ],
        },
        {
          title: '帮助支持',
          icon: BadgeHelp,
          items: [
            { title: '帮助中心', url: '/demo/help-center' },
            { title: '使用教程', url: '/demo/tutorials' },
            { title: '常见问题', url: '/demo/faq' },
            { title: '联系我们', url: '/demo/contact' },
          ],
        },
      ],
    },
  ])

  const otherPages = ref<NavGroup[]>([
    {
      title: '其他',
      items: [
        {
          title: '套餐与价格',
          icon: CreditCard,
          url: '/demo/billing',
        },
      ],
    },
  ])

  return {
    navData,
    otherPages,
  }
}
