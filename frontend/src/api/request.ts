import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { useAuthStore } from '@/stores/auth'

/**
 * 统一的 Tauri invoke 封装，替代原来的 axios request
 * 自动注入 token，处理错误
 */
export async function tauriInvoke<T = any>(command: string, args: Record<string, any> = {}): Promise<{ code: number; data: T; message: string }> {
  const authStore = useAuthStore()

  // 自动注入 token
  if (authStore.token && !args.token) {
    args.token = authStore.token
  }

  try {
    const response = await invoke<{ code: number; data: T; message: string }>(command, args)
    return response
  } catch (error: any) {
    const errorMsg = typeof error === 'string' ? error : error.message || '请求失败'

    // 未授权处理
    if (errorMsg.includes('未授权') || errorMsg.includes('无效的登录凭证')) {
      if (authStore.isAuthenticated) {
        authStore.clearAuth()
        ElMessage.error('登录已过期，请重新登录')
      }
    } else {
      ElMessage.error(errorMsg)
    }

    return Promise.reject(new Error(errorMsg))
  }
}

export default tauriInvoke
