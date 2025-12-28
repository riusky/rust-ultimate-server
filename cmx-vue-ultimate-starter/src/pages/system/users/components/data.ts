import { Circle, CircleCheck, CircleOff, CircleX, Shield, User, UserCheck, UserCog, UserX } from 'lucide-vue-next'
import { h } from 'vue'

export const genders = [
  {
    value: 'Unknown',
    label: '未知',
    icon: h(User),
  },
  {
    value: 'Male',
    label: '男',
    icon: h(UserCheck),
  },
  {
    value: 'Female',
    label: '女',
    icon: h(UserX),
  },
]

export const statuses = [
  {
    value: 'Active',
    label: '正常',
    icon: h(CircleCheck),
  },
  {
    value: 'Inactive',
    label: '未激活',
    icon: h(Circle),
  },
  {
    value: 'Suspended',
    label: '已停用',
    icon: h(CircleOff),
  },
  {
    value: 'Deleted',
    label: '已删除',
    icon: h(CircleX),
  },
]

export const userTypes = [
  {
    value: 'Sys',
    label: '系统用户',
    icon: h(Shield),
  },
  {
    value: 'User',
    label: '普通用户',
    icon: h(User),
  },
  {
    value: 'Admin',
    label: '管理员',
    icon: h(UserCog),
  },
]
