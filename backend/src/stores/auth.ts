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
      console.log('⚠️ 登录请求正在进行中，忽略重复请求')
      return { success: false, message: '登录请求正在进行中' }
    }

    try {
      isLoggingIn.value = true
      console.log('🔐 开始登录请求...')
      const response = await apiLogin(loginForm)
      console.log('✅ 登录API响应成功:', response)
      token.value = response.data.token
      user.value = response.data.user
      localStorage.setItem('token', token.value)
      console.log('💾 Token已保存到localStorage')
      return { success: true }
    } catch (error: any) {
      console.error('❌ 登录失败:', error)
      return { success: false, message: error.response?.data?.message || '登录失败' }
    } finally {
      isLoggingIn.value = false
    }
  }

  const logout = async () => {
    if (isLoggingOut.value) {
      console.log('⚠️ 注销请求正在进行中，忽略重复请求')
      return
    }

    try {
      isLoggingOut.value = true
      console.log('🚪 开始注销请求...')
      await apiLogout()
      console.log('✅ 注销API调用成功')
    } catch (error) {
      console.error('❌ 注销API调用失败:', error)
    } finally {
      console.log('🧹 清理本地认证状态')
      token.value = ''
      user.value = null
      localStorage.removeItem('token')
      isLoggingOut.value = false
    }
  }

  const fetchUserInfo = async (autoLogoutOnError = true) => {
    try {
      console.log('👤 获取用户信息...', { autoLogoutOnError })
      const response = await getUserInfo()
      user.value = response.data
      console.log('✅ 用户信息获取成功:', response.data)
    } catch (error) {
      console.error('❌ 获取用户信息失败:', error)
      if (autoLogoutOnError) {
        console.log('🚪 自动注销 (autoLogoutOnError=true)')
        logout()
      } else {
        console.log('🧹 仅清理本地状态 (autoLogoutOnError=false)')
        // 清除无效的token但不调用logout API
        token.value = ''
        user.value = null
        localStorage.removeItem('token')
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
    fetchUserInfo
  }
})
