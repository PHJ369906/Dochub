import request from './request'
import type { LoginForm, RegisterForm } from '@/types/auth'

// 登录
export const login = (data: LoginForm) => {
  return request.post('/auth/login', data)
}

// 注册
export const register = (data: RegisterForm) => {
  return request.post('/auth/register', data)
}

// 登出
export const logout = () => {
  return request.post('/auth/logout')
}

// 获取用户信息
export const getUserInfo = () => {
  return request.get('/auth/userinfo')
}

// 获取用户列表（管理员）
export const getUserList = (params: any) => {
  return request.get('/auth/users', { params })
}

// 获取用户详情（管理员）
export const getUserDetail = (id: number) => {
  return request.get(`/auth/users/${id}`)
}

// 切换用户状态（管理员）
export const toggleUserStatus = (id: number) => {
  return request.put(`/auth/users/${id}/toggle-status`)
}
