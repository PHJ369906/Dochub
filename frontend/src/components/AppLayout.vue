<template>
  <div class="app-layout">
    <!-- 侧边栏 -->
    <aside class="sidebar" :class="{ 'sidebar-collapsed': sidebarCollapsed }">
      <div class="sidebar-header">
        <div class="logo-container">
          <div class="logo" :class="{ 'logo-clickable': sidebarCollapsed }" @click="sidebarCollapsed && toggleSidebar()">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div v-if="!sidebarCollapsed" class="logo-text">
            <h2>DocHub</h2>
            <span>文档管理系统</span>
          </div>
        </div>
        <button
          v-if="!sidebarCollapsed"
          class="sidebar-toggle"
          @click="toggleSidebar"
          title="收起侧边栏"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"/>
          </svg>
        </button>
      </div>

      <nav class="sidebar-nav">
        <div class="nav-section">
          <h3 v-if="!sidebarCollapsed" class="nav-section-title">主要功能</h3>
          <ul class="nav-menu">
            <li class="nav-item">
              <router-link to="/home" class="nav-link" active-class="nav-link-active">
                <div class="nav-icon">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                    <polyline points="9,22 9,12 15,12 15,22"/>
                  </svg>
                </div>
                <span v-if="!sidebarCollapsed" class="nav-text">工作台</span>
              </router-link>
            </li>
            <li class="nav-item">
              <router-link to="/files" class="nav-link" active-class="nav-link-active">
                <div class="nav-icon">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                    <polyline points="13,2 13,9 20,9"/>
                  </svg>
                </div>
                <span v-if="!sidebarCollapsed" class="nav-text">文档管理</span>
              </router-link>
            </li>
          </ul>
        </div>

        <div class="nav-section" v-if="user?.role === 'admin'">
          <h3 v-if="!sidebarCollapsed" class="nav-section-title">系统管理</h3>
          <ul class="nav-menu">
            <li class="nav-item">
              <router-link to="/users" class="nav-link" active-class="nav-link-active">
                <div class="nav-icon">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                    <circle cx="9" cy="7" r="4"/>
                    <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                    <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                  </svg>
                </div>
                <span v-if="!sidebarCollapsed" class="nav-text">用户管理</span>
              </router-link>
            </li>
          </ul>
        </div>
      </nav>

      <div class="sidebar-footer">
        <div ref="userProfileRef" class="user-profile" @click="handleUserProfileClick">
          <div class="user-avatar">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
          </div>
          <div v-if="!sidebarCollapsed" class="user-info">
            <div class="user-name">{{ user?.username }}</div>
            <div class="user-role">{{ user?.role === 'admin' ? '管理员' : '普通用户' }}</div>
          </div>
          <div v-if="!sidebarCollapsed" class="user-menu-toggle">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <polyline points="6,9 12,15 18,9"/>
            </svg>
          </div>
        </div>
      </div>

      <!-- 用户菜单 Teleport 到 body，不受 overflow: hidden 影响 -->
      <Teleport to="body">
        <div v-if="showUserMenu" ref="userMenuRef" class="user-menu-popover" :style="userMenuStyle">
          <button class="user-menu-item" @click="goToProfile">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
            个人资料
          </button>
          <button class="user-menu-item logout" @click="handleLogout">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
              <polyline points="16,17 21,12 16,7"/>
              <line x1="21" y1="12" x2="9" y2="12"/>
            </svg>
            退出登录
          </button>
        </div>
      </Teleport>
    </aside>

    <!-- 主内容区域 -->
    <main class="main-content">
      <div class="content-wrapper">
        <router-view />
      </div>
    </main>

    <!-- 移动端遮罩 -->
    <div 
      v-if="showMobileOverlay" 
      class="mobile-overlay" 
      @click="closeMobileSidebar"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const sidebarCollapsed = ref(false)
const showUserMenu = ref(false)
const showMobileOverlay = ref(false)
const isMobile = ref(false)
const userProfileRef = ref<HTMLElement>()
const userMenuRef = ref<HTMLElement>()
const userMenuStyle = ref<Record<string, string>>({})

const user = computed(() => authStore.user)

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
  if (isMobile.value) {
    showMobileOverlay.value = !sidebarCollapsed.value
  }
}

const closeMobileSidebar = () => {
  if (isMobile.value) {
    sidebarCollapsed.value = true
    showMobileOverlay.value = false
  }
}

const handleResize = () => {
  isMobile.value = window.innerWidth < 768
  if (isMobile.value) {
    sidebarCollapsed.value = true
    showMobileOverlay.value = false
  }
}

const goToProfile = () => {
  showUserMenu.value = false
  router.push('/profile')
}

const updateMenuPosition = () => {
  if (!userProfileRef.value) return
  const rect = userProfileRef.value.getBoundingClientRect()
  if (sidebarCollapsed.value) {
    // 收缩状态：菜单在头像右侧弹出
    userMenuStyle.value = {
      left: `${rect.right + 8}px`,
      bottom: `${window.innerHeight - rect.bottom}px`
    }
  } else {
    // 展开状态：菜单在头像上方弹出
    userMenuStyle.value = {
      left: `${rect.left}px`,
      bottom: `${window.innerHeight - rect.top + 8}px`,
      width: `${rect.width}px`
    }
  }
}

const handleUserProfileClick = () => {
  showUserMenu.value = !showUserMenu.value
  if (showUserMenu.value) {
    nextTick(updateMenuPosition)
  }
}

const handleClickOutside = (e: MouseEvent) => {
  if (!showUserMenu.value) return
  const target = e.target as Node
  if (
    userProfileRef.value?.contains(target) ||
    userMenuRef.value?.contains(target)
  ) return
  showUserMenu.value = false
}

const handleLogout = async () => {
  showUserMenu.value = false
  try {
    await ElMessageBox.confirm('确定要退出登录吗？', '退出确认', {
      confirmButtonText: '退出',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await authStore.logout()
    ElMessage.success('退出成功')
    router.push('/login')
  } catch (error) {
    // 用户取消操作
  }
}

onMounted(() => {
  handleResize()
  window.addEventListener('resize', handleResize)
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
  background: var(--bg-secondary);
}

/* 简约侧边栏 */
.sidebar {
  width: 280px;
  background: var(--bg-primary);
  border-right: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  z-index: 1000;
  transition: width 0.2s ease;
  overflow: hidden;
}

.sidebar-collapsed .sidebar-header {
  justify-content: center;
  padding: 2rem 0.5rem;
}

.sidebar-collapsed {
  width: 80px;
}

.sidebar-header {
  padding: 2rem 1.5rem;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 90px;
}

.logo-container {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
}

.logo {
  width: 3rem;
  height: 3rem;
  background: var(--accent-600);
  border-radius: var(--radius-xl);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.logo svg {
  width: 1.5rem;
  height: 1.5rem;
  color: white;
}

.logo-clickable {
  cursor: pointer;
}

.logo-clickable:hover {
  opacity: 0.85;
}

.logo-text h2 {
  font-size: 1.375rem;
  font-weight: 800;
  color: var(--neutral-900);
  margin: 0;
  line-height: 1.1;
  letter-spacing: -0.025em;
}

.logo-text span {
  font-size: 0.75rem;
  color: var(--neutral-600);
  line-height: 1.2;
  font-weight: 500;
}

.sidebar-toggle {
  width: 1.75rem;
  height: 1.75rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.sidebar-toggle:hover {
  background: var(--bg-tertiary);
  border-color: var(--border-medium);
}

.sidebar-toggle svg {
  width: 0.875rem;
  height: 0.875rem;
  color: var(--neutral-500);
}

/* 导航 */
.sidebar-nav {
  flex: 1;
  padding: 1rem 0;
  overflow-y: auto;
}

.nav-section {
  margin-bottom: 2rem;
}

.nav-section:last-child {
  margin-bottom: 0;
}

.nav-section-title {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--neutral-500);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 1rem 0;
  padding: 0 1.5rem;
}

.nav-menu {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  margin-bottom: 0.25rem;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1.5rem;
  color: var(--neutral-600);
  text-decoration: none;
  transition: all 0.2s ease;
  border-radius: 0 var(--radius-lg) var(--radius-lg) 0;
  margin-right: 1rem;
  font-weight: 500;
}

.nav-link:hover {
  background: var(--bg-tertiary);
  color: var(--neutral-900);
}

.nav-link-active {
  background: var(--accent-50);
  color: var(--accent-700);
  font-weight: 600;
  border-left: 2px solid var(--accent-600);
  border-radius: 0 var(--radius-lg) var(--radius-lg) 0;
}

.nav-icon {
  width: 1.5rem;
  height: 1.5rem;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.nav-link-active .nav-icon {
  color: var(--accent-600);
}

/* 收缩状态导航 */
.sidebar-collapsed .nav-link {
  justify-content: center;
  padding: 0.75rem;
  margin-right: 0;
  border-radius: var(--radius-lg);
  margin: 0 0.75rem 0.25rem;
}

.sidebar-collapsed .nav-link-active {
  border-left: none;
  border-radius: var(--radius-lg);
}

.sidebar-collapsed .nav-icon {
  width: 2rem;
  height: 2rem;
  border-radius: var(--radius-md);
}

.sidebar-collapsed .nav-section-title {
  display: none;
}

.nav-icon svg {
  width: 1rem;
  height: 1rem;
  stroke-width: 2;
}

.nav-link-active .nav-icon svg {
  color: var(--accent-700);
}

.nav-text {
  font-size: 0.875rem;
  font-weight: 500;
  letter-spacing: -0.025em;
}

/* 侧边栏底部 */
.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid var(--border-light);
  position: relative;
}

.user-profile {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all 0.2s ease;
}

.user-profile:hover {
  background: var(--bg-secondary);
}

/* 收缩状态底部 */
.sidebar-collapsed .user-profile {
  justify-content: center;
}

.sidebar-collapsed .sidebar-footer {
  padding: 0.75rem;
}

.user-avatar {
  width: 2rem;
  height: 2rem;
  background: var(--accent-100);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.user-avatar svg {
  width: 1rem;
  height: 1rem;
  color: var(--accent-600);
}

.user-info {
  flex: 1;
  min-width: 0;
}

.user-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--neutral-900);
  line-height: 1.2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.user-role {
  font-size: 0.75rem;
  color: var(--neutral-500);
  line-height: 1.2;
}

.user-menu-toggle {
  width: 1rem;
  height: 1rem;
  flex-shrink: 0;
}

.user-menu-toggle svg {
  width: 100%;
  height: 100%;
  color: var(--neutral-400);
}

/* 主内容区域 */
.main-content {
  flex: 1;
  margin-left: 280px;
  transition: all 0.3s ease;
  min-height: 100vh;
}

.sidebar-collapsed + .main-content {
  margin-left: 80px;
}

.content-wrapper {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

/* 移动端遮罩 */
.mobile-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
  opacity: 0;
  animation: fadeIn 0.3s ease forwards;
}

/* 动画 */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active {
  transition: all 0.3s ease;
}

.slide-up-leave-active {
  transition: all 0.2s ease;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar {
    transform: translateX(-100%);
  }
  
  .sidebar:not(.sidebar-collapsed) {
    transform: translateX(0);
  }
  
  .main-content {
    margin-left: 0;
  }
  
  .content-wrapper {
    padding: 1rem;
  }
}

@keyframes fadeIn {
  to {
    opacity: 1;
  }
}
</style>

<style>
/* 用户菜单弹出层 - 全局样式（Teleport 到 body） */
.user-menu-popover {
  position: fixed;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  padding: 0.5rem;
  z-index: 1100;
  min-width: 140px;
}

.user-menu-popover .user-menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: none;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  color: var(--neutral-700);
  white-space: nowrap;
}

.user-menu-popover .user-menu-item:hover {
  background: var(--bg-secondary);
  color: var(--neutral-900);
}

.user-menu-popover .user-menu-item.logout:hover {
  background: var(--error);
  color: white;
}

.user-menu-popover .user-menu-item svg {
  width: 1rem;
  height: 1rem;
  flex-shrink: 0;
}
</style>