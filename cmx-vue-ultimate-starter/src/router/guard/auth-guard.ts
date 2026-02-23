import type { Router } from 'vue-router'

import { storeToRefs } from 'pinia'

import { getCurrentUser } from '@/services/api/rest/user/auth'
import pinia from '@/plugins/pinia'
import { useAuthStore } from '@/stores/auth'

/** Only validate session once per page load when user claims to be logged in. */
let sessionValidatedThisLoad = false

export function authGuard(router: Router) {
  router.beforeEach(async (to, _from) => {
    const authStore = useAuthStore(pinia)
    const { isLogin } = storeToRefs(authStore)

    // Public routes or already on login page: allow
    if (!to.meta.auth || to.name === '/auth/sign-in-2') {
      return true
    }

    // Requires auth but not logged in (per store): redirect to login
    if (!unref(isLogin)) {
      return {
        name: '/auth/sign-in-2',
        query: { redirect: to.fullPath },
      }
    }

    // Requires auth and store says logged in: validate session with backend (once per load)
    if (sessionValidatedThisLoad) {
      return true
    }

    try {
      await getCurrentUser()
      sessionValidatedThisLoad = true
      return true
    } catch {
      authStore.clearLogin()
      return {
        name: '/auth/sign-in-2',
        query: { redirect: to.fullPath },
      }
    }
  })
}
