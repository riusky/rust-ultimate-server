import { defineStore } from 'pinia'

export const useAuthStore = defineStore(
  'auth',
  () => {
    const isLogin = ref(false)
    const username = ref('')

    function setLogin(name: string) {
      isLogin.value = true
      username.value = name
    }

    function clearLogin() {
      isLogin.value = false
      username.value = ''
    }

    return {
      isLogin,
      username,
      setLogin,
      clearLogin,
    }
  },
  {
    persist: {
      pick: ['isLogin', 'username'],
    },
  },
)
