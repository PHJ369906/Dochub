import axios from 'axios'
import { ElMessage } from 'element-plus'
import { useAuthStore } from '@/stores/auth'

const request = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json'
  }
})

// 请求拦截器
request.interceptors.request.use(
  (config) => {
    const authStore = useAuthStore()
    if (authStore.token) {
      config.headers.Authorization = `Bearer ${authStore.token}`
    }
    // 确保每次请求都有正确的Accept头
    config.headers.Accept = 'application/json'
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
request.interceptors.response.use(
  (response) => {
    // 如果响应是字符串，先解析为JSON
    let responseData = response.data
    if (typeof responseData === 'string') {
      try {
        responseData = JSON.parse(responseData)
      } catch (error) {
        ElMessage.error('响应数据格式错误')
        return Promise.reject(new Error('响应数据格式错误'))
      }
    }
    
    const { code, message, data } = responseData
    if (code === 200) {
      // 返回完整的响应格式，保持与组件中的使用一致
      return { code, data, message }
    } else {
      ElMessage.error(message || '请求失败')
      return Promise.reject(new Error(message || '请求失败'))
    }
  },
  (error) => {
    if (error.response?.status === 401) {
      const authStore = useAuthStore()
      // 只有在用户已经登录的情况下才显示过期消息并调用logout
      if (authStore.isAuthenticated) {
        authStore.logout()
        ElMessage.error('登录已过期，请重新登录')
      } else {
        // 如果用户本来就没有登录，只清除本地token
        authStore.token = ''
        authStore.user = null
        localStorage.removeItem('token')
      }
    } else {
      ElMessage.error(error.response?.data?.message || '网络错误')
    }
    return Promise.reject(error)
  }
)

export default request
