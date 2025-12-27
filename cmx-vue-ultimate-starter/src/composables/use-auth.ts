import { storeToRefs } from 'pinia'

import { useAuthStore } from '@/stores/auth'
import { login as apiLogin, logoff as apiLogoff } from '@/services/api/auth'

export function useAuth() {
  const router = useRouter()

  const authStore = useAuthStore()
  const { isLogin, username } = storeToRefs(authStore)
  const loading = ref(false)
  const error = ref('')

  async function logout() {
    try {
      await apiLogoff()
    } catch (e) {
      console.error('Logoff error:', e)
    } finally {
      authStore.clearLogin()
      router.push({ path: '/auth/sign-in-2' })
    }
  }

  function toHome() {
    router.push({ path: '/' })
  }

  async function login(loginUsername: string, password: string) {
    loading.value = true
    error.value = ''

    try {
      const response = await apiLogin({ username: loginUsername, pwd: password })

      if (response.result.success) {
        authStore.setLogin(loginUsername)

        const redirect = router.currentRoute.value.query.redirect as string
        if (!redirect || redirect.startsWith('//')) {
          toHome()
        } else {
          router.push(redirect)
        }
      }
    } catch (e: unknown) {
      const axiosError = e as { response?: { status?: number } }
      if (axiosError.response?.status === 401) {
        error.value = '用户名或密码错误'
      } else {
        error.value = '登录失败，请稍后重试'
      }
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    isLogin,
    username,
    logout,
    login,
  }
}
