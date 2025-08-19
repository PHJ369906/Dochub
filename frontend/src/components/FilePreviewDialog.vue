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
        <div v-if="previewType === 'image'" class="image-preview">
          <el-image
            :src="previewUrl"
            :alt="file.title"
            fit="contain"
            style="width: 100%; height: 500px;"
            :preview-src-list="[previewUrl]"
          />
        </div>

        <!-- 视频预览 -->
        <div v-else-if="previewType === 'video'" class="video-preview">
          <video
            :src="previewUrl"
            controls
            style="width: 100%; height: 500px;"
          >
            您的浏览器不支持视频播放
          </video>
        </div>

        <!-- 音频预览 -->
        <div v-else-if="previewType === 'audio'" class="audio-preview">
          <audio
            :src="previewUrl"
            controls
            style="width: 100%;"
          >
            您的浏览器不支持音频播放
          </audio>
        </div>

        <!-- PDF预览 -->
        <div v-else-if="previewType === 'pdf'" class="pdf-preview">
          <iframe
            :src="pdfPreviewUrl"
            style="width: 100%; height: 600px; border: none;"
          />
        </div>

        <!-- Office文档预览 -->
        <div v-else-if="previewType === 'office'" class="office-preview">
          <!-- 内嵌预览框架 -->
          <iframe
            :src="onlinePreviewUrl"
            style="width: 100%; height: 600px; border: none;"
            @load="iframeLoaded"
          />
        </div>

        <!-- 文本预览 -->
        <div v-else-if="previewType === 'text'" class="text-preview">
          <pre v-if="textContent">{{ textContent }}</pre>
          <div v-else class="no-preview">
            <el-icon class="no-preview-icon"><Document /></el-icon>
            <p>文本内容加载失败</p>
          </div>
        </div>

        <!-- 不支持预览 -->
        <div v-else class="no-preview">
          <el-icon class="no-preview-icon"><Document /></el-icon>
          <p>该文件类型暂不支持在线预览</p>
          <el-button type="primary" @click="downloadFile">下载文件</el-button>
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
import { Document, Download, View } from '@element-plus/icons-vue'
import { useAuthStore } from '@/stores/auth'
import {
  getFilePreview,
  getPdfPagePreview,
  getOfficePreview,
  getImagePreview,
  getTextPreview
} from '@/api/file'
import type { PolicyFile, FilePreviewInfo } from '@/types/file'

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
const previewInfo = ref<FilePreviewInfo | null>(null)
const textContent = ref('')
const onlinePreviewUrl = ref('')

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const previewType = computed(() => {
  if (!props.file) return 'unknown'

  const fileType = props.file.fileType.toLowerCase()

  if (fileType === 'pdf') return 'pdf'
  if (['doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx'].includes(fileType)) return 'office'
  if (['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'].includes(fileType)) return 'image'
  if (['mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv', 'webm'].includes(fileType)) return 'video'
  if (['mp3', 'wav', 'flac', 'aac', 'ogg', 'm4a'].includes(fileType)) return 'audio'
  if (['txt', 'md', 'json', 'xml', 'html', 'htm', 'css', 'js', 'ts'].includes(fileType)) return 'text'

  return 'unknown'
})

const previewUrl = computed(() => {
  if (!props.file) return ''

  switch (previewType.value) {
    case 'image':
      return getImagePreview(props.file.id)
    case 'video':
    case 'audio':
      return `/api/files/${props.file.id}/preview`
    case 'text':
      return getTextPreview(props.file.id, false)
    default:
      return `/api/files/${props.file.id}/preview`
  }
})

const officePreviewUrl = computed(() => {
  if (!props.file || previewType.value !== 'office') return ''
  return getOfficePreview(props.file.id)
})

const pdfPreviewUrl = computed(() => {
  if (!props.file || previewType.value !== 'pdf') return ''
  return getPdfPagePreview(props.file.id, 0, 150)
})

const loadPreviewInfo = async () => {
  if (!props.file) return

  previewLoading.value = true
  try {
    const response = await getFilePreview(props.file.id)
    if (response.code === 200) {
      previewInfo.value = response.data

      // 根据文件类型加载相应内容
      if (previewType.value === 'text') {
        await loadTextContent()
      } else if (previewType.value === 'office') {
        // Office文档自动加载预览
        onlinePreviewUrl.value = `/api/preview/onlinePreview?url=${props.file.id}`
      }
    } else {
      ElMessage.error(response.message || '加载预览信息失败')
    }
  } catch (error) {
    console.error('加载预览信息失败:', error)
    ElMessage.error('加载预览信息失败')
  } finally {
    previewLoading.value = false
  }
}

const loadTextContent = async () => {
  if (!props.file) return

  try {
    const response = await fetch(getTextPreview(props.file.id, false))
    if (response.ok) {
      textContent.value = await response.text()
    }
  } catch (error) {
    console.error('加载文本内容失败:', error)
  }
}


const downloadFile = async () => {
  if (props.file) {
    try {
      // 使用store中的token来确保认证状态一致
      const authStore = useAuthStore()
      const response = await fetch(`/api/files/${props.file.id}/download`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${authStore.token}`
        }
      })

      if (!response.ok) {
        if (response.status === 401) {
          ElMessage.error('用户未登录，请重新登录')
        } else {
          ElMessage.error('下载失败：' + response.statusText)
        }
        return
      }

      // 获取文件内容
      const blob = await response.blob()
      
      // 创建下载链接
      const url = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = props.file.originalName
      document.body.appendChild(link)
      link.click()
      
      // 清理
      document.body.removeChild(link)
      window.URL.revokeObjectURL(url)
      
      ElMessage.success('文件下载成功')
    } catch (error) {
      console.error('下载失败:', error)
      ElMessage.error('下载失败，请稍后重试')
    }
  }
}


const iframeLoaded = () => {
  // iframe加载完成
  console.log('预览加载完成')
}

const handleClose = () => {
  emit('update:modelValue', false)
}

const formatFileSize = (size: number) => {
  if (size < 1024) {
    return size + ' B'
  } else if (size < 1024 * 1024) {
    return (size / 1024).toFixed(1) + ' KB'
  } else if (size < 1024 * 1024 * 1024) {
    return (size / (1024 * 1024)).toFixed(1) + ' MB'
  } else {
    return (size / (1024 * 1024 * 1024)).toFixed(1) + ' GB'
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

// 监听文件变化
watch(() => props.file, (newFile) => {
  if (newFile && props.modelValue) {
    loadPreviewInfo()
  }
}, { immediate: true })

// 监听对话框显示状态
watch(dialogVisible, (visible) => {
  if (visible && props.file) {
    loadPreviewInfo()
  } else {
    // 重置状态
    previewInfo.value = null
    textContent.value = ''
    onlinePreviewUrl.value = ''
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

.image-preview,
.video-preview,
.audio-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
}

.pdf-preview {
  display: flex;
  flex-direction: column;
}

.pdf-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background-color: #f8f9fa;
  border-bottom: 1px solid #e6e6e6;
}

.page-info {
  font-weight: 500;
  color: #606266;
}

.pdf-content {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
  overflow: auto;
}

.office-preview {
  height: 600px;
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

.no-preview {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 400px;
  color: #909399;
}

.no-preview-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.dialog-footer {
  text-align: right;
}
</style>
