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
      <el-form-item label="选择文件" prop="files" required>
        <el-upload
          ref="uploadRef"
          :file-list="fileList"
          :auto-upload="false"
          :multiple="true"
          :limit="10"
          :on-change="handleFileChange"
          :on-remove="handleFileRemove"
          :before-upload="beforeUpload"
          drag
        >
          <el-icon class="el-icon--upload"><upload-filled /></el-icon>
          <div class="el-upload__text">
            将文件拖到此处，或<em>点击上传</em>
          </div>
          <template #tip>
            <div class="el-upload__tip">
              支持 PDF、Word、Excel、PPT、图片、视频等格式，单个文件不超过100MB
            </div>
          </template>
        </el-upload>
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
import { ElMessage, type FormInstance, type FormRules, type UploadFile, type UploadFiles } from 'element-plus'
import { UploadFilled } from '@element-plus/icons-vue'
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
const uploadRef = ref()
const uploading = ref(false)
const fileList = ref<UploadFile[]>([])

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

const uploadRules: FormRules = {
  files: [
    { required: true, message: '请选择要上传的文件', trigger: 'change' }
  ]
}

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

  // 先创建所有节点
  categories.forEach(category => {
    map.set(category.id, {
      id: category.id,
      name: category.name,
      children: []
    })
  })

  // 构建树形结构
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

const handleFileChange = (file: UploadFile, files: UploadFiles) => {
  fileList.value = files
  
  // 如果只有一个文件且没有设置标题，使用文件名作为标题
  if (files.length === 1 && !uploadForm.title) {
    uploadForm.title = file.name.substring(0, file.name.lastIndexOf('.'))
  }
}

const handleFileRemove = (file: UploadFile, files: UploadFiles) => {
  fileList.value = files
}

const beforeUpload = (file: File) => {
  const allowedTypes = [
    'pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx',
    'txt', 'md', 'html', 'htm', 'xml', 'json',
    'jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp',
    'mp4', 'avi', 'mov', 'wmv', 'flv', 'mkv',
    'mp3', 'wav', 'flac', 'aac', 'ogg', 'zip', 'rar', '7z'
  ]
  
  const fileType = file.name.split('.').pop()?.toLowerCase()
  if (!fileType || !allowedTypes.includes(fileType)) {
    ElMessage.error('不支持的文件类型')
    return false
  }
  
  const maxSize = 100 * 1024 * 1024 // 100MB
  if (file.size > maxSize) {
    ElMessage.error('文件大小不能超过100MB')
    return false
  }
  
  return true
}

const handleUpload = async () => {
  if (!uploadFormRef.value) return
  
  if (fileList.value.length === 0) {
    ElMessage.error('请选择要上传的文件')
    return
  }
  
  uploading.value = true
  
  try {
    // 逐个上传文件
    for (const fileItem of fileList.value) {
      if (fileItem.raw) {
        const formData = new FormData()
        formData.append('file', fileItem.raw)
        
        // 添加其他表单数据
        if (uploadForm.title) formData.append('title', uploadForm.title)
        if (uploadForm.description) formData.append('description', uploadForm.description)
        if (uploadForm.categoryId) formData.append('categoryId', uploadForm.categoryId.toString())
        if (uploadForm.tags && uploadForm.tags.length > 0) {
          uploadForm.tags.forEach(tag => formData.append('tags', tag))
        }
        if (uploadForm.documentNumber) formData.append('documentNumber', uploadForm.documentNumber)
        if (uploadForm.issueDate) formData.append('issueDate', uploadForm.issueDate)
        if (uploadForm.effectiveDate) formData.append('effectiveDate', uploadForm.effectiveDate)
        if (uploadForm.issuingAuthority) formData.append('issuingAuthority', uploadForm.issuingAuthority)
        formData.append('isPublic', uploadForm.isPublic.toString())
        
        const response = await uploadFile(formData)
        if (response.code !== 200) {
          throw new Error(response.message || '上传失败')
        }
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
  fileList.value = []
  uploadRef.value?.clearFiles()
}

// 监听对话框显示状态
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

:deep(.el-upload-dragger) {
  width: 100%;
}
</style>
