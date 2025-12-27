import type { Router } from 'vue-router'

import { storeToRefs } from 'pinia'

import pinia from '@/plugins/pinia'
import { useAuthStore } from '@/stores/auth'

export function authGuard(router: Router) {
  router.beforeEach((to, _from) => {
    const authStore = useAuthStore(pinia)
    const { isLogin } = storeToRefs(authStore)

    // If the page requires login but the user is not logged in, redirect to the login page and record the original target page.
    if (to.meta.auth && !unref(isLogin) && to.name !== '/auth/sign-in-2') {
      return {
        name: '/auth/sign-in-2',
        query: { redirect: to.fullPath },
      }
    }
  })
}
