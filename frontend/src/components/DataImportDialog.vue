<template>
  <el-dialog
    v-model="dialogVisible"
    title="导入数据"
    width="620px"
    :before-close="handleClose"
  >
    <div class="import-dialog">
      <!-- 步骤1：选择文件 -->
      <div v-if="step === 'select'" class="import-step">
        <div class="file-drop-zone" @click="selectFile">
          <div class="drop-icon">
            <svg viewBox="0 0 48 48" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M24 32V16M16 24l8-8 8 8"/>
              <rect x="6" y="6" width="36" height="36" rx="4"/>
            </svg>
          </div>
          <p class="drop-text">点击选择 .dochub 文件</p>
          <p class="drop-hint">选择之前导出的数据包进行导入</p>
        </div>
      </div>

      <!-- 步骤2：预览 -->
      <div v-if="step === 'preview'" class="import-step">
        <div class="preview-header">
          <div class="file-info">
            <span class="file-name">{{ getFileName(importPath) }}</span>
            <el-button link size="small" @click="step = 'select'">重新选择</el-button>
          </div>
        </div>

        <!-- 预览统计 -->
        <div class="preview-stats">
          <div class="stat-card">
            <span class="stat-number">{{ preview?.categoryCount || 0 }}</span>
            <span class="stat-label">分类</span>
          </div>
          <div class="stat-card">
            <span class="stat-number">{{ preview?.fileCount || 0 }}</span>
            <span class="stat-label">文件</span>
          </div>
          <div class="stat-card">
            <span class="stat-number">{{ preview?.tagCount || 0 }}</span>
            <span class="stat-label">标签</span>
          </div>
          <div class="stat-card">
            <span class="stat-number">{{ formatFileSize(preview?.totalSize || 0) }}</span>
            <span class="stat-label">总大小</span>
          </div>
        </div>

        <!-- 导出信息 -->
        <div class="preview-meta">
          <span>导出时间：{{ preview?.exportTime }}</span>
          <span>版本：{{ preview?.version }}</span>
        </div>

        <!-- 分类预览 -->
        <div v-if="preview?.categories && preview.categories.length > 0" class="preview-section">
          <h4 class="section-title">分类列表</h4>
          <div class="category-preview-list">
            <div v-for="cat in preview.categories" :key="cat.name" class="category-preview-item">
              <span class="cat-name">{{ cat.name }}</span>
              <span class="cat-count">{{ cat.fileCount }} 个文件</span>
            </div>
          </div>
        </div>

        <!-- 冲突警告 -->
        <div v-if="hasConflicts" class="conflict-section">
          <el-alert type="warning" :closable="false" show-icon>
            <template #title>
              检测到数据冲突
            </template>
            <div class="conflict-details">
              <div v-if="preview?.conflictFiles?.length" class="conflict-group">
                <p>同名文件 ({{ preview.conflictFiles.length }} 个)：</p>
                <ul>
                  <li v-for="f in preview.conflictFiles.slice(0, 5)" :key="f">{{ f }}</li>
                  <li v-if="preview.conflictFiles.length > 5">...等 {{ preview.conflictFiles.length }} 个</li>
                </ul>
              </div>
              <div v-if="preview?.conflictCategories?.length" class="conflict-group">
                <p>同名分类 ({{ preview.conflictCategories.length }} 个)：</p>
                <ul>
                  <li v-for="c in preview.conflictCategories.slice(0, 5)" :key="c">{{ c }}</li>
                  <li v-if="preview.conflictCategories.length > 5">...等 {{ preview.conflictCategories.length }} 个</li>
                </ul>
              </div>
            </div>
          </el-alert>

          <div class="strategy-selector">
            <h4 class="section-title">冲突处理策略</h4>
            <el-radio-group v-model="importStrategy">
              <el-radio value="skip">
                <div class="strategy-option">
                  <span class="strategy-name">跳过已存在</span>
                  <span class="strategy-desc">遇到同名文件时保留现有数据，跳过导入</span>
                </div>
              </el-radio>
              <el-radio value="overwrite">
                <div class="strategy-option">
                  <span class="strategy-name">覆盖同名</span>
                  <span class="strategy-desc">遇到同名文件时用导入数据替换现有数据</span>
                </div>
              </el-radio>
            </el-radio-group>
          </div>
        </div>
      </div>

      <!-- 步骤3：结果 -->
      <div v-if="step === 'result'" class="import-step">
        <div class="result-container">
          <div class="result-icon success">
            <svg viewBox="0 0 48 48" fill="none" stroke="currentColor" stroke-width="3">
              <circle cx="24" cy="24" r="20"/>
              <polyline points="16,24 22,30 32,18"/>
            </svg>
          </div>
          <h3 class="result-title">导入完成</h3>
          <div class="result-stats">
            <div class="result-stat">
              <span class="result-number">{{ importResult?.categoriesImported || 0 }}</span>
              <span class="result-label">新增分类</span>
            </div>
            <div class="result-stat">
              <span class="result-number">{{ importResult?.filesImported || 0 }}</span>
              <span class="result-label">导入文件</span>
            </div>
            <div class="result-stat">
              <span class="result-number">{{ importResult?.tagsImported || 0 }}</span>
              <span class="result-label">新增标签</span>
            </div>
            <div v-if="importResult?.filesSkipped" class="result-stat">
              <span class="result-number">{{ importResult.filesSkipped }}</span>
              <span class="result-label">跳过文件</span>
            </div>
            <div v-if="importResult?.filesOverwritten" class="result-stat">
              <span class="result-number">{{ importResult.filesOverwritten }}</span>
              <span class="result-label">覆盖文件</span>
            </div>
          </div>
          <div v-if="importResult?.errors?.length" class="result-errors">
            <el-alert type="warning" :closable="false">
              <template #title>部分文件导入出错</template>
              <ul>
                <li v-for="(err, i) in importResult.errors" :key="i">{{ err }}</li>
              </ul>
            </el-alert>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <template v-if="step === 'select'">
        <el-button @click="handleClose">取消</el-button>
      </template>
      <template v-if="step === 'preview'">
        <el-button @click="step = 'select'">返回</el-button>
        <el-button type="primary" :loading="importing" @click="handleImport">
          {{ importing ? '导入中...' : '开始导入' }}
        </el-button>
      </template>
      <template v-if="step === 'result'">
        <el-button type="primary" @click="handleDone">完成</el-button>
      </template>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { open } from '@tauri-apps/plugin-dialog'
import { previewImport, importData } from '@/api/file'

interface Props {
  modelValue: boolean
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'success'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const step = ref<'select' | 'preview' | 'result'>('select')
const importPath = ref('')
const preview = ref<any>(null)
const importResult = ref<any>(null)
const importing = ref(false)
const importStrategy = ref('skip')

const hasConflicts = computed(() => {
  return (preview.value?.conflictFiles?.length > 0) || (preview.value?.conflictCategories?.length > 0)
})

const getFileName = (path: string) => {
  return path.split('/').pop() || path.split('\\').pop() || path
}

const formatFileSize = (size: number) => {
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB'
  if (size < 1024 * 1024 * 1024) return (size / 1024 / 1024).toFixed(1) + ' MB'
  return (size / 1024 / 1024 / 1024).toFixed(1) + ' GB'
}

const selectFile = async () => {
  try {
    const result = await open({
      multiple: false,
      filters: [{
        name: 'DocHub 数据包',
        extensions: ['dochub']
      }]
    })

    if (!result) return

    const path = typeof result === 'string' ? result : result.path
    importPath.value = path

    // 预览
    const res = await previewImport(path)
    if (res.code === 200) {
      preview.value = res.data
      step.value = 'preview'
    }
  } catch (error: any) {
    ElMessage.error('预览失败: ' + (error.message || error))
  }
}

const handleImport = async () => {
  importing.value = true
  try {
    const res = await importData({
      importPath: importPath.value,
      strategy: importStrategy.value,
    })

    if (res.code === 200) {
      importResult.value = res.data
      step.value = 'result'
    }
  } catch (error: any) {
    ElMessage.error('导入失败: ' + (error.message || error))
  } finally {
    importing.value = false
  }
}

const handleDone = () => {
  emit('success')
  handleClose()
}

const handleClose = () => {
  dialogVisible.value = false
  step.value = 'select'
  importPath.value = ''
  preview.value = null
  importResult.value = null
  importStrategy.value = 'skip'
}

watch(dialogVisible, (visible) => {
  if (visible) {
    step.value = 'select'
  }
})
</script>

<style scoped>
.import-dialog {
  min-height: 200px;
}

.file-drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  border: 2px dashed var(--el-border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
}

.file-drop-zone:hover {
  border-color: var(--el-color-primary);
  background: var(--el-color-primary-light-9);
}

.drop-icon {
  width: 48px;
  height: 48px;
  color: var(--el-text-color-secondary);
  margin-bottom: 12px;
}

.drop-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  margin: 0 0 4px 0;
}

.drop-hint {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  margin: 0;
}

.preview-header {
  margin-bottom: 16px;
}

.file-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--el-fill-color-lighter);
  border-radius: 8px;
}

.file-name {
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.preview-stats {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-bottom: 16px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 8px;
  background: var(--el-fill-color-lighter);
  border-radius: 10px;
}

.stat-number {
  font-size: 20px;
  font-weight: 700;
  color: var(--el-color-primary);
}

.stat-label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-top: 4px;
}

.preview-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-bottom: 16px;
}

.section-title {
  margin: 0 0 8px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.category-preview-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 16px;
}

.category-preview-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: var(--el-fill-color-light);
  border-radius: 16px;
  font-size: 13px;
}

.cat-name {
  color: var(--el-text-color-primary);
}

.cat-count {
  color: var(--el-text-color-secondary);
  font-size: 12px;
}

.conflict-section {
  margin-top: 16px;
}

.conflict-details {
  margin-top: 8px;
}

.conflict-group {
  margin-bottom: 8px;
}

.conflict-group p {
  margin: 0 0 4px 0;
  font-weight: 500;
}

.conflict-group ul {
  margin: 0;
  padding-left: 20px;
}

.conflict-group li {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.strategy-selector {
  margin-top: 16px;
}

.strategy-selector .el-radio-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.strategy-option {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.strategy-name {
  font-size: 14px;
  font-weight: 500;
}

.strategy-desc {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.result-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px 0;
}

.result-icon {
  width: 64px;
  height: 64px;
  margin-bottom: 16px;
}

.result-icon.success {
  color: var(--el-color-success);
}

.result-title {
  margin: 0 0 20px 0;
  font-size: 18px;
  color: var(--el-text-color-primary);
}

.result-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  justify-content: center;
  margin-bottom: 16px;
}

.result-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 80px;
  padding: 12px 16px;
  background: var(--el-fill-color-lighter);
  border-radius: 10px;
}

.result-number {
  font-size: 24px;
  font-weight: 700;
  color: var(--el-color-primary);
}

.result-label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-top: 4px;
}

.result-errors {
  width: 100%;
  margin-top: 16px;
}

.result-errors ul {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.result-errors li {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
</style>
