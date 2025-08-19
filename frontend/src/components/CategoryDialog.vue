<template>
  <el-dialog
    v-model="dialogVisible"
    :title="props.editCategory ? '编辑分类' : '新建分类'"
    width="500px"
    :before-close="handleClose"
  >
    <el-form
      ref="categoryFormRef"
      :model="categoryForm"
      :rules="categoryRules"
      label-width="100px"
    >
      <el-form-item label="分类名称" prop="name">
        <el-input
          v-model="categoryForm.name"
          placeholder="请输入分类名称"
          clearable
        />
      </el-form-item>

      <el-form-item label="父分类">
        <el-cascader
          v-model="categoryForm.parentId"
          :options="categoryOptions"
          :props="cascaderProps"
          placeholder="请选择父分类（不选则为顶级分类）"
          clearable
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="分类描述">
        <el-input
          v-model="categoryForm.description"
          type="textarea"
          :rows="3"
          placeholder="请输入分类描述"
        />
      </el-form-item>

      <el-form-item label="排序">
        <el-input-number
          v-model="categoryForm.sortOrder"
          :min="0"
          :max="999"
          placeholder="数字越小排序越靠前"
          style="width: 100%"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" :loading="saving" @click="handleSave">
          {{ saving ? '保存中...' : (props.editCategory ? '更新' : '确定') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { createCategory, updateCategory } from '@/api/file'
import type { FileCategory, CategoryForm } from '@/types/file'

interface Props {
  modelValue: boolean
  categories: FileCategory[]
  editCategory?: FileCategory | null
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'success'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const categoryFormRef = ref<FormInstance>()
const saving = ref(false)

const categoryForm = reactive<CategoryForm & { sortOrder: number }>({
  name: '',
  description: '',
  parentId: undefined,
  sortOrder: 0
})

const categoryRules: FormRules = {
  name: [
    { required: true, message: '请输入分类名称', trigger: 'blur' },
    { min: 1, max: 50, message: '分类名称长度在1-50个字符', trigger: 'blur' }
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
  if (!categoryFormRef.value) return

  await categoryFormRef.value.validate(async (valid) => {
    if (valid) {
      saving.value = true
      try {
        const requestData = {
          name: categoryForm.name,
          description: categoryForm.description || undefined,
          parentId: categoryForm.parentId || undefined,
          sortOrder: categoryForm.sortOrder
        }

        let response
        if (props.editCategory) {
          // 编辑模式
          response = await updateCategory(props.editCategory.id!, {
            ...requestData,
            enabled: true // 默认保持启用状态
          })
          if (response.code === 200) {
            ElMessage.success('分类更新成功')
          } else {
            ElMessage.error(response.message || '更新失败')
          }
        } else {
          // 创建模式
          response = await createCategory(requestData)
          if (response.code === 200) {
            ElMessage.success('分类创建成功')
          } else {
            ElMessage.error(response.message || '创建失败')
          }
        }
        
        if (response.code === 200) {
          emit('success')
          resetForm()
        }
      } catch (error: any) {
        const action = props.editCategory ? '更新' : '创建'
        ElMessage.error(error.message || `${action}失败`)
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
  Object.assign(categoryForm, {
    name: '',
    description: '',
    parentId: undefined,
    sortOrder: 0
  })
  categoryFormRef.value?.resetFields()
}

// 监听编辑分类变化，自动填充表单
watch(() => props.editCategory, (newCategory) => {
  if (newCategory) {
    Object.assign(categoryForm, {
      name: newCategory.name || '',
      description: newCategory.description || '',
      parentId: newCategory.parentId,
      sortOrder: newCategory.sortOrder || 0
    })
  } else {
    resetForm()
  }
}, { immediate: true })
</script>

<style scoped>
.dialog-footer {
  text-align: right;
}
</style>
