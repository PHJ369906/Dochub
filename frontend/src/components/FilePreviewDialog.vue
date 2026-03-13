<template>
  <el-dialog
    v-model="dialogVisible"
    :title="file?.title || '文件预览'"
    width="90%"
    :before-close="handleClose"
    class="preview-dialog"
  >
    <div v-if="file" class="preview-container">
      <!-- 文件信息 -->
      <div class="file-info">
        <div class="info-item">
          <span class="label">文件名：</span>
          <span class="value">{{ file.originalName }}</span>
        </div>
        <div class="info-item">
          <span class="label">文件大小：</span>
          <span class="value">{{ formatFileSize(file.fileSize) }}</span>
        </div>
        <div class="info-item">
          <span class="label">文件类型：</span>
          <span class="value">{{ file.fileType.toUpperCase() }}</span>
        </div>
        <div class="info-item">
          <span class="label">上传时间：</span>
          <span class="value">{{ formatDate(file.createTime) }}</span>
        </div>
        <div v-if="file.issuingAuthority" class="info-item">
          <span class="label">发布机关：</span>
          <span class="value">{{ file.issuingAuthority }}</span>
        </div>
      </div>

      <!-- 预览内容 -->
      <div class="preview-content" v-loading="previewLoading">
        <!-- 图片预览 -->
        <div v-if="previewType === 'image' && previewData" class="image-preview">
          <el-image
            :src="`data:${previewData.mimeType};base64,${previewData.content}`"
            :alt="file.title"
            fit="contain"
            style="width: 100%; height: 500px;"
            :preview-src-list="[`data:${previewData.mimeType};base64,${previewData.content}`]"
          />
        </div>

        <!-- PDF预览 -->
        <div v-else-if="previewType === 'pdf' && previewData" class="pdf-preview">
          <iframe
            :src="`data:application/pdf;base64,${previewData.content}`"
            style="width: 100%; height: 600px; border: none;"
          />
        </div>

        <!-- 文本预览 -->
        <div v-else-if="previewType === 'text' && previewData" class="text-preview">
          <pre>{{ previewData.content }}</pre>
        </div>

        <!-- HTML 预览（Word/Excel 文档） -->
        <div v-else-if="previewType === 'html' && previewData" class="html-preview">
          <div v-html="previewData.content" class="html-content"></div>
        </div>

        <!-- 视频预览 -->
        <div v-else-if="previewType === 'video' && previewData" class="video-preview">
          <video controls preload="metadata" :key="assetUrl">
            <source :src="assetUrl" :type="previewData.mimeType" />
            您的浏览器不支持视频播放
          </video>
        </div>

        <!-- 音频预览 -->
        <div v-else-if="previewType === 'audio' && previewData" class="audio-preview">
          <audio controls preload="metadata" :key="assetUrl">
            <source :src="assetUrl" :type="previewData.mimeType" />
            您的浏览器不支持音频播放
          </audio>
        </div>

        <!-- Office/其他 - 用系统应用打开 -->
        <div v-else-if="previewType === 'office' || previewType === 'external'" class="no-preview">
          <el-icon class="no-preview-icon"><Document /></el-icon>
          <p>{{ previewType === 'office' ? 'Office 文档请使用系统默认应用打开' : '该文件类型暂不支持在线预览' }}</p>
          <el-button type="primary" @click="openWithSystemApp">用系统应用打开</el-button>
          <el-button @click="openFileFolder">打开文件所在目录</el-button>
        </div>

        <!-- 不支持预览 -->
        <div v-else-if="!previewLoading" class="no-preview">
          <el-icon class="no-preview-icon"><Document /></el-icon>
          <p>该文件类型暂不支持在线预览</p>
          <el-button type="primary" @click="openWithSystemApp">用系统应用打开</el-button>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">关闭</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { Document } from '@element-plus/icons-vue'
import { open as shellOpen } from '@tauri-apps/plugin-shell'
import { convertFileSrc } from '@tauri-apps/api/core'
import { previewFile as apiPreviewFile, getFilePath, openFileWithSystem, openFileFolder as apiOpenFileFolder } from '@/api/file'
import type { PolicyFile } from '@/types/file'

interface Props {
  modelValue: boolean
  file: PolicyFile | null
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const previewLoading = ref(false)
const previewData = ref<{ previewType: string; content: string; mimeType: string } | null>(null)
const filePath = ref('')

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const previewType = computed(() => {
  return previewData.value?.previewType || 'unknown'
})

const assetUrl = computed(() => {
  if ((previewType.value === 'video' || previewType.value === 'audio') && previewData.value?.content) {
    return convertFileSrc(previewData.value.content)
  }
  return ''
})

const loadPreview = async () => {
  if (!props.file) return

  previewLoading.value = true
  try {
    const response = await apiPreviewFile(props.file.id)
    if (response.code === 200) {
      previewData.value = response.data
    }

    // 获取文件路径用于系统打开
    const pathResp = await getFilePath(props.file.id)
    if (pathResp.code === 200) {
      filePath.value = pathResp.data
    }
  } catch (error) {
    console.error('加载预览失败:', error)
  } finally {
    previewLoading.value = false
  }
}

const openWithSystemApp = async () => {
  if (props.file) {
    try {
      await openFileWithSystem(props.file.id)
    } catch (error) {
      ElMessage.error('无法打开文件')
    }
  }
}

const openFileFolder = async () => {
  if (props.file) {
    try {
      await apiOpenFileFolder(props.file.id)
    } catch (error) {
      ElMessage.error('无法打开目录')
    }
  }
}

const handleClose = () => {
  emit('update:modelValue', false)
}

const formatFileSize = (size: number) => {
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB'
  if (size < 1024 * 1024 * 1024) return (size / (1024 * 1024)).toFixed(1) + ' MB'
  return (size / (1024 * 1024 * 1024)).toFixed(1) + ' GB'
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

watch(() => props.file, (newFile) => {
  if (newFile && props.modelValue) {
    loadPreview()
  }
}, { immediate: true })

watch(dialogVisible, (visible) => {
  if (visible && props.file) {
    loadPreview()
  } else {
    previewData.value = null
    filePath.value = ''
  }
})
</script>

<style scoped>
.preview-dialog {
  :deep(.el-dialog__body) {
    padding: 20px;
    max-height: 80vh;
    overflow-y: auto;
  }
}

.preview-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.file-info {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  padding: 16px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.info-item {
  display: flex;
  align-items: center;
}

.label {
  font-weight: 500;
  color: #606266;
  margin-right: 8px;
}

.value {
  color: #303133;
}

.preview-content {
  min-height: 400px;
  border: 1px solid #e6e6e6;
  border-radius: 4px;
  overflow: hidden;
}

.image-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
}

.pdf-preview {
  display: flex;
  flex-direction: column;
}

.text-preview {
  padding: 20px;
  background-color: #f8f9fa;
  height: 500px;
  overflow: auto;
}

.text-preview pre {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
}

.html-preview {
  padding: 20px;
  max-height: 600px;
  overflow-y: auto;
  background-color: #fff;
}

.html-content {
  max-width: 100%;
}

.html-content :deep(p) {
  margin: 0 0 8px 0;
  line-height: 1.8;
}

.html-content :deep(h1),
.html-content :deep(h2),
.html-content :deep(h3),
.html-content :deep(h4),
.html-content :deep(h5),
.html-content :deep(h6) {
  margin-top: 16px;
  margin-bottom: 8px;
}

.html-content :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin: 12px 0;
}

.html-content :deep(td),
.html-content :deep(th) {
  border: 1px solid #ccc;
  padding: 6px 10px;
  text-align: left;
  vertical-align: top;
}

.html-content :deep(img) {
  max-width: 100%;
  height: auto;
}

.video-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
  background-color: #000;
}

.video-preview video {
  max-width: 100%;
  max-height: 600px;
}

.audio-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 60px 20px;
}

.audio-preview audio {
  width: 80%;
  max-width: 500px;
}

.no-preview {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 400px;
  color: #909399;
  gap: 16px;
}

.no-preview-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.dialog-footer {
  text-align: right;
}
</style>
