import { tauriInvoke } from './request'
import type { LoginForm, RegisterForm } from '@/types/auth'

// 登录（不需要 token）
export const login = (data: LoginForm) => {
  return tauriInvoke('login', {
    username: data.username,
    password: data.password,
  })
}

// 注册（不需要 token）
export const register = (data: RegisterForm) => {
  return tauriInvoke('register', {
    username: data.username,
    password: data.password,
    email: data.email || null,
  })
}

// 登出
export const logout = () => {
  return tauriInvoke('logout', {})
}

// 获取用户信息
export const getUserInfo = () => {
  return tauriInvoke('get_current_user', {})
}

// 获取用户列表（管理员）
export const getUserList = (params: any) => {
  return tauriInvoke('get_user_list', {
    current: params.current,
    size: params.size,
    username: params.username || null,
    role: params.role || null,
  })
}

// 获取用户详情（管理员）
export const getUserDetail = (id: number) => {
  return tauriInvoke('get_user_detail', { id })
}

// 切换用户状态（管理员）
export const toggleUserStatus = (id: number) => {
  return tauriInvoke('toggle_user_status', { id })
}
