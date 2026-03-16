<template>
  <div class="dashboard-page">
    <!-- 欢迎区域 -->
    <section class="welcome-section">
      <div class="welcome-card surface-elevated surface-interactive">
        <div class="welcome-content">
          <div class="welcome-text">
            <h1 class="welcome-title">
              欢迎回来，<span class="gradient-text">{{ user?.username }}</span>
            </h1>
            <p class="welcome-subtitle">开始管理您的文档，享受高效的工作体验</p>
          </div>
          <div class="welcome-stats">
            <div class="stat-card">
              <div class="stat-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
                  <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
                </svg>
              </div>
              <div class="stat-info">
                <div class="stat-value">{{ user?.role === 'admin' ? '管理员' : '普通用户' }}</div>
                <div class="stat-label">当前身份</div>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <circle cx="12" cy="12" r="10"/>
                  <polyline points="12,6 12,12 16,14"/>
                </svg>
              </div>
              <div class="stat-info">
                <div class="stat-value">{{ formatDate(user?.lastLoginTime) }}</div>
                <div class="stat-label">上次登录</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- 快速操作区域 -->
    <section class="quick-actions-section">
      <div class="section-header">
        <h2 class="section-title">快速操作</h2>
        <p class="section-subtitle">选择您需要的功能模块</p>
      </div>
      
      <div class="action-grid">
        <div class="action-card primary surface-interactive" @click="goToFiles">
          <div class="card-content">
            <div class="card-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                <polyline points="13,2 13,9 20,9"/>
              </svg>
            </div>
            <h3 class="card-title">文档管理</h3>
            <p class="card-description">上传、查看、管理您的文档</p>
            <div class="card-arrow">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <line x1="7" y1="17" x2="17" y2="7"/>
                <polyline points="7,7 17,7 17,17"/>
              </svg>
            </div>
          </div>
        </div>

        <div class="action-card surface-interactive" @click="goToProfile">
          <div class="card-content">
            <div class="card-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                <circle cx="12" cy="7" r="4"/>
              </svg>
            </div>
            <h3 class="card-title">个人设置</h3>
            <p class="card-description">管理您的账户信息和偏好设置</p>
            <div class="card-arrow">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <line x1="7" y1="17" x2="17" y2="7"/>
                <polyline points="7,7 17,7 17,17"/>
              </svg>
            </div>
          </div>
        </div>

        <div v-if="user?.role === 'admin'" class="action-card surface-interactive" @click="goToUsers">
          <div class="card-content">
            <div class="card-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
              </svg>
            </div>
            <h3 class="card-title">用户管理</h3>
            <p class="card-description">管理系统用户和权限分配</p>
            <div class="card-arrow">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <line x1="7" y1="17" x2="17" y2="7"/>
                <polyline points="7,7 17,7 17,17"/>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- 最近活动 -->
    <section class="recent-activity-section">
      <div class="section-header">
        <h2 class="section-title">系统概览</h2>
        <p class="section-subtitle">了解系统运行状态</p>
      </div>
      
      <div class="activity-grid">
        <div class="activity-card surface-elevated">
          <div class="activity-header">
            <div class="activity-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                <polyline points="13,2 13,9 20,9"/>
              </svg>
            </div>
            <h3>文档统计</h3>
          </div>
          <div class="activity-content">
            <div class="activity-metric">
              <span class="metric-value">{{ totalFileCount }}</span>
              <span class="metric-label">总文档数</span>
            </div>
            <div class="activity-description">
              {{ totalFileCount > 0 ? `已上传 ${totalFileCount} 个文档` : '开始上传您的第一个文档' }}
            </div>
          </div>
        </div>

        <div class="activity-card surface-elevated">
          <div class="activity-header">
            <div class="activity-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <circle cx="12" cy="12" r="10"/>
                <polyline points="12,6 12,12 16,14"/>
              </svg>
            </div>
            <h3>最近活动</h3>
          </div>
          <div class="activity-content">
            <div class="activity-metric">
              <span class="metric-value">今日</span>
              <span class="metric-label">登录时间</span>
            </div>
            <div class="activity-description">
              欢迎回到文档管理系统
            </div>
          </div>
        </div>

        <div v-if="user?.role === 'admin'" class="activity-card surface-elevated">
          <div class="activity-header">
            <div class="activity-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
              </svg>
            </div>
            <h3>用户管理</h3>
          </div>
          <div class="activity-content">
            <div class="activity-metric">
              <span class="metric-value">管理员</span>
              <span class="metric-label">当前权限</span>
            </div>
            <div class="activity-description">
              您拥有完整的系统管理权限
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getFileList } from '@/api/file'

const router = useRouter()
const authStore = useAuthStore()

const user = computed(() => authStore.user)
const totalFileCount = ref(0)

const formatDate = (dateString?: string) => {
  if (!dateString) return '首次登录'
  return new Date(dateString).toLocaleDateString('zh-CN')
}

onMounted(async () => {
  try {
    const response = await getFileList({ page: 0, size: 1 })
    if (response.code === 200) {
      totalFileCount.value = response.data.total || 0
    }
  } catch (error) {
    // 加载失败静默处理
  }
})

const goToProfile = () => {
  router.push('/profile')
}

const goToUsers = () => {
  router.push('/users')
}

const goToFiles = () => {
  router.push('/files')
}
</script>

<style scoped>
.dashboard-page {
  max-width: 1200px;
  margin: 0 auto;
}

/* 欢迎区域 */
.welcome-section {
  margin-bottom: 3rem;
}

.welcome-card {
  border-radius: var(--radius-xl);
  padding: 2.5rem;
  background: var(--bg-primary);
}

.welcome-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.welcome-text {
  flex: 1;
}

.welcome-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--neutral-900);
  margin: 0 0 1rem 0;
  line-height: 1.2;
}

.welcome-subtitle {
  font-size: 1.125rem;
  color: var(--neutral-600);
  margin: 0;
  line-height: 1.5;
}

.welcome-stats {
  display: flex;
  gap: 2rem;
  flex-shrink: 0;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
}

.stat-icon {
  width: 2.5rem;
  height: 2.5rem;
  background: var(--accent-100);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-icon svg {
  width: 1.25rem;
  height: 1.25rem;
  color: var(--accent-600);
}

.stat-info {
  text-align: left;
}

.stat-value {
  display: block;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--neutral-900);
  line-height: 1.2;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--neutral-500);
  line-height: 1.2;
}

/* 区域标题 */
.section-header {
  margin-bottom: 2rem;
  text-align: center;
}

.section-title {
  font-size: 1.875rem;
  font-weight: 700;
  color: var(--neutral-900);
  margin: 0 0 0.5rem 0;
}

.section-subtitle {
  font-size: 1rem;
  color: var(--neutral-600);
  margin: 0;
}

/* 快速操作区域 */
.quick-actions-section {
  margin-bottom: 3rem;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 1.5rem;
}

.action-card {
  background: var(--bg-primary);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-light);
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.action-card.primary {
  border-color: var(--accent-200);
}

.action-card:hover {
  background: var(--bg-secondary);
  box-shadow: var(--shadow-md);
}

.card-content {
  padding: 2rem;
  position: relative;
  z-index: 1;
}

.card-icon {
  width: 3rem;
  height: 3rem;
  background: var(--neutral-100);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
}

.action-card.primary .card-icon {
  background: var(--accent-100);
}

.action-card:hover .card-icon {
  background: var(--accent-100);
}

.card-icon svg {
  width: 1.5rem;
  height: 1.5rem;
  color: var(--neutral-600);
}

.action-card.primary .card-icon svg {
  color: var(--accent-600);
}

.card-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--neutral-900);
  margin: 0 0 0.75rem 0;
}

.card-description {
  font-size: 0.875rem;
  color: var(--neutral-600);
  margin: 0 0 1.5rem 0;
  line-height: 1.5;
}

.card-arrow {
  width: 1.5rem;
  height: 1.5rem;
  opacity: 0.4;
  transition: all 0.2s ease;
  margin-left: auto;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-card:hover .card-arrow {
  opacity: 1;
}

.card-arrow svg {
  width: 1rem;
  height: 1rem;
  color: var(--accent-600);
}

/* 最近活动区域 */
.recent-activity-section {
  margin-bottom: 2rem;
}

.activity-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.activity-card {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
  border: 1px solid var(--border-light);
}

.activity-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.activity-icon {
  width: 2rem;
  height: 2rem;
  background: var(--accent-100);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.activity-icon svg {
  width: 1rem;
  height: 1rem;
  color: var(--accent-600);
}

.activity-header h3 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--neutral-900);
  margin: 0;
}

.activity-content {
  text-align: center;
}

.activity-metric {
  margin-bottom: 1rem;
}

.metric-value {
  display: block;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--accent-600);
  line-height: 1.2;
}

.metric-label {
  font-size: 0.875rem;
  color: var(--neutral-500);
  line-height: 1.2;
}

.activity-description {
  font-size: 0.875rem;
  color: var(--neutral-600);
  line-height: 1.4;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .welcome-content {
    flex-direction: column;
    gap: 2rem;
    text-align: center;
  }
  
  .welcome-stats {
    justify-content: center;
    flex-wrap: wrap;
  }
  
  .welcome-title {
    font-size: 2rem;
  }
  
  .action-grid,
  .activity-grid {
    grid-template-columns: 1fr;
  }
  
  .stat-card {
    flex-direction: column;
    text-align: center;
    gap: 0.75rem;
  }
}
</style>
