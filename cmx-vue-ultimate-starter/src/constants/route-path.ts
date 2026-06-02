import type { RouteLocationRaw } from 'vue-router'

export const RouterPath: Record<string, RouteLocationRaw> = {
  HOME: '/dashboard',
  LOGIN: '/login',
} as const
