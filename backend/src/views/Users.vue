<template>
  <div class="users-container">
    <el-page-header @back="goBack" content="用户管理" />
    
    <div class="users-content">
      <el-card>
        <template #header>
          <div class="card-header">
            <span>用户列表</span>
            <el-button type="primary" @click="refreshData">
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
          </div>
        </template>
        
        <!-- 搜索栏 -->
        <div class="search-bar">
          <el-form :model="searchForm" inline>
            <el-form-item label="用户名">
              <el-input
                v-model="searchForm.username"
                placeholder="请输入用户名"
                clearable
                style="width: 200px"
              />
            </el-form-item>
            <el-form-item label="角色">
              <el-select
                v-model="searchForm.role"
                placeholder="请选择角色"
                clearable
                style="width: 120px"
              >
                <el-option label="管理员" value="admin" />
                <el-option label="普通用户" value="user" />
              </el-select>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleSearch">搜索</el-button>
              <el-button @click="handleReset">重置</el-button>
            </el-form-item>
          </el-form>
        </div>
        
        <!-- 用户表格 -->
        <el-table
          v-loading="loading"
          :data="userList"
          style="width: 100%"
          stripe
        >
          <el-table-column prop="id" label="ID" width="80" />
          <el-table-column prop="username" label="用户名" />
          <el-table-column prop="email" label="邮箱" />
          <el-table-column prop="role" label="角色" width="100">
            <template #default="{ row }">
              <el-tag :type="row.role === 'admin' ? 'danger' : 'primary'">
                {{ row.role === 'admin' ? '管理员' : '普通用户' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="enabled" label="状态" width="100">
            <template #default="{ row }">
              <el-tag :type="row.enabled ? 'success' : 'danger'">
                {{ row.enabled ? '正常' : '禁用' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="createTime" label="注册时间" width="180">
            <template #default="{ row }">
              {{ formatDate(row.createTime) }}
            </template>
          </el-table-column>
          <el-table-column label="操作" width="150" fixed="right">
            <template #default="{ row }">
              <el-button
                type="primary"
                size="small"
                @click="handleView(row)"
              >
                查看
              </el-button>
              <el-button
                :type="row.enabled ? 'danger' : 'success'"
                size="small"
                @click="handleToggleStatus(row)"
              >
                {{ row.enabled ? '禁用' : '启用' }}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        
        <!-- 分页 -->
        <div class="pagination-container">
          <el-pagination
            v-model:current-page="pagination.page"
            v-model:page-size="pagination.size"
            :page-sizes="[10, 20, 50, 100]"
            :total="pagination.total"
            layout="total, sizes, prev, pager, next, jumper"
            @size-change="handleSizeChange"
            @current-change="handleCurrentChange"
          />
        </div>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh } from '@element-plus/icons-vue'
import { getUserList } from '@/api/auth'
import type { User } from '@/types/auth'

const router = useRouter()

const loading = ref(false)
const userList = ref<User[]>([])

const searchForm = reactive({
  username: '',
  role: ''
})

const pagination = reactive({
  page: 1,
  size: 10,
  total: 0
})

onMounted(() => {
  fetchUserList()
})

const fetchUserList = async () => {
  loading.value = true
  try {
    const response = await getUserList({
      page: pagination.page - 1, // 后端从0开始
      size: pagination.size,
      ...searchForm
    })
    
    if (response.code === 200) {
      userList.value = response.data.content || []
      pagination.total = response.data.totalElements || 0
    } else {
      ElMessage.error(response.message || '获取用户列表失败')
    }
  } catch (error) {
    ElMessage.error('获取用户列表失败')
  } finally {
    loading.value = false
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

const goBack = () => {
  router.back()
}

const refreshData = () => {
  fetchUserList()
}

const handleSearch = () => {
  pagination.page = 1
  fetchUserList()
}

const handleReset = () => {
  searchForm.username = ''
  searchForm.role = ''
  pagination.page = 1
  fetchUserList()
}

const handleSizeChange = (size: number) => {
  pagination.size = size
  pagination.page = 1
  fetchUserList()
}

const handleCurrentChange = (page: number) => {
  pagination.page = page
  fetchUserList()
}

const handleView = (user: User) => {
  ElMessage.info(`查看用户: ${user.username}`)
  // 这里可以打开用户详情对话框或跳转到详情页
}

const handleToggleStatus = async (user: User) => {
  const action = user.enabled ? '禁用' : '启用'
  try {
    await ElMessageBox.confirm(
      `确定要${action}用户 "${user.username}" 吗？`,
      '确认操作',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 这里应该调用切换用户状态的API
    ElMessage.success(`${action}成功`)
    fetchUserList()
  } catch (error) {
    // 用户取消操作
  }
}
</script>

<style scoped>
.users-container {
  padding: 20px;
  background-color: #f5f5f5;
  min-height: 100vh;
}

.users-content {
  margin-top: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.search-bar {
  margin-bottom: 20px;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.pagination-container {
  margin-top: 20px;
  text-align: right;
}
</style>
