<template>
  <div class="advanced-search">
    <el-form :model="searchForm" label-width="100px" @submit.prevent="handleSearch">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="关键词">
            <el-input
              v-model="searchForm.keyword"
              placeholder="搜索文件标题、描述"
              clearable
            />
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="文件分类">
            <el-cascader
              v-model="searchForm.categoryId"
              :options="categoryOptions"
              :props="cascaderProps"
              placeholder="请选择分类"
              clearable
              style="width: 100%"
            />
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="文件类型">
            <el-select
              v-model="searchForm.fileTypes"
              multiple
              placeholder="选择文件类型"
              style="width: 100%"
            >
              <el-option-group label="文档类型">
                <el-option label="PDF" value="pdf" />
                <el-option label="Word" value="doc,docx" />
                <el-option label="Excel" value="xls,xlsx" />
                <el-option label="PowerPoint" value="ppt,pptx" />
              </el-option-group>
              <el-option-group label="图片类型">
                <el-option label="JPG" value="jpg,jpeg" />
                <el-option label="PNG" value="png" />
                <el-option label="GIF" value="gif" />
              </el-option-group>
              <el-option-group label="其他类型">
                <el-option label="视频" value="mp4,avi,mov" />
                <el-option label="音频" value="mp3,wav,flac" />
                <el-option label="压缩包" value="zip,rar,7z" />
              </el-option-group>
            </el-select>
          </el-form-item>
        </el-col>
      </el-row>

      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="文件标签">
            <el-select
              v-model="searchForm.tags"
              multiple
              filterable
              placeholder="选择标签"
              style="width: 100%"
            >
              <el-option
                v-for="tag in availableTags"
                :key="tag.id"
                :label="tag.name"
                :value="tag.name"
              />
            </el-select>
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="发布机关">
            <el-input
              v-model="searchForm.issuingAuthority"
              placeholder="请输入发布机关"
              clearable
            />
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="公开状态">
            <el-select
              v-model="searchForm.isPublic"
              placeholder="选择公开状态"
              clearable
              style="width: 100%"
            >
              <el-option label="公开" :value="true" />
              <el-option label="内部" :value="false" />
            </el-select>
          </el-form-item>
        </el-col>
      </el-row>

      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="上传时间">
            <el-date-picker
              v-model="uploadDateRange"
              type="daterange"
              range-separator="至"
              start-placeholder="开始日期"
              end-placeholder="结束日期"
              style="width: 100%"
              @change="handleUploadDateChange"
            />
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="发布时间">
            <el-date-picker
              v-model="issueDateRange"
              type="daterange"
              range-separator="至"
              start-placeholder="开始日期"
              end-placeholder="结束日期"
              style="width: 100%"
              @change="handleIssueDateChange"
            />
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="文件大小">
            <div class="size-range">
              <el-input-number
                v-model="searchForm.minSize"
                :min="0"
                placeholder="最小"
                style="width: 45%"
              />
              <span style="margin: 0 10px">-</span>
              <el-input-number
                v-model="searchForm.maxSize"
                :min="0"
                placeholder="最大"
                style="width: 45%"
              />
            </div>
            <div class="size-unit">
              <el-radio-group v-model="sizeUnit" size="small">
                <el-radio-button label="KB">KB</el-radio-button>
                <el-radio-button label="MB">MB</el-radio-button>
                <el-radio-button label="GB">GB</el-radio-button>
              </el-radio-group>
            </div>
          </el-form-item>
        </el-col>
      </el-row>

      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="排序方式">
            <el-select
              v-model="searchForm.sort"
              placeholder="选择排序字段"
              style="width: 100%"
            >
              <el-option label="上传时间" value="createTime" />
              <el-option label="文件名称" value="title" />
              <el-option label="文件大小" value="fileSize" />
              <el-option label="查看次数" value="viewCount" />
              <el-option label="下载次数" value="downloadCount" />
              <el-option label="发布时间" value="issueDate" />
            </el-select>
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="排序方向">
            <el-radio-group v-model="searchForm.direction">
              <el-radio label="desc">降序</el-radio>
              <el-radio label="asc">升序</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-col>
        
        <el-col :span="8">
          <el-form-item label="每页显示">
            <el-select
              v-model="searchForm.size"
              style="width: 100%"
            >
              <el-option label="10条" :value="10" />
              <el-option label="20条" :value="20" />
              <el-option label="50条" :value="50" />
              <el-option label="100条" :value="100" />
            </el-select>
          </el-form-item>
        </el-col>
      </el-row>

      <el-row>
        <el-col :span="24">
          <el-form-item>
            <div class="search-actions">
              <el-button type="primary" @click="handleSearch">
                <el-icon><Search /></el-icon>
                搜索
              </el-button>
              <el-button @click="handleReset">
                <el-icon><Refresh /></el-icon>
                重置
              </el-button>
              <el-button @click="saveSearchTemplate">
                <el-icon><Collection /></el-icon>
                保存搜索
              </el-button>
              <el-dropdown @command="loadSearchTemplate">
                <el-button>
                  <el-icon><FolderOpened /></el-icon>
                  加载搜索<el-icon><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item
                      v-for="template in searchTemplates"
                      :key="template.id"
                      :command="template"
                    >
                      {{ template.name }}
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </el-form-item>
        </el-col>
      </el-row>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { 
  Search, Refresh, Collection, FolderOpened, ArrowDown 
} from '@element-plus/icons-vue'
import type { FileCategory, FileSearchParams } from '@/types/file'

interface Props {
  categories: FileCategory[]
  availableTags: any[]
}

interface Emits {
  (e: 'search', params: FileSearchParams): void
  (e: 'reset'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const searchForm = reactive<FileSearchParams & {
  fileTypes: string[]
  minSize?: number
  maxSize?: number
}>({
  keyword: '',
  categoryId: undefined,
  tags: [],
  issuingAuthority: '',
  isPublic: undefined,
  fileTypes: [],
  minSize: undefined,
  maxSize: undefined,
  sort: 'createTime',
  direction: 'desc',
  size: 10
})

const uploadDateRange = ref<[Date, Date] | null>(null)
const issueDateRange = ref<[Date, Date] | null>(null)
const sizeUnit = ref('MB')
const searchTemplates = ref([])

const cascaderProps = {
  value: 'id',
  label: 'name',
  children: 'children',
  emitPath: false
}

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

const handleUploadDateChange = (dates: [Date, Date] | null) => {
  if (dates) {
    searchForm.startDate = dates[0].toISOString()
    searchForm.endDate = dates[1].toISOString()
  } else {
    searchForm.startDate = undefined
    searchForm.endDate = undefined
  }
}

const handleIssueDateChange = (dates: [Date, Date] | null) => {
  if (dates) {
    searchForm.issueStartDate = dates[0].toISOString()
    searchForm.issueEndDate = dates[1].toISOString()
  } else {
    searchForm.issueStartDate = undefined
    searchForm.issueEndDate = undefined
  }
}

const handleSearch = () => {
  // 处理文件大小转换
  const params = { ...searchForm }
  
  if (searchForm.minSize) {
    params.minFileSize = convertSizeToBytes(searchForm.minSize, sizeUnit.value)
  }
  if (searchForm.maxSize) {
    params.maxFileSize = convertSizeToBytes(searchForm.maxSize, sizeUnit.value)
  }
  
  emit('search', params)
}

const handleReset = () => {
  Object.assign(searchForm, {
    keyword: '',
    categoryId: undefined,
    tags: [],
    issuingAuthority: '',
    isPublic: undefined,
    fileTypes: [],
    minSize: undefined,
    maxSize: undefined,
    sort: 'createTime',
    direction: 'desc',
    size: 10
  })
  
  uploadDateRange.value = null
  issueDateRange.value = null
  sizeUnit.value = 'MB'
  
  emit('reset')
}

const convertSizeToBytes = (size: number, unit: string) => {
  switch (unit) {
    case 'KB':
      return size * 1024
    case 'MB':
      return size * 1024 * 1024
    case 'GB':
      return size * 1024 * 1024 * 1024
    default:
      return size
  }
}

const saveSearchTemplate = () => {
  ElMessage.info('搜索模板保存功能待实现')
}

const loadSearchTemplate = (template: any) => {
  ElMessage.info('搜索模板加载功能待实现')
}

onMounted(() => {
  // 加载搜索模板
})
</script>

<style scoped>
.advanced-search {
  background: white;
  padding: 24px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.size-range {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.size-unit {
  margin-top: 8px;
}

.search-actions {
  display: flex;
  gap: 12px;
}
</style>
