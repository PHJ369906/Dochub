<template>
  <div class="profile-container">
    <el-page-header @back="goBack" content="个人资料" />
    
    <div class="profile-content">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-card class="profile-card">
            <div class="avatar-section">
              <el-avatar :size="120" :src="user?.avatar">
                <User />
              </el-avatar>
              <h2>{{ user?.username }}</h2>
              <el-tag :type="user?.role === 'admin' ? 'danger' : 'primary'">
                {{ user?.role === 'admin' ? '管理员' : '普通用户' }}
              </el-tag>
            </div>
          </el-card>
        </el-col>
        
        <el-col :span="16">
          <el-card class="info-card">
            <template #header>
              <div class="card-header">
                <span>基本信息</span>
                <el-button type="primary" size="small" @click="editMode = !editMode">
                  {{ editMode ? '取消编辑' : '编辑资料' }}
                </el-button>
              </div>
            </template>
            
            <el-form
              ref="profileFormRef"
              :model="profileForm"
              :rules="profileRules"
              label-width="100px"
              :disabled="!editMode"
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
              
              <el-form-item label="注册时间">
                <el-input :value="formatDate(user?.createTime)" disabled />
              </el-form-item>
              
              <el-form-item label="账户状态">
                <el-tag :type="user?.enabled ? 'success' : 'danger'">
                  {{ user?.enabled ? '正常' : '已禁用' }}
                </el-tag>
              </el-form-item>
              
              <el-form-item v-if="editMode">
                <el-button type="primary" @click="handleSave" :loading="saving">
                  保存修改
                </el-button>
                <el-button @click="handleCancel">取消</el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
      </el-row>
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

const goBack = () => {
  router.back()
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
.profile-container {
  padding: 20px;
  background-color: #f5f5f5;
  min-height: 100vh;
}

.profile-content {
  margin-top: 20px;
}

.profile-card {
  text-align: center;
}

.avatar-section {
  padding: 20px;
}

.avatar-section h2 {
  margin: 16px 0 8px 0;
  color: #303133;
}

.info-card {
  height: fit-content;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
