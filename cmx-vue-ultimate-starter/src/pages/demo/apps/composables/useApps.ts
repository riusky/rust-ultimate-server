import { ref, computed } from 'vue'
import type { IApp, AppCategory, AppFilter, AppStats } from '../type'
import appsData from '../data/apps'

export function useApps() {
  const apps = ref<IApp[]>(appsData)
  const filter = ref<AppFilter>({})
  const searchQuery = ref('')
  const selectedCategory = ref<AppCategory | 'all'>('all')
  const selectedPricing = ref<string>('all')
  const showFeaturedOnly = ref(false)
  const showConnectedOnly = ref(false)
  const minRating = ref(0)

  // Computed properties
  const filteredApps = computed(() => {
    let filtered = apps.value

    // Search filter
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      filtered = filtered.filter(
        (app) =>
          app.name.toLowerCase().includes(query) ||
          app.desc.toLowerCase().includes(query) ||
          app.tags.some((tag) => tag.toLowerCase().includes(query)) ||
          app.developer.toLowerCase().includes(query),
      )
    }

    // Category filter
    if (selectedCategory.value !== 'all') {
      filtered = filtered.filter((app) => app.category === selectedCategory.value)
    }

    // Pricing filter
    if (selectedPricing.value !== 'all') {
      filtered = filtered.filter((app) => app.pricing?.type === selectedPricing.value)
    }

    // Featured filter
    if (showFeaturedOnly.value) {
      filtered = filtered.filter((app) => app.featured)
    }

    // Connected filter
    if (showConnectedOnly.value) {
      filtered = filtered.filter((app) => app.connected)
    }

    // Rating filter
    if (minRating.value > 0) {
      filtered = filtered.filter((app) => app.rating >= minRating.value)
    }

    return filtered
  })

  const categories = computed(() => {
    const categories = apps.value.reduce((acc, app) => {
      acc.add(app.category)
      return acc
    }, new Set<AppCategory>())
    return Array.from(categories)
  })

  const pricingTypes = computed(() => {
    const types = apps.value.reduce((acc, app) => {
      if (app.pricing?.type) {
        acc.add(app.pricing.type)
      }
      return acc
    }, new Set<string>())
    return Array.from(types)
  })

  const stats = computed<AppStats>(() => {
    const byCategory = {} as Record<AppCategory, number>

    apps.value.forEach((app) => {
      byCategory[app.category] = (byCategory[app.category] || 0) + 1
    })

    return {
      total: apps.value.length,
      connected: apps.value.filter((app) => app.connected).length,
      byCategory,
      featured: apps.value.filter((app) => app.featured).length,
    }
  })

  const connectedApps = computed(() => {
    return apps.value.filter((app) => app.connected)
  })

  const featuredApps = computed(() => {
    return apps.value.filter((app) => app.featured)
  })

  // Actions
  const toggleAppConnection = (appId: string) => {
    const app = apps.value.find((a) => a.id === appId)
    if (app) {
      app.connected = !app.connected
    }
  }

  const getAppById = (appId: string) => {
    return apps.value.find((app) => app.id === appId)
  }

  const getAppsByCategory = (category: AppCategory) => {
    return apps.value.filter((app) => app.category === category)
  }

  const clearFilters = () => {
    searchQuery.value = ''
    selectedCategory.value = 'all'
    selectedPricing.value = 'all'
    showFeaturedOnly.value = false
    minRating.value = 0
  }

  return {
    // State
    apps,
    filter,
    searchQuery,
    selectedCategory,
    selectedPricing,
    showFeaturedOnly,
    showConnectedOnly,
    minRating,

    // Computed
    filteredApps,
    categories,
    pricingTypes,
    stats,
    connectedApps,
    featuredApps,

    // Actions
    toggleAppConnection,
    getAppById,
    getAppsByCategory,
    clearFilters,
  }
}
