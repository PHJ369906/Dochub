import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { login as apiLogin, logout as apiLogout, getUserInfo } from '@/api/auth'
import type { LoginForm, User } from '@/types/auth'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string>(localStorage.getItem('token') || '')
  const user = ref<User | null>(null)
  const isLoggingIn = ref(false)
  const isLoggingOut = ref(false)

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.role === 'admin')

  const login = async (loginForm: LoginForm) => {
    if (isLoggingIn.value) {
      return { success: false, message: '登录请求正在进行中' }
    }

    try {
      isLoggingIn.value = true
      const response = await apiLogin(loginForm)
      token.value = response.data.token
      user.value = response.data.user
      localStorage.setItem('token', token.value)
      return { success: true }
    } catch (error: any) {
      return { success: false, message: error.message || '登录失败' }
    } finally {
      isLoggingIn.value = false
    }
  }

  const logout = async () => {
    if (isLoggingOut.value) {
      return
    }

    try {
      isLoggingOut.value = true
      await apiLogout()
    } catch (error) {
      // 忽略注销 API 错误
    } finally {
      clearAuth()
      isLoggingOut.value = false
    }
  }

  const clearAuth = () => {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
  }

  const fetchUserInfo = async (autoLogoutOnError = true) => {
    try {
      const response = await getUserInfo()
      user.value = response.data
    } catch (error) {
      if (autoLogoutOnError) {
        logout()
      } else {
        clearAuth()
      }
    }
  }

  // 初始化时如果有token，获取用户信息（但不自动注销）
  if (token.value) {
    fetchUserInfo(false)
  }

  return {
    token,
    user,
    isAuthenticated,
    isAdmin,
    login,
    logout,
    clearAuth,
    fetchUserInfo
  }
})
