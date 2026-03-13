<template>
  <div class="profile-page fade-in">
    <div class="page-header">
      <h1 class="page-title">个人资料</h1>
      <p class="page-subtitle">管理您的账户信息</p>
    </div>

    <div class="profile-content">
      <div class="profile-grid">
        <!-- 左侧头像卡片 -->
        <div class="avatar-card">
          <div class="avatar-wrapper">
            <el-avatar :size="96" :src="user?.avatar">
              <User />
            </el-avatar>
          </div>
          <h2 class="user-name">{{ user?.username }}</h2>
          <el-tag :type="user?.role === 'admin' ? 'danger' : 'primary'" size="small">
            {{ user?.role === 'admin' ? '管理员' : '普通用户' }}
          </el-tag>
          <div class="avatar-meta">
            <div class="meta-item">
              <span class="meta-label">注册时间</span>
              <span class="meta-value">{{ formatDate(user?.createTime) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">账户状态</span>
              <el-tag :type="user?.enabled ? 'success' : 'danger'" size="small">
                {{ user?.enabled ? '正常' : '已禁用' }}
              </el-tag>
            </div>
          </div>
        </div>

        <!-- 右侧信息表单 -->
        <div class="info-card">
          <div class="card-header">
            <h3>基本信息</h3>
            <el-button
              :type="editMode ? 'default' : 'primary'"
              size="small"
              @click="editMode = !editMode"
            >
              {{ editMode ? '取消编辑' : '编辑资料' }}
            </el-button>
          </div>

          <el-form
            ref="profileFormRef"
            :model="profileForm"
            :rules="profileRules"
            label-width="100px"
            :disabled="!editMode"
            class="profile-form"
          >
            <el-form-item label="用户名" prop="username">
              <el-input v-model="profileForm.username" disabled />
            </el-form-item>

            <el-form-item label="邮箱" prop="email">
              <el-input v-model="profileForm.email" />
            </el-form-item>

            <el-form-item label="角色">
              <el-input :value="user?.role === 'admin' ? '管理员' : '普通用户'" disabled />
            </el-form-item>

            <el-form-item v-if="editMode" class="form-actions">
              <el-button type="primary" @click="handleSave" :loading="saving">
                保存修改
              </el-button>
              <el-button @click="handleCancel">取消</el-button>
            </el-form-item>
          </el-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { useAuthStore } from '@/stores/auth'
import { User } from '@element-plus/icons-vue'

const router = useRouter()
const authStore = useAuthStore()

const profileFormRef = ref<FormInstance>()
const editMode = ref(false)
const saving = ref(false)

const user = computed(() => authStore.user)

const profileForm = reactive({
  username: '',
  email: ''
})

const profileRules: FormRules = {
  email: [
    { type: 'email', message: '请输入正确的邮箱地址', trigger: 'blur' }
  ]
}

// 监听用户信息变化，更新表单
watch(user, (newUser) => {
  if (newUser) {
    profileForm.username = newUser.username
    profileForm.email = newUser.email || ''
  }
}, { immediate: true })

const formatDate = (dateString?: string) => {
  if (!dateString) return '-'
  return new Date(dateString).toLocaleString('zh-CN')
}

const handleSave = async () => {
  if (!profileFormRef.value) return

  await profileFormRef.value.validate(async (valid) => {
    if (valid) {
      saving.value = true
      try {
        // 这里应该调用更新用户信息的API
        // await updateUserProfile(profileForm)
        ElMessage.success('保存成功')
        editMode.value = false
      } catch (error) {
        ElMessage.error('保存失败，请重试')
      } finally {
        saving.value = false
      }
    }
  })
}

const handleCancel = () => {
  // 重置表单数据
  if (user.value) {
    profileForm.username = user.value.username
    profileForm.email = user.value.email || ''
  }
  editMode.value = false
}
</script>

<style scoped>
.profile-page {
  max-width: 960px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 2rem;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--neutral-900);
  margin: 0 0 0.25rem 0;
}

.page-subtitle {
  font-size: 0.875rem;
  color: var(--neutral-500);
  margin: 0;
}

.profile-grid {
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: 1.5rem;
  align-items: start;
}

.avatar-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-xl);
  padding: 2rem;
  text-align: center;
  box-shadow: var(--shadow-sm);
}

.avatar-wrapper {
  margin-bottom: 1rem;
}

.user-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--neutral-900);
  margin: 0 0 0.5rem 0;
}

.avatar-meta {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-light);
  text-align: left;
}

.meta-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
}

.meta-label {
  font-size: 0.813rem;
  color: var(--neutral-500);
}

.meta-value {
  font-size: 0.813rem;
  color: var(--neutral-700);
  font-weight: 500;
}

.info-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-xl);
  padding: 2rem;
  box-shadow: var(--shadow-sm);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-light);
}

.card-header h3 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--neutral-900);
  margin: 0;
}

.profile-form .el-form-item {
  margin-bottom: 1.25rem;
}

.form-actions {
  margin-top: 0.5rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-light);
}

@media (max-width: 768px) {
  .profile-grid {
    grid-template-columns: 1fr;
  }
}
</style>
