<template>
  <div class="login-container fade-in">
    <div class="login-content">
      <!-- 左侧品牌区域 -->
      <div class="brand-section">
        <div class="brand-content">
          <div class="brand-logo">
            <div class="logo-icon">
              <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h1 class="brand-title">文档管理系统</h1>
          </div>
          <p class="brand-subtitle">安全高效的文档管理平台</p>
          <div class="feature-list">
            <div class="feature-item">
              <div class="feature-icon">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <span>安全可靠的权限管理</span>
            </div>
            <div class="feature-item">
              <div class="feature-icon">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z" />
                </svg>
              </div>
              <span>多用户协作平台</span>
            </div>
            <div class="feature-item">
              <div class="feature-icon">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path d="M4 3a2 2 0 100 4h12a2 2 0 100-4H4z" />
                  <path fill-rule="evenodd" d="M3 8h14v7a2 2 0 01-2 2H5a2 2 0 01-2-2V8zm5 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" clip-rule="evenodd" />
                </svg>
              </div>
              <span>智能文档管理</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧登录表单 -->
      <div class="form-section">
        <div class="login-form">
          <div class="form-header">
            <h2>欢迎回来</h2>
            <p>请登录您的账户以继续</p>
          </div>
          
          <el-form
            ref="loginFormRef"
            :model="loginForm"
            :rules="loginRules"
            label-width="0"
            size="large"
            class="login-form-content"
          >
            <el-form-item prop="username">
              <div class="input-wrapper">
                <el-input
                  v-model="loginForm.username"
                  placeholder="请输入用户名"
                  prefix-icon="User"
                  clearable
                  class="form-input"
                />
              </div>
            </el-form-item>
            
            <el-form-item prop="password">
              <div class="input-wrapper">
                <el-input
                  v-model="loginForm.password"
                  type="password"
                  placeholder="请输入密码"
                  prefix-icon="Lock"
                  show-password
                  clearable
                  class="form-input"
                  @keyup.enter="handleLogin"
                />
              </div>
            </el-form-item>
            
            <el-form-item>
              <el-button
                type="primary"
                size="large"
                class="login-button"
                :loading="loading"
                @click="handleLogin"
              >
                {{ loading ? '登录中...' : '立即登录' }}
              </el-button>
            </el-form-item>
          </el-form>

          <div class="form-footer">
            <div class="demo-accounts">
              <p class="demo-title">演示账户</p>
              <div class="demo-list">
                <button class="demo-account" @click="fillAccount('admin', 'admin123')">
                  <span class="demo-role">管理员</span>
                  <span class="demo-username">admin</span>
                </button>
                <button class="demo-account" @click="fillAccount('user', 'user123')">
                  <span class="demo-role">普通用户</span>
                  <span class="demo-username">user</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { useAuthStore } from '@/stores/auth'
import type { LoginForm } from '@/types/auth'

const router = useRouter()
const authStore = useAuthStore()

const loginFormRef = ref<FormInstance>()
const loading = ref(false)

const loginForm = reactive<LoginForm>({
  username: '',
  password: ''
})

const loginRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 50, message: '用户名长度为3-50个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ]
}

const handleLogin = async () => {
  if (!loginFormRef.value) return
  
  await loginFormRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const result = await authStore.login(loginForm)
        if (result.success) {
          ElMessage.success('登录成功')
          router.push('/home')
        } else {
          ElMessage.error(result.message || '登录失败')
        }
      } catch (error) {
        ElMessage.error('登录失败，请重试')
      } finally {
        loading.value = false
      }
    }
  })
}

const fillAccount = (username: string, password: string) => {
  loginForm.username = username
  loginForm.password = password
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  padding: 2rem;
}

.login-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  max-width: 1200px;
  width: 100%;
  background: var(--bg-primary);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  overflow: hidden;
  min-height: 600px;
  border: 1px solid var(--border-light);
}

/* 左侧品牌区域 */
.brand-section {
  background: linear-gradient(135deg, var(--accent-600) 0%, var(--accent-800) 100%);
  padding: 3rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  color: white;
  position: relative;
  overflow: hidden;
}

.brand-section::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grain" width="100" height="100" patternUnits="userSpaceOnUse"><circle cx="25" cy="25" r="1" fill="rgba(255,255,255,0.05)"/><circle cx="75" cy="75" r="1" fill="rgba(255,255,255,0.05)"/><circle cx="50" cy="10" r="0.5" fill="rgba(255,255,255,0.03)"/><circle cx="20" cy="80" r="0.5" fill="rgba(255,255,255,0.03)"/></pattern></defs><rect width="100" height="100" fill="url(%23grain)"/></svg>');
  z-index: 0;
}

.brand-content {
  position: relative;
  z-index: 1;
}

.brand-logo {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.logo-icon {
  width: 3rem;
  height: 3rem;
  background: rgba(255, 255, 255, 0.15);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.logo-icon svg {
  width: 1.5rem;
  height: 1.5rem;
  color: white;
}

.brand-title {
  font-size: 1.875rem;
  font-weight: 700;
  margin: 0;
  letter-spacing: -0.025em;
}

.brand-subtitle {
  font-size: 1.125rem;
  opacity: 0.9;
  margin-bottom: 3rem;
  line-height: 1.6;
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.feature-icon {
  width: 2rem;
  height: 2rem;
  background: rgba(255, 255, 255, 0.15);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.feature-icon svg {
  width: 1rem;
  height: 1rem;
  color: white;
}

/* 右侧表单区域 */
.form-section {
  padding: 3rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
}

.login-form {
  width: 100%;
  max-width: 400px;
}

.form-header {
  text-align: center;
  margin-bottom: 2rem;
}

.form-header h2 {
  font-size: 1.875rem;
  font-weight: 700;
  color: var(--neutral-900);
  margin: 0 0 0.5rem 0;
  letter-spacing: -0.025em;
}

.form-header p {
  color: var(--neutral-600);
  margin: 0;
  font-size: 1rem;
}

.login-form-content {
  margin-bottom: 2rem;
}

.input-wrapper {
  margin-bottom: 1rem;
}

.form-input :deep(.el-input__wrapper) {
  padding: 0.875rem 1rem;
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-medium);
  box-shadow: none;
  transition: all 0.2s ease;
  background: var(--bg-primary);
}

.form-input :deep(.el-input__wrapper:hover) {
  border-color: var(--border-strong);
}

.form-input :deep(.el-input__wrapper.is-focus) {
  border-color: var(--accent-500);
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

.login-button {
  width: 100%;
  height: 3rem;
  font-size: 1rem;
  font-weight: 600;
  border-radius: var(--radius-lg);
  margin-top: 0.5rem;
  background: linear-gradient(135deg, var(--accent-600) 0%, var(--accent-700) 100%);
  border: none;
  transition: all 0.2s ease;
}

.login-button:hover {
  background: linear-gradient(135deg, var(--accent-700) 0%, var(--accent-800) 100%);
  transform: translateY(-1px);
  box-shadow: var(--shadow-lg);
}

/* 表单底部 */
.form-footer {
  text-align: center;
}

.demo-accounts {
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
}

.demo-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--neutral-700);
  margin: 0 0 1rem 0;
}

.demo-list {
  display: flex;
  gap: 0.75rem;
  justify-content: center;
}

.demo-account {
  padding: 0.75rem 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-medium);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
  min-width: 5rem;
}

.demo-account:hover {
  border-color: var(--accent-400);
  background: var(--accent-50);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.demo-role {
  font-size: 0.75rem;
  color: var(--neutral-600);
  font-weight: 500;
}

.demo-username {
  font-size: 0.875rem;
  color: var(--neutral-900);
  font-weight: 600;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .login-content {
    grid-template-columns: 1fr;
    margin: 1rem;
  }

  .brand-section {
    padding: 2rem;
    min-height: auto;
  }

  .brand-title {
    font-size: 1.5rem;
  }

  .brand-subtitle {
    font-size: 1rem;
    margin-bottom: 2rem;
  }

  .feature-list {
    gap: 1rem;
  }

  .form-section {
    padding: 2rem;
  }

  .demo-list {
    flex-direction: column;
    gap: 0.5rem;
  }

  .demo-account {
    min-width: auto;
    width: 100%;
  }
}
</style>
