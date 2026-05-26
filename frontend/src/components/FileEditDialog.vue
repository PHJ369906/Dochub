<template>
  <el-dialog
    v-model="dialogVisible"
    title="编辑文档"
    width="600px"
    :before-close="handleClose"
    class="file-edit-dialog"
  >
    <el-form
      ref="fileFormRef"
      :model="fileForm"
      :rules="fileRules"
      label-width="100px"
      class="file-edit-form"
    >
      <el-form-item label="文档标题" prop="title">
        <el-input
          v-model="fileForm.title"
          placeholder="请输入文档标题"
          clearable
          maxlength="100"
          show-word-limit
        />
      </el-form-item>

      <el-form-item label="原始文件名" prop="originalName">
        <el-input
          v-model="fileForm.originalName"
          placeholder="请输入原始文件名（含扩展名）"
          clearable
          maxlength="200"
        />
      </el-form-item>

      <el-form-item label="文档分类" prop="categoryId">
        <el-cascader
          v-model="fileForm.categoryId"
          :options="categoryOptions"
          :props="cascaderProps"
          placeholder="请选择文档分类"
          clearable
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="发布机关">
        <el-input
          v-model="fileForm.issuingAuthority"
          placeholder="请输入发布机关"
          clearable
          maxlength="50"
        />
      </el-form-item>

      <el-form-item label="文档描述">
        <el-input
          v-model="fileForm.description"
          type="textarea"
          :rows="4"
          placeholder="请输入文档描述"
          maxlength="500"
          show-word-limit
        />
      </el-form-item>

      <el-form-item label="标签">
        <el-select
          v-model="fileForm.tags"
          multiple
          filterable
          allow-create
          placeholder="请选择或输入标签"
          style="width: 100%"
        >
          <el-option
            v-for="tag in availableTags"
            :key="tag"
            :label="tag"
            :value="tag"
          />
        </el-select>
      </el-form-item>


      <el-form-item label="是否公开" prop="isPublic">
        <el-switch
          v-model="fileForm.isPublic"
          active-text="公开"
          inactive-text="私有"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" :loading="saving" @click="handleSave">
          {{ saving ? '保存中...' : '保存' }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, nextTick } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { updateFile } from '@/api/file'
import type { PolicyFile, FileCategory } from '@/types/file'

interface Props {
  modelValue: boolean
  editFile?: PolicyFile | null
  categories: FileCategory[]
  availableTags?: string[]
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'success'): void
}

const props = withDefaults(defineProps<Props>(), {
  availableTags: () => []
})

const emit = defineEmits<Emits>()

const fileFormRef = ref<FormInstance>()
const saving = ref(false)

const fileForm = reactive({
  title: '',
  originalName: '',
  categoryId: undefined as number | undefined,
  issuingAuthority: '',
  description: '',
  tags: [] as string[],
  isPublic: true
})

const fileRules: FormRules = {
  title: [
    { required: true, message: '请输入文档标题', trigger: 'blur' },
    { min: 1, max: 100, message: '标题长度在1-100个字符', trigger: 'blur' }
  ],
  originalName: [
    { required: true, message: '请输入原始文件名', trigger: 'blur' },
    { min: 1, max: 200, message: '文件名长度在1-200个字符', trigger: 'blur' }
  ]
}

const cascaderProps = {
  value: 'id',
  label: 'name',
  children: 'children',
  emitPath: false,
  checkStrictly: true
}

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const categoryOptions = computed(() => {
  return buildCategoryTree(props.categories)
})

const buildCategoryTree = (categories: FileCategory[]) => {
  if (!categories || !Array.isArray(categories)) {
    return []
  }
  
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

const handleSave = async () => {
  if (!fileFormRef.value || !props.editFile) return

  await fileFormRef.value.validate(async (valid) => {
    if (valid) {
      saving.value = true
      try {
        const updateData = {
          title: fileForm.title,
          originalName: fileForm.originalName,
          categoryId: fileForm.categoryId,
          issuingAuthority: fileForm.issuingAuthority || undefined,
          description: fileForm.description || undefined,
          tags: fileForm.tags.length > 0 ? fileForm.tags : undefined,
          isPublic: fileForm.isPublic
        }

        const response = await updateFile(props.editFile.id!, updateData)
        if (response.code === 200) {
          ElMessage.success('文档更新成功')
          emit('success')
          resetForm()
        } else {
          ElMessage.error(response.message || '更新失败')
        }
      } catch (error: any) {
        ElMessage.error(error.message || '更新失败')
      } finally {
        saving.value = false
      }
    }
  })
}

const handleClose = () => {
  if (!saving.value) {
    resetForm()
    emit('update:modelValue', false)
  }
}

const resetForm = () => {
  Object.assign(fileForm, {
    title: '',
    originalName: '',
    categoryId: undefined,
    issuingAuthority: '',
    description: '',
    tags: [],
    isPublic: true
  })
  fileFormRef.value?.resetFields()
}

// 监听编辑文件变化，自动填充表单
watch(() => props.editFile, (newFile) => {
  if (newFile && props.modelValue) {
    nextTick(() => {
      Object.assign(fileForm, {
        title: newFile.title || '',
        originalName: newFile.originalName || '',
        categoryId: newFile.categoryId || undefined,
        issuingAuthority: newFile.issuingAuthority || '',
        description: newFile.description || '',
        tags: newFile.tags ? newFile.tags.map((t: any) => typeof t === 'string' ? t : t.name) : [],
        isPublic: newFile.isPublic !== false
      })
    })
  } else if (!newFile) {
    resetForm()
  }
})

// 监听对话框打开，填充表单
watch(() => props.modelValue, (visible) => {
  if (visible && props.editFile) {
    nextTick(() => {
      Object.assign(fileForm, {
        title: props.editFile!.title || '',
        originalName: props.editFile!.originalName || '',
        categoryId: props.editFile!.categoryId || undefined,
        issuingAuthority: props.editFile!.issuingAuthority || '',
        description: props.editFile!.description || '',
        tags: props.editFile!.tags ? props.editFile!.tags.map((t: any) => typeof t === 'string' ? t : t.name) : [],
        isPublic: props.editFile!.isPublic !== false
      })
    })
  }
})
</script>

<style scoped>
.file-edit-form {
  padding: 0 8px;
}

.file-edit-form .el-form-item {
  margin-bottom: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 16px;
  border-top: 1px solid var(--border-light);
}
</style>