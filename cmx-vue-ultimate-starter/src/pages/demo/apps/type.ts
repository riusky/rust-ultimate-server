import type { Component } from 'vue'

export interface IApp {
  id: string
  name: string
  logo: Component
  desc: string
  category: AppCategory
  status: AppStatus
  connected: boolean
  featured: boolean
  rating: number
  reviews: number
  lastUpdated: string
  version: string
  developer: string
  website?: string
  permissions: string[]
  tags: string[]
  screenshots?: string[]
  pricing?: AppPricing
  requirements?: AppRequirements
}

export type AppCategory =
  | 'communication'
  | 'productivity'
  | 'development'
  | 'design'
  | 'finance'
  | 'entertainment'
  | 'utilities'
  | 'security'

export type AppStatus = 'active' | 'inactive' | 'maintenance' | 'beta'

export interface AppPricing {
  type: 'free' | 'freemium' | 'paid' | 'subscription'
  price?: number
  currency?: string
  period?: 'monthly' | 'yearly' | 'lifetime'
}

export interface AppRequirements {
  minVersion?: string
  dependencies?: string[]
  storage?: string
  memory?: string
}

export interface AppFilter {
  category?: AppCategory
  status?: AppStatus
  pricing?: AppPricing['type']
  featured?: boolean
  rating?: number
  search?: string
}

export interface AppStats {
  total: number
  connected: number
  byCategory: Record<AppCategory, number>
  featured: number
}
