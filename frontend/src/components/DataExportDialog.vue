<template>
  <el-dialog
    v-model="dialogVisible"
    title="导出数据"
    width="680px"
    :before-close="handleClose"
  >
    <div class="export-dialog">
      <!-- 选择模式 -->
      <div class="export-section">
        <h4 class="section-title">选择导出内容</h4>
        <el-tabs v-model="activeTab" class="export-tabs">
          <el-tab-pane label="按分类选择" name="category">
            <div class="tree-container">
              <div class="tree-actions">
                <el-button link size="small" @click="handleCheckAll(true)">全选</el-button>
                <el-button link size="small" @click="handleCheckAll(false)">取消全选</el-button>
              </div>
              <el-tree
                ref="categoryTreeRef"
                :data="categoryTree"
                :props="treeProps"
                show-checkbox
                node-key="id"
                default-expand-all
                class="export-tree"
              >
                <template #default="{ node, data }">
                  <span class="tree-node">
                    <span>{{ node.label }}</span>
                    <span class="file-count-badge">{{ data.fileCount || 0 }} 个文件</span>
                  </span>
                </template>
              </el-tree>
              <div v-if="categoryTree.length === 0" class="empty-tip">暂无分类数据</div>
            </div>
          </el-tab-pane>
          <el-tab-pane label="按文件选择" name="file">
            <div class="file-select-container">
              <el-input
                v-model="fileSearchKeyword"
                placeholder="搜索文件..."
                clearable
                class="file-search"
              />
              <div class="file-list-wrapper">
                <el-checkbox-group v-model="selectedFileIds">
                  <div v-for="file in filteredFiles" :key="file.id" class="file-check-item">
                    <el-checkbox :value="file.id">
                      <div class="file-check-info">
                        <span class="file-check-name">{{ file.title }}</span>
                        <span class="file-check-meta">
                          {{ file.fileType.toUpperCase() }} · {{ formatFileSize(file.fileSize) }}
                        </span>
                      </div>
                    </el-checkbox>
                  </div>
                </el-checkbox-group>
                <div v-if="filteredFiles.length === 0" class="empty-tip">暂无文件数据</div>
              </div>
              <div class="file-select-actions">
                <el-button link size="small" @click="selectAllFiles(true)">全选</el-button>
                <el-button link size="small" @click="selectAllFiles(false)">取消全选</el-button>
                <span class="selected-count">已选 {{ selectedFileIds.length }} 个文件</span>
              </div>
            </div>
          </el-tab-pane>
        </el-tabs>
      </div>

      <!-- 导出统计 -->
      <div class="export-summary">
        <div class="summary-item">
          <span class="summary-label">分类</span>
          <span class="summary-value">{{ exportStats.categories }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">文件</span>
          <span class="summary-value">{{ exportStats.files }}</span>
        </div>
      </div>
    </div>

    <template #footer>
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" :loading="exporting" @click="handleExport" :disabled="!hasSelection">
        {{ exporting ? '导出中...' : '导出' }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { save } from '@tauri-apps/plugin-dialog'
import { getCategories, getFileList, exportData } from '@/api/file'

interface Props {
  modelValue: boolean
  categories: any[]
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const activeTab = ref('category')
const exporting = ref(false)
const categoryTreeRef = ref()
const fileSearchKeyword = ref('')
const selectedFileIds = ref<number[]>([])
const allFiles = ref<any[]>([])
const categoryTree = ref<any[]>([])

const treeProps = {
  children: 'children',
  label: 'name'
}

const filteredFiles = computed(() => {
  if (!fileSearchKeyword.value) return allFiles.value
  const kw = fileSearchKeyword.value.toLowerCase()
  return allFiles.value.filter(f =>
    f.title.toLowerCase().includes(kw) || f.originalName.toLowerCase().includes(kw)
  )
})

const exportStats = computed(() => {
  if (activeTab.value === 'category') {
    const checkedNodes = categoryTreeRef.value?.getCheckedNodes() || []
    return {
      categories: checkedNodes.length,
      files: checkedNodes.reduce((sum: number, n: any) => sum + (n.fileCount || 0), 0)
    }
  }
  return {
    categories: 0,
    files: selectedFileIds.value.length
  }
})

const hasSelection = computed(() => {
  if (activeTab.value === 'category') {
    return (categoryTreeRef.value?.getCheckedKeys() || []).length > 0
  }
  return selectedFileIds.value.length > 0
})

const handleCheckAll = (checked: boolean) => {
  if (checked) {
    categoryTreeRef.value?.setCheckedNodes(categoryTree.value)
  } else {
    categoryTreeRef.value?.setCheckedKeys([])
  }
}

const selectAllFiles = (checked: boolean) => {
  if (checked) {
    selectedFileIds.value = allFiles.value.map(f => f.id)
  } else {
    selectedFileIds.value = []
  }
}

const formatFileSize = (size: number) => {
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB'
  if (size < 1024 * 1024 * 1024) return (size / 1024 / 1024).toFixed(1) + ' MB'
  return (size / 1024 / 1024 / 1024).toFixed(1) + ' GB'
}

const loadData = async () => {
  try {
    // 加载分类树
    const catRes = await getCategories()
    if (catRes.code === 200) {
      categoryTree.value = buildTree(catRes.data)
    }

    // 加载所有文件
    const fileRes = await getFileList({ page: 0, size: 9999 })
    if (fileRes.code === 200) {
      allFiles.value = fileRes.data.records || []
    }
  } catch (error) {
    console.error('加载数据失败:', error)
  }
}

const buildTree = (categories: any[]): any[] => {
  const map = new Map()
  const result: any[] = []
  categories.forEach(c => map.set(c.id, { ...c, children: [] }))
  categories.forEach(c => {
    const node = map.get(c.id)
    if (c.parentId && map.has(c.parentId)) {
      map.get(c.parentId).children.push(node)
    } else {
      result.push(node)
    }
  })
  return result
}

const handleExport = async () => {
  // 选择保存路径
  const savePath = await save({
    defaultPath: `DocHub_export_${new Date().toISOString().slice(0, 10)}.dochub`,
    filters: [{
      name: 'DocHub 数据包',
      extensions: ['dochub']
    }]
  })

  if (!savePath) return

  exporting.value = true
  try {
    let fileIds: number[] = []
    let categoryIds: number[] = []

    if (activeTab.value === 'category') {
      categoryIds = categoryTreeRef.value?.getCheckedKeys() || []
    } else {
      fileIds = selectedFileIds.value
    }

    const res = await exportData({
      fileIds,
      categoryIds,
      exportPath: savePath,
    })

    if (res.code === 200) {
      ElMessage.success(`导出成功！共 ${res.data.fileCount} 个文件，${res.data.categoryCount} 个分类`)
      handleClose()
    }
  } catch (error: any) {
    ElMessage.error('导出失败: ' + (error.message || error))
  } finally {
    exporting.value = false
  }
}

const handleClose = () => {
  dialogVisible.value = false
  selectedFileIds.value = []
  fileSearchKeyword.value = ''
  categoryTreeRef.value?.setCheckedKeys([])
}

watch(dialogVisible, (visible) => {
  if (visible) {
    loadData()
  }
})
</script>

<style scoped>
.export-dialog {
  max-height: 60vh;
  overflow-y: auto;
}

.section-title {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.tree-container {
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 8px;
  padding: 12px;
  max-height: 350px;
  overflow-y: auto;
}

.tree-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.file-count-badge {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  background: var(--el-fill-color-light);
  padding: 1px 6px;
  border-radius: 10px;
}

.file-select-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.file-list-wrapper {
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 8px;
  padding: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.file-check-item {
  padding: 6px 8px;
  border-radius: 6px;
  transition: background 0.2s;
}

.file-check-item:hover {
  background: var(--el-fill-color-light);
}

.file-check-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.file-check-name {
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.file-check-meta {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.file-select-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.selected-count {
  margin-left: auto;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.export-summary {
  display: flex;
  gap: 24px;
  padding: 12px 16px;
  margin-top: 16px;
  background: var(--el-fill-color-lighter);
  border-radius: 8px;
}

.summary-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.summary-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.summary-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--el-color-primary);
}

.empty-tip {
  text-align: center;
  padding: 24px 0;
  font-size: 13px;
  color: var(--el-text-color-placeholder);
}

.export-tabs :deep(.el-tabs__header) {
  margin-bottom: 12px;
}
</style>
