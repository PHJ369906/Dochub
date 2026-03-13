<template>
  <el-dialog
    v-model="dialogVisible"
    title="上传文件"
    width="600px"
    :before-close="handleClose"
  >
    <el-form
      ref="uploadFormRef"
      :model="uploadForm"
      :rules="uploadRules"
      label-width="100px"
    >
      <el-form-item label="选择文件" required>
        <div class="file-select-area">
          <el-button type="primary" @click="selectFiles">
            <el-icon><upload-filled /></el-icon>
            选择文件
          </el-button>
          <div v-if="selectedFiles.length > 0" class="selected-files">
            <div v-for="(file, index) in selectedFiles" :key="index" class="file-item">
              <span class="file-name">{{ getFileName(file) }}</span>
              <el-button link size="small" @click="removeFile(index)">
                <el-icon><Close /></el-icon>
              </el-button>
            </div>
          </div>
          <div v-else class="no-file-tip">
            支持 PDF、Word、Excel、PPT、图片、视频等格式，单个文件不超过100MB
          </div>
        </div>
      </el-form-item>

      <el-form-item label="文件标题">
        <el-input
          v-model="uploadForm.title"
          placeholder="不填写则使用文件名"
          clearable
        />
      </el-form-item>

      <el-form-item label="文件描述">
        <el-input
          v-model="uploadForm.description"
          type="textarea"
          :rows="3"
          placeholder="请输入文件描述"
        />
      </el-form-item>

      <el-form-item label="文件分类">
        <el-cascader
          v-model="uploadForm.categoryId"
          :options="categoryOptions"
          :props="cascaderProps"
          placeholder="请选择文件分类"
          clearable
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="文件标签">
        <el-select
          v-model="uploadForm.tags"
          multiple
          filterable
          allow-create
          placeholder="请选择或输入标签"
          style="width: 100%"
        >
          <el-option
            v-for="tag in availableTags"
            :key="tag.value"
            :label="tag.label"
            :value="tag.value"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="文件编号">
        <el-input
          v-model="uploadForm.documentNumber"
          placeholder="请输入文件编号"
          clearable
        />
      </el-form-item>

      <el-form-item label="发布日期">
        <el-date-picker
          v-model="uploadForm.issueDate"
          type="date"
          placeholder="请选择发布日期"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="生效日期">
        <el-date-picker
          v-model="uploadForm.effectiveDate"
          type="date"
          placeholder="请选择生效日期"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="发布机关">
        <el-input
          v-model="uploadForm.issuingAuthority"
          placeholder="请输入发布机关"
          clearable
        />
      </el-form-item>

      <el-form-item label="公开状态">
        <el-radio-group v-model="uploadForm.isPublic">
          <el-radio :label="true">公开</el-radio>
          <el-radio :label="false">内部</el-radio>
        </el-radio-group>
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" :loading="uploading" @click="handleUpload">
          {{ uploading ? '上传中...' : '确定上传' }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { UploadFilled, Close } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { uploadFile } from '@/api/file'
import type { FileCategory, FileUploadForm } from '@/types/file'

interface Props {
  modelValue: boolean
  categories: FileCategory[]
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'success'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const uploadFormRef = ref<FormInstance>()
const uploading = ref(false)
const selectedFiles = ref<string[]>([])

const uploadForm = reactive<FileUploadForm & { isPublic: boolean }>({
  title: '',
  description: '',
  categoryId: undefined,
  tags: [],
  documentNumber: '',
  issueDate: '',
  effectiveDate: '',
  issuingAuthority: '',
  isPublic: true
})

const uploadRules: FormRules = {}

const availableTags = ref([
  { label: '政策法规', value: '政策法规' },
  { label: '通知公告', value: '通知公告' },
  { label: '规范性文件', value: '规范性文件' },
  { label: '重要文件', value: '重要文件' },
  { label: '临时文件', value: '临时文件' }
])

const cascaderProps = {
  value: 'id',
  label: 'name',
  children: 'children',
  emitPath: false
}

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const categoryOptions = computed(() => {
  return buildCategoryTree(props.categories)
})

const buildCategoryTree = (categories: FileCategory[]) => {
  const tree: any[] = []
  const map = new Map()

  categories.forEach(category => {
    map.set(category.id, {
      id: category.id,
      name: category.name,
      children: []
    })
  })

  categories.forEach(category => {
    const node = map.get(category.id)
    if (category.parentId && map.has(category.parentId)) {
      map.get(category.parentId).children.push(node)
    } else {
      tree.push(node)
    }
  })

  return tree
}

const getFileName = (filePath: string) => {
  return filePath.split('/').pop() || filePath.split('\\').pop() || filePath
}

const selectFiles = async () => {
  try {
    const result = await open({
      multiple: true,
      filters: [{
        name: '文档',
        extensions: [
          'pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx',
          'txt', 'md', 'html', 'htm', 'xml', 'json',
          'jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp',
          'mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv',
          'mp3', 'wav', 'flac', 'aac', 'ogg', 'zip', 'rar', '7z'
        ]
      }]
    })

    if (result) {
      const paths = Array.isArray(result) ? result : [result]
      selectedFiles.value = paths.map(p => typeof p === 'string' ? p : p.path)

      // 如果只有一个文件且没有设置标题，使用文件名作为标题
      if (selectedFiles.value.length === 1 && !uploadForm.title) {
        const fileName = getFileName(selectedFiles.value[0])
        uploadForm.title = fileName.substring(0, fileName.lastIndexOf('.')) || fileName
      }
    }
  } catch (error) {
    console.error('选择文件失败:', error)
  }
}

const removeFile = (index: number) => {
  selectedFiles.value.splice(index, 1)
}

const handleUpload = async () => {
  if (selectedFiles.value.length === 0) {
    ElMessage.error('请选择要上传的文件')
    return
  }

  uploading.value = true

  try {
    for (const filePath of selectedFiles.value) {
      const response = await uploadFile({
        filePath,
        title: uploadForm.title || undefined,
        description: uploadForm.description || undefined,
        categoryId: uploadForm.categoryId,
        tags: uploadForm.tags && uploadForm.tags.length > 0 ? uploadForm.tags : undefined,
        documentNumber: uploadForm.documentNumber || undefined,
        issueDate: uploadForm.issueDate || undefined,
        effectiveDate: uploadForm.effectiveDate || undefined,
        issuingAuthority: uploadForm.issuingAuthority || undefined,
        isPublic: uploadForm.isPublic,
      })
      if (response.code !== 200) {
        throw new Error(response.message || '上传失败')
      }
    }

    ElMessage.success('文件上传成功')
    emit('success')
    resetForm()
  } catch (error: any) {
    ElMessage.error(error.message || '上传失败')
  } finally {
    uploading.value = false
  }
}

const handleClose = () => {
  if (!uploading.value) {
    resetForm()
    emit('update:modelValue', false)
  }
}

const resetForm = () => {
  Object.assign(uploadForm, {
    title: '',
    description: '',
    categoryId: undefined,
    tags: [],
    documentNumber: '',
    issueDate: '',
    effectiveDate: '',
    issuingAuthority: '',
    isPublic: true
  })
  selectedFiles.value = []
}

watch(dialogVisible, (visible) => {
  if (!visible) {
    resetForm()
  }
})
</script>

<style scoped>
.dialog-footer {
  text-align: right;
}

.file-select-area {
  width: 100%;
}

.selected-files {
  margin-top: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-light);
}

.file-name {
  font-size: 14px;
  color: var(--neutral-700);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.no-file-tip {
  margin-top: 8px;
  font-size: 12px;
  color: var(--neutral-500);
}
</style>
