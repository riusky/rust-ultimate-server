import {
  Server,
  Database,
  Shield,
  Users,
  Settings,
  Monitor,
  Network,
  HardDrive,
  Activity,
  Bell,
  FileText,
  Terminal,
  Cloud,
  Lock,
  Cpu,
} from 'lucide-vue-next'

import type { NavGroup } from '@/components/app-sidebar/types'

export function useSystemSidebar() {
  const navData = ref<NavGroup[]>([
        {
      title: '安全管理',
      items: [
        {
          title: '用户权限',
          icon: Users,
          items: [
            { title: '用户管理', url: '/system/users' },
            { title: '角色管理', url: '/system/roles' },
            { title: '权限分配', url: '/system/permissions' },
          ],
        },
        {
          title: '安全策略',
          icon: Shield,
          items: [
            { title: '安全策略', url: '/system/security' },
            { title: '漏洞扫描', url: '/system/vulnerability-scan' },
            { title: '安全审计', url: '/system/security-audit' },
          ],
        },
        {
          title: '访问控制',
          icon: Lock,
          items: [
            { title: '访问控制', url: '/system/access-control' },
            { title: '身份认证', url: '/system/authentication' },
            { title: '会话管理', url: '/system/session' },
          ],
        },
        {
          title: '日志管理',
          icon: FileText,
          items: [
            { title: '审计日志', url: '/system/audit' },
            { title: '操作日志', url: '/system/operation-logs' },
            { title: '安全日志', url: '/system/security-logs' },
          ],
        },
      ],
    },
    {
      title: '系统概览',
      items: [
        {
          title: '监控中心',
          icon: Activity,
          items: [
            { title: '系统监控', url: '/system/monitoring' },
            { title: '性能监控', url: '/system/performance' },
            { title: '日志监控', url: '/system/log-monitoring' },
            { title: '告警监控', url: '/system/alert-monitoring' },
          ],
        },
        {
          title: '资源管理',
          icon: Cpu,
          items: [
            { title: '资源使用', url: '/system/resources' },
            { title: '容量规划', url: '/system/capacity' },
            { title: '负载均衡', url: '/system/load-balancing' },
          ],
        },
      ],
    },
    {
      title: '服务器管理',
      items: [
        {
          title: '服务器管理',
          icon: Server,
          items: [
            { title: '服务器列表', url: '/system/servers' },
            { title: '服务器状态', url: '/system/server-status' },
            { title: '服务器配置', url: '/system/server-config' },
          ],
        },
        {
          title: '数据库管理',
          icon: Database,
          items: [
            { title: '数据库列表', url: '/system/databases' },
            { title: '数据库备份', url: '/system/database-backup' },
            { title: '性能优化', url: '/system/database-optimization' },
          ],
        },
        {
          title: '网络管理',
          icon: Network,
          items: [
            { title: '网络配置', url: '/system/network' },
            { title: '防火墙设置', url: '/system/firewall' },
            { title: 'DNS管理', url: '/system/dns' },
          ],
        },
        {
          title: '存储管理',
          icon: HardDrive,
          items: [
            { title: '存储管理', url: '/system/storage' },
            { title: '备份管理', url: '/system/backup' },
            { title: '磁盘监控', url: '/system/disk-monitoring' },
          ],
        },
      ],
    },
    {
      title: '运维管理',
      items: [
        {
          title: '系统配置',
          icon: Settings,
          items: [
            { title: '系统设置', url: '/system/settings' },
            { title: '参数配置', url: '/system/parameters' },
            { title: '环境配置', url: '/system/environment' },
          ],
        },
        {
          title: '告警管理',
          icon: Bell,
          items: [
            { title: '告警管理', url: '/system/alerts' },
            { title: '通知设置', url: '/system/notifications' },
            { title: '告警规则', url: '/system/alert-rules' },
          ],
        },
        {
          title: '运维工具',
          icon: Terminal,
          items: [
            { title: '命令行工具', url: '/system/terminal' },
            { title: '脚本管理', url: '/system/scripts' },
            { title: '任务调度', url: '/system/scheduling' },
          ],
        },
        {
          title: '云服务',
          icon: Cloud,
          items: [
            { title: '云服务管理', url: '/system/cloud' },
            { title: '容器管理', url: '/system/containers' },
            { title: '服务发现', url: '/system/service-discovery' },
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
          title: '系统文档',
          url: '/system/docs',
          icon: FileText,
        },
        {
          title: '技术支持',
          url: '/system/support',
          icon: Users,
        },
      ],
    },
  ])

  return {
    navData,
    otherPages,
  }
}
