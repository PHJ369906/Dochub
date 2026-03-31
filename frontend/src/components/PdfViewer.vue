<template>
  <div class="pdf-viewer">
    <div class="pdf-toolbar">
      <el-button-group>
        <el-button size="small" @click="prevPage" :disabled="currentPage <= 1">‹ 上一页</el-button>
        <el-button size="small" @click="nextPage" :disabled="currentPage >= totalPages">下一页 ›</el-button>
      </el-button-group>
      <el-input-number
        v-model="jumpPage"
        :min="1"
        :max="totalPages"
        :controls="false"
        size="small"
        class="pdf-jump-input"
        @change="handleJump"
        @keyup.enter="handleJump"
      />
      <span class="pdf-page-info">/ {{ totalPages }}</span>
      <el-button-group>
        <el-button size="small" @click="zoomOut" :disabled="scale <= 0.5">－</el-button>
        <el-button size="small" @click="zoomIn" :disabled="scale >= 3">＋</el-button>
      </el-button-group>
      <span class="pdf-scale-info">{{ Math.round(scale * 100) }}%</span>
    </div>
    <div class="pdf-canvas-area">
      <div v-if="error" class="pdf-error">
        <p>{{ error }}</p>
        <el-button size="small" @click="emit('open-external')">用系统应用打开</el-button>
      </div>
      <div v-else-if="loading" class="pdf-loading">
        <el-icon class="is-loading"><Loading /></el-icon>
        <span>PDF 加载中...</span>
      </div>
      <canvas v-show="!loading && !error" ref="canvasRef" class="pdf-canvas" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { Loading } from '@element-plus/icons-vue'
import * as pdfjsLib from 'pdfjs-dist'

pdfjsLib.GlobalWorkerOptions.workerSrc = '/pdf.worker.min.mjs'

const props = defineProps<{
  base64: string
}>()

const emit = defineEmits<{
  (e: 'open-external'): void
}>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const currentPage = ref(1)
const totalPages = ref(0)
const jumpPage = ref(1)
const scale = ref(1.2)
const loading = ref(true)
const error = ref('')

let pdfDoc: any = null
let renderTask: any = null

// onMounted 时 canvas 一定已在 DOM 中，无时序问题
onMounted(async () => {
  await initPdf()
})

onUnmounted(() => {
  renderTask?.cancel()
  pdfDoc?.destroy()
  pdfDoc = null
})

const initPdf = async () => {
  loading.value = true
  error.value = ''
  try {
    const binary = atob(props.base64)
    const bytes = new Uint8Array(binary.length)
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i)
    }
    pdfDoc = await pdfjsLib.getDocument({ data: bytes }).promise
    totalPages.value = pdfDoc.numPages
    loading.value = false
    await renderPage(1)
  } catch (e: any) {
    loading.value = false
    error.value = `PDF 加载失败：${e?.message || '未知错误'}`
  }
}

const renderPage = async (pageNum: number) => {
  if (!pdfDoc || !canvasRef.value) return

  // 取消上一个正在进行的渲染
  renderTask?.cancel()

  try {
    const page = await pdfDoc.getPage(pageNum)
    const dpr = window.devicePixelRatio || 1
    // 用 DPR 放大视口，使 canvas 物理像素与屏幕像素对齐（消除模糊）
    const viewport = page.getViewport({ scale: scale.value * dpr })
    const canvas = canvasRef.value
    const ctx = canvas.getContext('2d')
    if (!ctx) {
      error.value = '无法获取 Canvas 2D 上下文'
      return
    }
    // canvas 实际像素 = 视口尺寸（已乘 DPR）
    canvas.width = viewport.width
    canvas.height = viewport.height
    // CSS 显示尺寸缩回逻辑像素，保持布局不变
    canvas.style.width = `${viewport.width / dpr}px`
    canvas.style.height = `${viewport.height / dpr}px`
    renderTask = page.render({ canvasContext: ctx, viewport })
    await renderTask.promise
    currentPage.value = pageNum
    jumpPage.value = pageNum
  } catch (e: any) {
    if (e?.name !== 'RenderingCancelledException') {
      error.value = `渲染失败：${e?.message || '未知错误'}`
    }
  }
}

const prevPage = () => {
  if (currentPage.value > 1) renderPage(currentPage.value - 1)
}

const nextPage = () => {
  if (currentPage.value < totalPages.value) renderPage(currentPage.value + 1)
}

const handleJump = () => {
  const page = Math.round(jumpPage.value)
  if (page >= 1 && page <= totalPages.value && page !== currentPage.value) {
    renderPage(page)
  } else {
    jumpPage.value = currentPage.value
  }
}

const zoomIn = () => {
  scale.value = Math.min(3, parseFloat((scale.value + 0.2).toFixed(1)))
  renderPage(currentPage.value)
}

const zoomOut = () => {
  scale.value = Math.max(0.5, parseFloat((scale.value - 0.2).toFixed(1)))
  renderPage(currentPage.value)
}
</script>

<style scoped>
.pdf-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.pdf-toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  background: #f5f7fa;
  border-bottom: 1px solid #e6e6e6;
  flex-shrink: 0;
}

.pdf-jump-input {
  width: 56px;
}

.pdf-jump-input :deep(.el-input__inner) {
  text-align: center;
  padding: 0 4px;
}

.pdf-page-info,
.pdf-scale-info {
  font-size: 13px;
  color: #606266;
  min-width: 32px;
}

.pdf-canvas-area {
  flex: 1;
  overflow: auto;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 16px;
  background: #525659;
  min-height: 460px;
}

.pdf-canvas {
  display: block;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.4);
}

.pdf-loading,
.pdf-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: #ccc;
  min-height: 400px;
  font-size: 14px;
}

.pdf-loading .el-icon {
  font-size: 32px;
}

.pdf-error {
  color: #f56c6c;
}
</style>
