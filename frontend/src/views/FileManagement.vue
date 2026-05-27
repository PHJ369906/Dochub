<template>
  <div class="file-management-page fade-in">
    <!-- 简洁的页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <h1 class="page-title">文档管理</h1>
          <div class="quick-stats">
            <span class="stat-item">共 {{ totalFileCount }} 个文档</span>
            <span class="stat-divider">•</span>
            <span class="stat-item">{{ categoryOptions.length }} 个分类</span>
            <template v-if="searchForm.categoryId">
              <span class="stat-divider">•</span>
              <span class="stat-item stat-filtered">当前筛选 {{ pagination.total }} 条</span>
            </template>
          </div>
        </div>
        <div class="header-actions">
          <el-dropdown @command="handleDataTransfer" class="data-transfer-dropdown">
            <el-button class="transfer-btn">
              <el-icon><Sort /></el-icon>
              数据管理
              <el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="export">
                  <el-icon><Upload /></el-icon>
                  导出数据
                </el-dropdown-item>
                <el-dropdown-item command="import">
                  <el-icon><Download /></el-icon>
                  导入数据
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          <el-button type="primary" @click="showUploadDialog = true" class="upload-btn">
            <el-icon><Upload /></el-icon>
            上传文档
          </el-button>
        </div>
      </div>
    </div>

    <!-- 重构的主要内容区域 -->
    <div class="main-layout" :style="mainLayoutStyle">
      <!-- 精简的侧边栏 -->
      <aside class="sidebar">
        <div class="sidebar-header">
          <h3 class="sidebar-title">分类</h3>
          <el-button
            v-if="user?.role === 'admin'"
            link
            size="small"
            @click="() => { currentEditCategory = null; showCategoryDialog = true }"
            class="add-btn"
          >
            <el-icon><Plus /></el-icon>
          </el-button>
        </div>

        <div class="sidebar-content">
          <!-- 全部文档 -->
          <div
            class="cat-item"
            :class="{ 'is-active': !searchForm.categoryId }"
            @click="handleAllCategoryClick"
          >
            <span class="cat-indent" style="width:0" />
            <span class="cat-name">全部文档</span>
            <span class="cat-count">{{ totalFileCount }}</span>
          </div>

          <!-- 自定义扁平分类列表 -->
          <template v-for="cat in flatCategories" :key="cat.id">
            <div
              class="cat-item"
              :class="{ 'is-active': searchForm.categoryId === cat.id }"
              :style="{ paddingLeft: `${6 + cat.depth * 14}px` }"
              @click="handleCategoryClick(cat)"
            >
              <!-- 展开/折叠箭头 -->
              <span
                v-if="cat.hasChildren"
                class="cat-arrow"
                :class="{ 'is-expanded': expandedIds.has(cat.id) }"
                @click.stop="toggleExpand(cat.id)"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                  <polyline points="9 18 15 12 9 6"/>
                </svg>
              </span>
              <span v-else class="cat-arrow-placeholder" />

              <el-tooltip :content="cat.name" placement="right" :show-after="600" :disabled="cat.name.length <= 10">
                <span class="cat-name">{{ cat.name }}</span>
              </el-tooltip>
              <span class="cat-count">{{ cat.totalCount || 0 }}</span>

              <span v-if="user?.role === 'admin'" class="cat-actions">
                <el-button link size="small" @click.stop="editCategory(cat)">
                  <el-icon><Edit /></el-icon>
                </el-button>
                <el-button link size="small" @click.stop="openMoveDialog(cat)">
                  <el-icon><Rank /></el-icon>
                </el-button>
                <el-button link size="small" @click.stop="deleteCategory(cat)">
                  <el-icon><Delete /></el-icon>
                </el-button>
              </span>
            </div>
          </template>
        </div>
      </aside>

      <!-- 侧边栏拖拽调宽把手 -->
      <div class="resize-handle" @mousedown="onResizeStart" />

      <!-- 主内容区域 -->
      <main class="main-content">
        <!-- 工具栏 -->
        <div class="toolbar">
          <div class="toolbar-left">
            <el-input
              v-model="searchForm.keyword"
              placeholder="搜索文档..."
              class="search-input"
              clearable
              @keyup.enter="handleSearch"
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
            </el-input>
            <el-button @click="handleSearch" class="search-btn">
              搜索
            </el-button>
          </div>
          <div class="toolbar-right">
            <el-button
              @click="showAdvancedSearch = !showAdvancedSearch"
              :class="{ 'is-active': showAdvancedSearch }"
              class="filter-btn"
            >
              <el-icon><Filter /></el-icon>
              筛选
            </el-button>
            <el-button @click="refreshFileList" class="refresh-btn">
              <el-icon><Refresh /></el-icon>
            </el-button>
            <el-dropdown v-if="selectedFiles.length > 0" @command="handleBatchAction">
              <el-button type="danger" class="batch-btn">
                批量操作 ({{ selectedFiles.length }})
                <el-icon class="el-icon--right"><ArrowDown /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="delete">
                    <el-icon><Delete /></el-icon>
                    删除选中
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </div>

        <!-- 高级搜索面板 -->
        <el-collapse-transition>
          <div v-show="showAdvancedSearch" class="advanced-filters">
            <div class="filter-row">
              <el-input
                v-model="searchForm.issuingAuthority"
                placeholder="发布机关"
                clearable
                class="filter-input"
              />
              <el-select
                v-model="searchForm.tags"
                multiple
                placeholder="标签"
                clearable
                class="filter-select"
                @change="handleTagsChange"
              >
                <el-option
                  v-for="tag in availableTags"
                  :key="tag.id"
                  :label="tag.name"
                  :value="tag.name"
                />
              </el-select>
              <el-date-picker
                v-model="dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                @change="handleDateRangeChange"
                class="filter-date"
              />
              <el-button @click="resetSearch" class="reset-btn">
                重置
              </el-button>
            </div>
          </div>
        </el-collapse-transition>

        <!-- 文件列表 -->
        <div class="file-list-container">
          <div class="file-list">

            <!-- 现代化文件表格 -->
            <el-table
              v-loading="loading"
              :data="fileList"
              style="width: 100%"
              @selection-change="handleSelectionChange"
              empty-text="暂无文档数据"
              class="modern-file-table"
              :row-class-name="getRowClassName"
            >
              <el-table-column type="selection" width="60" />
              <el-table-column prop="title" label="文档信息" min-width="350">
                <template #default="{ row }">
                  <div class="modern-file-title hover-lift">
                    <div class="file-icon-wrapper">
                      <div class="file-icon" :class="getFileTypeClass(row.fileType)">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                          <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                          <polyline points="13,2 13,9 20,9"/>
                        </svg>
                      </div>
                      <div class="file-type-badge">{{ row.fileType.toUpperCase() }}</div>
                    </div>
                    <div class="file-details">
                      <div @click="previewFile(row)" class="file-name">{{ row.title }}</div>
                      <div class="file-meta">
                        <span class="file-size">{{ formatFileSize(row.fileSize) }}</span>
                        <span class="file-separator">•</span>
                        <span class="file-upload-time">{{ formatDate(row.createTime) }}</span>
                      </div>
                    </div>
                  </div>
                </template>
              </el-table-column>
              <el-table-column prop="category" label="分类" width="140">
                <template #default="{ row }">
                  <el-tag v-if="row.category" class="modern-category-tag" size="small">
                    <div class="tag-icon">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                      </svg>
                    </div>
                    {{ row.category.name }}
                  </el-tag>
                  <span v-else class="no-category">未分类</span>
                </template>
              </el-table-column>
              <el-table-column prop="issuingAuthority" label="发布机关" width="160" show-overflow-tooltip>
                <template #default="{ row }">
                  <div class="authority-info">
                    <div class="authority-icon">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                        <circle cx="9" cy="7" r="4"/>
                        <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                        <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                      </svg>
                    </div>
                    <span>{{ row.issuingAuthority || '未知' }}</span>
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="数据统计" width="140">
                <template #default="{ row }">
                  <div class="modern-file-stats">
                    <div class="stat-item">
                      <div class="stat-icon view">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                          <circle cx="12" cy="12" r="3"/>
                        </svg>
                      </div>
                      <span class="stat-value">{{ row.viewCount || 0 }}</span>
                    </div>
                    <div class="stat-item">
                      <div class="stat-icon download">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                          <polyline points="7,10 12,15 17,10"/>
                          <line x1="12" y1="15" x2="12" y2="3"/>
                        </svg>
                      </div>
                      <span class="stat-value">{{ row.downloadCount || 0 }}</span>
                    </div>
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="操作" width="220" fixed="right">
                <template #default="{ row }">
                  <div class="modern-action-buttons">
                    <el-button type="primary" size="small" @click="previewFile(row)" class="action-btn preview">
                      <el-icon><View /></el-icon>
                      预览
                    </el-button>
                    <el-button size="small" @click="downloadFile(row)" class="action-btn download">
                      <el-icon><Download /></el-icon>
                      下载
                    </el-button>
                    <el-dropdown v-if="canEditFile(row)" @command="handleFileAction" trigger="click" class="action-dropdown">
                      <el-button size="small" class="action-btn more">
                        <el-icon><MoreFilled /></el-icon>
                      </el-button>
                      <template #dropdown>
                        <el-dropdown-menu class="modern-dropdown-menu">
                          <el-dropdown-item :command="{action: 'edit', file: row}" class="dropdown-item">
                            <div class="dropdown-item-content">
                              <el-icon class="dropdown-icon"><Edit /></el-icon>
                              <span>编辑文档</span>
                            </div>
                          </el-dropdown-item>
                          <el-dropdown-item :command="{action: 'delete', file: row}" class="dropdown-item danger">
                            <div class="dropdown-item-content">
                              <el-icon class="dropdown-icon"><Delete /></el-icon>
                              <span>删除文档</span>
                            </div>
                          </el-dropdown-item>
                        </el-dropdown-menu>
                      </template>
                    </el-dropdown>
                  </div>
                </template>
              </el-table-column>
            </el-table>

            <!-- 分页 -->
            <div class="pagination-container">
              <el-pagination
                v-model:current-page="pagination.page"
                v-model:page-size="pagination.size"
                :page-sizes="[10, 20, 50, 100]"
                :total="pagination.total"
                layout="total, sizes, prev, pager, next, jumper"
                @size-change="handleSizeChange"
                @current-change="handleCurrentChange"
              />
            </div>
          </div>
        </div>
      </main>
    </div>

    <!-- 文件上传对话框 -->
    <FileUploadDialog
      v-model="showUploadDialog"
      :categories="categoryOptions"
      @success="handleUploadSuccess"
    />

    <!-- 分类管理对话框 -->
    <CategoryDialog
      v-model="showCategoryDialog"
      :categories="categoryOptions"
      :editCategory="currentEditCategory"
      @success="handleCategorySuccess"
    />

    <!-- 文件预览对话框 -->
    <FilePreviewDialog
      v-model="showPreviewDialog"
      :file="currentPreviewFile"
    />

    <!-- 文件编辑对话框 -->
    <FileEditDialog
      v-model="showFileEditDialog"
      :editFile="currentEditFile"
      :categories="categoryOptions"
      :availableTags="availableTags"
      @success="handleFileEditSuccess"
    />

    <!-- 数据导出对话框 -->
    <DataExportDialog
      v-model="showExportDialog"
      :categories="categoryOptions"
    />

    <!-- 移动分类对话框 -->
    <el-dialog
      v-model="showMoveCategoryDialog"
      title="移动分类"
      width="400px"
      :close-on-click-modal="false"
    >
      <div v-if="movingCategory" class="move-dialog-body">
        <div class="move-source">
          <span class="move-label">当前分类：</span>
          <el-tag>{{ movingCategory.name }}</el-tag>
        </div>
        <div class="move-target">
          <span class="move-label">移动到：</span>
          <el-select
            v-model="moveTargetId"
            placeholder="选择目标父分类（不选则移到根目录）"
            clearable
            style="width: 100%"
          >
            <el-option label="根目录（顶级分类）" :value="null" />
            <el-option
              v-for="opt in moveTargetOptions"
              :key="opt.id"
              :label="opt.label"
              :value="opt.id"
              :style="{ paddingLeft: `${12 + opt.depth * 16}px` }"
            />
          </el-select>
        </div>
        <p class="move-tip">移动后，该分类下的所有子分类和文件将保持原有层级结构一起迁移。</p>
      </div>
      <template #footer>
        <el-button @click="showMoveCategoryDialog = false">取消</el-button>
        <el-button type="primary" :loading="moveLoading" @click="confirmMove">确认移动</el-button>
      </template>
    </el-dialog>

    <!-- 数据导入对话框 -->
    <DataImportDialog
      v-model="showImportDialog"
      @success="handleImportSuccess"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Plus, Edit, Delete, Upload, Refresh, Search, Filter,
  View, Download, MoreFilled, RefreshLeft, ArrowDown, Sort, Rank
} from '@element-plus/icons-vue'
import { useAuthStore } from '@/stores/auth'
import { getFileList, getCategories, deleteFile, createCategory, deleteCategory as deleteCategoryApi, updateCategory, getAllTags, moveCategory as moveCategoryApi } from '@/api/file'
import type { PolicyFile, FileCategory, FileSearchParams } from '@/types/file'
import FileUploadDialog from '@/components/FileUploadDialog.vue'
import CategoryDialog from '@/components/CategoryDialog.vue'
import FilePreviewDialog from '@/components/FilePreviewDialog.vue'
import FileEditDialog from '@/components/FileEditDialog.vue'
import DataExportDialog from '@/components/DataExportDialog.vue'
import DataImportDialog from '@/components/DataImportDialog.vue'

const authStore = useAuthStore()
const user = computed(() => authStore.user)

// 侧边栏宽度（可拖拽调整，持久化到 localStorage）
const SIDEBAR_WIDTH_KEY = 'doc-sidebar-width'
const sidebarWidth = ref(parseInt(localStorage.getItem(SIDEBAR_WIDTH_KEY) || '260'))
const isResizing = ref(false)
const mainLayoutStyle = computed(() => ({
  gridTemplateColumns: `${sidebarWidth.value}px 4px 1fr`
}))

let _resizeStartX = 0
let _resizeStartWidth = 0
const _onResizeMove = (e: MouseEvent) => {
  const delta = e.clientX - _resizeStartX
  sidebarWidth.value = Math.min(480, Math.max(180, _resizeStartWidth + delta))
}
const _onResizeEnd = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', _onResizeMove)
  document.removeEventListener('mouseup', _onResizeEnd)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
  localStorage.setItem(SIDEBAR_WIDTH_KEY, String(sidebarWidth.value))
}
const onResizeStart = (e: MouseEvent) => {
  isResizing.value = true
  _resizeStartX = e.clientX
  _resizeStartWidth = sidebarWidth.value
  document.addEventListener('mousemove', _onResizeMove)
  document.addEventListener('mouseup', _onResizeEnd)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}

// 响应式数据
const loading = ref(false)
const fileList = ref<PolicyFile[]>([])
const categoryTree = ref<FileCategory[]>([])
const categoryOptions = ref<FileCategory[]>([])
const availableTags = ref([])
const selectedFiles = ref<PolicyFile[]>([])
const currentPreviewFile = ref<PolicyFile | null>(null)
const currentEditFile = ref<PolicyFile | null>(null)
const totalFileCount = ref(0)
const expandedIds = ref<Set<number>>(new Set()) // 已展开的分类 id

// 扁平化分类树，加入 depth / hasChildren，按展开状态过滤
type FlatCategory = FileCategory & { depth: number; hasChildren: boolean }
const flatCategories = computed<FlatCategory[]>(() => {
  const result: FlatCategory[] = []
  const walk = (list: FileCategory[], depth: number) => {
    list.forEach(cat => {
      const hasChildren = !!(cat.children && cat.children.length > 0)
      result.push({ ...cat, depth, hasChildren })
      if (hasChildren && expandedIds.value.has(cat.id)) {
        walk(cat.children!, depth + 1)
      }
    })
  }
  walk(categoryTree.value, 0)
  return result
})

const toggleExpand = (id: number) => {
  const s = new Set(expandedIds.value)
  if (s.has(id)) s.delete(id)
  else s.add(id)
  expandedIds.value = s
}

// 加载分类后默认展开第一层
watch(categoryTree, (tree) => {
  const ids = new Set<number>()
  tree.forEach(cat => ids.add(cat.id))
  expandedIds.value = ids
}, { immediate: true })

// 对话框显示状态
const showUploadDialog = ref(false)
const showCategoryDialog = ref(false)
const currentEditCategory = ref<FileCategory | null>(null)
const showPreviewDialog = ref(false)
const showAdvancedSearch = ref(false)
const showFileEditDialog = ref(false)
const showExportDialog = ref(false)
const showImportDialog = ref(false)
const showMoveCategoryDialog = ref(false)

// 移动分类
const movingCategory = ref<FileCategory | null>(null)
const moveTargetId = ref<number | null>(null)
const moveLoading = ref(false)

// 获取某分类及其所有后代的 id 集合（用于排除不可选项）
const getDescendantIds = (cat: FileCategory): Set<number> => {
  const ids = new Set<number>([cat.id])
  const walk = (list: FileCategory[]) => {
    list.forEach(c => {
      ids.add(c.id)
      if (c.children?.length) walk(c.children)
    })
  }
  if (cat.children?.length) walk(cat.children)
  return ids
}

// 移动对话框的可选目标列表（排除自身及后代）
const moveTargetOptions = computed(() => {
  if (!movingCategory.value) return []
  const excludeIds = getDescendantIds(movingCategory.value)
  const result: Array<{ id: number; label: string; depth: number }> = []
  const walk = (list: FileCategory[], depth: number) => {
    list.forEach(cat => {
      if (!excludeIds.has(cat.id)) {
        result.push({ id: cat.id, label: cat.name, depth })
        if (cat.children?.length) walk(cat.children, depth + 1)
      }
    })
  }
  walk(categoryTree.value, 0)
  return result
})

const openMoveDialog = (cat: FileCategory) => {
  movingCategory.value = cat
  moveTargetId.value = null
  showMoveCategoryDialog.value = true
}

const confirmMove = async () => {
  if (!movingCategory.value) return
  moveLoading.value = true
  try {
    const response = await moveCategoryApi(movingCategory.value.id, moveTargetId.value ?? undefined)
    if (response.code === 200) {
      ElMessage.success('分类移动成功')
      showMoveCategoryDialog.value = false
      await loadCategories()
      if (searchForm.categoryId === movingCategory.value.id) {
        loadFileList()
      }
    } else {
      ElMessage.error(response.message || '移动失败')
    }
  } catch (e: any) {
    ElMessage.error(e.message || '移动失败')
  } finally {
    moveLoading.value = false
  }
}

// 搜索表单
const searchForm = reactive<FileSearchParams>({
  keyword: '',
  categoryId: undefined,
  tags: [],
  issuingAuthority: '',
  page: 1,
  size: 10,
  sort: 'createTime',
  direction: 'desc'
})

const dateRange = ref<[Date, Date] | null>(null)

// 分页信息
const pagination = reactive({
  page: 1,
  size: 10,
  total: 0
})


// 生命周期
onMounted(() => {
  loadCategories()
  loadTags()
  loadFileList()
})

// 方法
const loadCategories = async () => {
  try {
    const response = await getCategories()
    if (response.code === 200) {
      const treeData = buildCategoryTree(response.data)
      categoryTree.value = treeData
      categoryOptions.value = flattenCategories(treeData)
    }
  } catch (error) {
    ElMessage.error('加载分类失败')
  }
}

const loadTags = async () => {
  try {
    const response = await getAllTags()
    if (response.code === 200) {
      availableTags.value = response.data
    }
  } catch (error) {
    console.error('加载标签失败:', error)
  }
}

const buildCategoryTree = (categories: FileCategory[]): FileCategory[] => {
  const categoryMap = new Map<number, FileCategory>()
  const result: FileCategory[] = []

  // 先创建所有节点的映射
  categories.forEach(category => {
    categoryMap.set(category.id, {
      ...category,
      children: [],
      fileCount: category.fileCount || 0,
      totalCount: category.fileCount || 0
    })
  })

  // 构建树形结构
  categories.forEach(category => {
    const node = categoryMap.get(category.id)
    if (node) {
      if (category.parentId && categoryMap.has(category.parentId)) {
        const parent = categoryMap.get(category.parentId)
        if (parent && parent.children) {
          parent.children.push(node)
        }
      } else {
        result.push(node)
      }
    }
  })

  return result
}

const flattenCategories = (categories: FileCategory[]): FileCategory[] => {
  const result: FileCategory[] = []
  const flatten = (cats: FileCategory[], level = 0) => {
    cats.forEach(cat => {
      result.push({ ...cat, level })
      if (cat.children && cat.children.length > 0) {
        flatten(cat.children, level + 1)
      }
    })
  }
  flatten(categories)
  return result
}

const loadFileList = async () => {
  loading.value = true
  try {
    const params = {
      ...searchForm,
      page: pagination.page - 1, // 后端从0开始
      size: pagination.size
    }
    const response = await getFileList(params)
    if (response.code === 200) {
      fileList.value = response.data.records || []
      pagination.total = response.data.total || 0
      // 无分类筛选时更新全局文档总数
      if (!searchForm.categoryId) {
        totalFileCount.value = response.data.total || 0
      }
    }
  } catch (error) {
    ElMessage.error('加载文件列表失败')
  } finally {
    loading.value = false
  }
}

const handleAllCategoryClick = () => {
  searchForm.categoryId = undefined
  pagination.page = 1
  
  loadFileList()
}

const handleCategoryClick = (data: FileCategory) => {
  searchForm.categoryId = data.id
  pagination.page = 1
  loadFileList()
}

const handleSearch = () => {
  pagination.page = 1
  loadFileList()
}

const resetSearch = () => {
  Object.assign(searchForm, {
    keyword: '',
    categoryId: undefined,
    tags: [],
    issuingAuthority: '',
    page: 1,
    size: 10,
    sort: 'createTime',
    direction: 'desc'
  })
  dateRange.value = null
  
  loadFileList()
}

const handleDateRangeChange = (dates: [Date, Date] | null) => {
  if (dates) {
    searchForm.startDate = dates[0].toISOString()
    searchForm.endDate = dates[1].toISOString()
  } else {
    searchForm.startDate = undefined
    searchForm.endDate = undefined
  }
  // 日期范围变化时自动触发搜索
  handleSearch()
}

const handleTagsChange = () => {
  // 标签变化时自动触发搜索
  handleSearch()
}

const refreshFileList = () => {
  loadFileList()
}

const handleSizeChange = (size: number) => {
  pagination.size = size
  pagination.page = 1
  loadFileList()
}

const handleCurrentChange = (page: number) => {
  pagination.page = page
  loadFileList()
}

const handleSelectionChange = (selection: PolicyFile[]) => {
  selectedFiles.value = selection
}

const previewFile = (file: PolicyFile) => {
  currentPreviewFile.value = file
  showPreviewDialog.value = true
}

const downloadFile = async (file: PolicyFile) => {
  try {
    const { openFileWithSystem } = await import('@/api/file')
    const response = await openFileWithSystem(file.id)
    if (response.code === 200) {
      ElMessage.success('文件已打开')
    } else {
      ElMessage.error('打开文件失败')
    }
  } catch (error) {
    console.error('打开文件失败:', error)
    ElMessage.error('打开文件失败，请稍后重试')
  }
}

const canEditFile = (file: PolicyFile) => {
  return user.value?.role === 'admin' || file.createdBy === user.value?.id
}

const handleFileAction = async ({ action, file }: { action: string, file: PolicyFile }) => {
  switch (action) {
    case 'edit':
      editFile(file)
      break
    case 'delete':
      await handleDeleteFile(file)
      break
  }
}

const editFile = (file: PolicyFile) => {
  currentEditFile.value = file
  showFileEditDialog.value = true
}

const handleDeleteFile = async (file: PolicyFile) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除文件 "${file.title}" 吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
  } catch {
    // 用户取消操作
    return
  }

  try {
    const response = await deleteFile(file.id)
    if (response.code === 200) {
      ElMessage.success('文件删除成功')
      loadFileList()
    }
  } catch (error: any) {
    ElMessage.error(error.message || '删除失败')
  }
}

const handleUploadSuccess = () => {
  showUploadDialog.value = false
  loadFileList()
}

const handleCategorySuccess = () => {
  showCategoryDialog.value = false
  currentEditCategory.value = null
  loadCategories()
}

const handleFileEditSuccess = () => {
  showFileEditDialog.value = false
  currentEditFile.value = null
  loadFileList()
}

const handleDataTransfer = (command: string) => {
  if (command === 'export') {
    showExportDialog.value = true
  } else if (command === 'import') {
    showImportDialog.value = true
  }
}

const handleImportSuccess = () => {
  loadCategories()
  loadTags()
  loadFileList()
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

// 新增：获取文件类型样式类
const getFileTypeClass = (fileType: string) => {
  const typeMap: Record<string, string> = {
    'pdf': 'file-type-pdf',
    'doc': 'file-type-doc',
    'docx': 'file-type-doc',
    'xls': 'file-type-excel',
    'xlsx': 'file-type-excel',
    'ppt': 'file-type-ppt',
    'pptx': 'file-type-ppt',
    'txt': 'file-type-text',
    'jpg': 'file-type-image',
    'jpeg': 'file-type-image',
    'png': 'file-type-image',
    'gif': 'file-type-image'
  }
  return typeMap[fileType.toLowerCase()] || 'file-type-default'
}

// 新增：获取表格行样式类
const getRowClassName = ({ rowIndex }: { rowIndex: number }) => {
  return rowIndex % 2 === 0 ? 'even-row' : 'odd-row'
}

const editCategory = (category: FileCategory) => {
  // 设置当前编辑的分类
  currentEditCategory.value = category
  showCategoryDialog.value = true
}

const deleteCategory = async (category: FileCategory) => {
  try {
    // 检查是否有子分类
    if (category.children && category.children.length > 0) {
      ElMessage.warning('该分类下还有子分类，请先删除子分类')
      return
    }
    
    // 检查是否有文件
    if (category.fileCount && category.fileCount > 0) {
      ElMessage.warning('该分类下还有文件，请先移动或删除文件')
      return
    }
    
    await ElMessageBox.confirm(
      `确定要删除分类 "${category.name}" 吗？`,
      '删除分类确认',
      {
        confirmButtonText: '确定删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const response = await deleteCategoryApi(category.id!)
    if (response.code === 200) {
      ElMessage.success('分类删除成功')
      await loadCategories()
      // 如果当前选中的是被删除的分类，清空选择
      if (searchForm.categoryId === category.id) {
        searchForm.categoryId = undefined
        await loadFileList()
      }
    } else {
      ElMessage.error(response.message || '删除失败')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error(error.message || '删除失败')
    }
  }
}

// 批量删除文件
const batchDelete = async () => {
  if (selectedFiles.value.length === 0) {
    ElMessage.warning('请先选择要删除的文件')
    return
  }

  try {
    await ElMessageBox.confirm(
      `确定要删除选中的 ${selectedFiles.value.length} 个文件吗？`,
      '批量删除确认',
      {
        confirmButtonText: '确定删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    // 并发执行删除
    const results = await Promise.allSettled(
      selectedFiles.value.map(file => deleteFile(file.id))
    )

    const successCount = results.filter(r => r.status === 'fulfilled' && (r.value as any).code === 200).length
    const failCount = results.length - successCount

    if (failCount === 0) {
      ElMessage.success(`成功删除 ${successCount} 个文件`)
    } else {
      ElMessage.warning(`删除完成：成功 ${successCount} 个，失败 ${failCount} 个`)
    }

    selectedFiles.value = []
    await loadFileList()
  } catch (error) {
    // 用户取消操作
  }
}

// 处理批量操作
const handleBatchAction = (command: string) => {
  switch (command) {
    case 'delete':
      batchDelete()
      break
  }
}
</script>

<style scoped>
/* 全局防溢出规则 */
* {
  box-sizing: border-box;
}

*:before,
*:after {
  box-sizing: border-box;
}

/* 防止所有元素产生水平滚动 */
.file-management-page *,
.file-management-page *:before,
.file-management-page *:after {
  max-width: 100%;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

/* 防溢出的文件管理页面样式 */
.file-management-page {
  max-width: 1400px;
  width: 100%;
  margin: 0 auto;
  padding: 0 1rem;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  overflow-x: hidden;
  box-sizing: border-box;
}

/* 简洁的页面头部 */
.page-header {
  margin-bottom: 1rem;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1.5rem 0;
  border-bottom: 1px solid var(--border-light);
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin: 0;
  color: var(--neutral-900);
}

.quick-stats {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--neutral-600);
}

.stat-item {
  font-weight: 500;
}

.stat-divider {
  color: var(--neutral-400);
  margin: 0 0.25rem;
}

.header-actions {
  display: flex;
  gap: 0.75rem;
}

.upload-btn {
  background: var(--accent-600);
  border: none;
  font-weight: 600;
  padding: 0.75rem 1.5rem;
}

.header-text {
  flex: 1;
}

.page-title {
  font-size: 2.5rem;
  font-weight: 800;
  margin: 0 0 0.75rem 0;
  line-height: 1.1;
  letter-spacing: -0.025em;
}

.page-subtitle {
  font-size: 1.125rem;
  color: var(--neutral-600);
  margin: 0 0 1.5rem 0;
  line-height: 1.5;
  font-weight: 400;
}

.header-stats {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.stat-badge {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--bg-glass);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-full);
  backdrop-filter: blur(10px);
}

.stat-number {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--accent-600);
}

.stat-label {
  font-size: 0.875rem;
  color: var(--neutral-600);
  font-weight: 500;
}

.header-actions {
  display: flex;
  gap: 1rem;
  position: relative;
  z-index: 1;
}

.action-button {
  padding: 0.75rem 1.5rem;
  font-weight: 600;
  border-radius: var(--radius-xl);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 防溢出的主布局 */
.main-layout {
  display: grid;
  /* grid-template-columns 由 JS 动态注入，默认三列：sidebar + handle + main */
  gap: 0;
  align-items: start;
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

/* 拖拽调宽把手 */
.resize-handle {
  width: 4px;
  align-self: stretch;
  cursor: col-resize;
  background: transparent;
  transition: background 0.2s;
  position: relative;
  z-index: 10;
}
.resize-handle::after {
  content: '';
  position: absolute;
  inset: 0 -3px;
}
.resize-handle:hover,
.resize-handle:active {
  background: var(--accent-300, #93c5fd);
}

/* 防溢出的侧边栏 */
.sidebar {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
  position: sticky;
  top: 1rem;
  max-height: calc(100vh - 2rem);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
  width: 100%;
  box-sizing: border-box;
}

.sidebar-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-secondary);
  min-height: 60px;
}

.sidebar-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--neutral-700);
  margin: 0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.add-btn {
  color: var(--neutral-500);
  padding: 0.5rem;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.add-btn:hover {
  color: var(--accent-600);
  background: var(--accent-50);
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  padding: 1rem;
  position: relative;
}

/* 美化分类面板滚动条 */
.sidebar-content::-webkit-scrollbar {
  width: 4px;
  height: 4px;
}

.sidebar-content::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-content::-webkit-scrollbar-thumb {
  background: var(--neutral-300);
  border-radius: 2px;
  transition: background 0.3s ease;
}

.sidebar-content::-webkit-scrollbar-thumb:hover {
  background: var(--accent-400);
}

.sidebar-content {
  scrollbar-width: thin;
  scrollbar-color: var(--neutral-300) transparent;
}

.sidebar-content::before {
  content: '';
  position: absolute;
  top: 0;
  left: 1rem;
  right: 1rem;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-light), transparent);
  opacity: 0.5;
}

/* 全部文档项 */
/* ── 自定义分类列表 ── */
.cat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px 6px 6px;
  margin-bottom: 1px;
  border-radius: var(--radius-md);
  cursor: pointer;
  border: 1px solid transparent;
  transition: background 0.15s, border-color 0.15s;
  min-height: 32px;
  position: relative;
  min-width: max-content;
}

.cat-item:hover {
  background: var(--bg-secondary);
}

.cat-item.is-active {
  background: var(--accent-50);
  border-color: var(--accent-200);
}

.cat-item.is-active .cat-name {
  color: var(--accent-700);
  font-weight: 600;
}

.cat-item.is-active .cat-count {
  background: var(--accent-600);
  color: white;
  border-color: transparent;
}

/* 箭头 */
.cat-arrow {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  transition: transform 0.2s ease;
  color: var(--neutral-400);
}

.cat-arrow svg {
  width: 12px;
  height: 12px;
}

.cat-arrow:hover {
  color: var(--accent-600);
  background: var(--accent-50);
}

.cat-arrow.is-expanded {
  transform: rotate(90deg);
}

.cat-arrow-placeholder {
  width: 16px;
  flex-shrink: 0;
}

/* 名称 */
.cat-name {
  flex: 0 0 auto;
  font-size: 0.8125rem;
  color: var(--neutral-700);
  font-weight: 500;
  white-space: nowrap;
  line-height: 1.4;
}

/* 数量徽章 */
.cat-count {
  flex-shrink: 0;
  margin-left: auto;
  font-size: 0.6875rem;
  font-weight: 600;
  color: var(--neutral-500);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-light);
  border-radius: 10px;
  padding: 0 6px;
  height: 18px;
  line-height: 18px;
  min-width: 22px;
  text-align: center;
}

/* 管理操作 — 悬停时显示，绝对定位覆盖右侧 */
.cat-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.15s;
  flex-shrink: 0;
}

.cat-item:hover .cat-actions {
  opacity: 1;
  pointer-events: auto;
}

.cat-actions .el-button {
  width: 22px;
  height: 22px;
  padding: 0;
  border-radius: 4px;
  color: var(--neutral-400);
}

.cat-actions .el-button:hover {
  color: var(--accent-600);
  background: var(--accent-50);
}

/* 移动按钮特殊颜色 */
.cat-actions .el-button:nth-child(2):hover {
  color: var(--accent-500);
  background: var(--accent-50);
}

.cat-actions .el-button:last-child:hover {
  color: var(--error, #f56c6c);
  background: #fff0f0;
}

/* 移动分类对话框 */
.move-dialog-body {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.move-source,
.move-target {
  display: flex;
  align-items: center;
  gap: 10px;
}

.move-label {
  flex-shrink: 0;
  font-size: 13px;
  color: var(--neutral-600);
  width: 70px;
}

.move-tip {
  font-size: 12px;
  color: var(--neutral-500);
  background: var(--bg-secondary);
  border-radius: 6px;
  padding: 8px 12px;
  margin: 0;
  line-height: 1.6;
}

/* 筛选状态标注 */
.stat-filtered {
  color: var(--accent-600);
  font-weight: 600;
}

/* 防溢出的主内容区域 */
.main-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-height: 0;
  min-width: 0;
  overflow: hidden;
  width: 100%;
  box-sizing: border-box;
}

/* 防溢出的工具栏 */
.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
  gap: 1rem;
  min-height: 60px;
  width: 100%;
  box-sizing: border-box;
  overflow: hidden;
}

.toolbar-left {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.toolbar-right {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-shrink: 0;
  flex-wrap: wrap;
}

.search-input {
  max-width: 320px;
  flex: 1;
  min-width: 0;
  width: 100%;
}

.search-input :deep(.el-input__wrapper) {
  height: 36px;
  align-items: center;
  width: 100%;
  box-sizing: border-box;
}

.search-input :deep(.el-input__inner) {
  width: 100%;
  box-sizing: border-box;
}

.search-btn {
  background: var(--accent-600);
  border-color: var(--accent-600);
  color: white;
  height: 36px;
  padding: 0 1rem;
  font-weight: 500;
}

.filter-btn {
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  color: var(--neutral-600);
  height: 36px;
  padding: 0 1rem;
  font-weight: 500;
}

.filter-btn.is-active {
  background: var(--accent-50);
  border-color: var(--accent-200);
  color: var(--accent-700);
}

.refresh-btn {
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  color: var(--neutral-600);
  height: 36px;
  padding: 0 0.75rem;
  font-weight: 500;
}

.batch-btn {
  background: var(--error);
  border-color: var(--error);
  color: white;
  height: 36px;
  padding: 0 1rem;
  font-weight: 500;
}

/* 防溢出的高级筛选 */
.advanced-filters {
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  padding: 1rem;
  border: 1px solid var(--border-light);
  margin-top: 1rem;
  width: 100%;
  box-sizing: border-box;
  overflow: hidden;
}

.filter-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  align-items: center;
  width: 100%;
}

.filter-input,
.filter-select,
.filter-date {
  background: var(--bg-primary);
  width: 100%;
  min-width: 0;
}

.filter-input :deep(.el-input__wrapper),
.filter-select :deep(.el-input__wrapper),
.filter-date :deep(.el-input__wrapper) {
  height: 36px;
  align-items: center;
  width: 100%;
  box-sizing: border-box;
}

.reset-btn {
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  color: var(--neutral-600);
  height: 36px;
  padding: 0 1rem;
  font-weight: 500;
  white-space: nowrap;
}

.advanced-search {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-light);
}

.search-form {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  align-items: end;
}

.search-form .el-form-item {
  margin-bottom: 0;
}

/* 防溢出的文件列表容器 */
.file-list-container {
  flex: 1;
  min-height: 0;
  min-width: 0;
  width: 100%;
  overflow: hidden;
}

.file-list {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
  overflow: hidden;
  height: 100%;
  display: flex;
  flex-direction: column;
  width: 100%;
  box-sizing: border-box;
}

/* 侧边栏样式 */
.category-sidebar {
  width: 320px;
  background: var(--bg-primary);
  border-right: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  position: sticky;
  top: 0;
  height: 100vh;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.sidebar-header {
  padding: 2rem 1.5rem 1.5rem;
  border-bottom: 1px solid var(--border-light);
  flex-shrink: 0;
}

.sidebar-header h3 {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--neutral-900);
  margin: 0 0 1rem 0;
}

.sidebar-header .el-button {
  width: 100%;
  justify-content: center;
}

.category-tree-container {
  flex: 1;
  overflow-y: auto;
  padding: 0 1.5rem 1.5rem;
}

.category-node {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 0.75rem 0;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.category-node:hover {
  background: var(--bg-secondary);
  transform: translateX(2px);
}

.category-name {
  flex: 1;
  font-weight: 500;
  color: var(--neutral-700);
}

.category-count {
  color: var(--neutral-500);
  font-size: 0.875rem;
  margin-left: 0.5rem;
  background: var(--bg-tertiary);
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius-sm);
  font-weight: 500;
  border: 1px solid var(--border-light);
}

.category-actions {
  display: none;
  margin-left: 0.75rem;
  gap: 0.25rem;
}

.category-node:hover .category-actions {
  display: flex;
}

/* 主内容区域 */
.file-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

/* 工具栏 */
.toolbar {
  background: var(--bg-primary);
  padding: 1.5rem 2rem;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
  position: sticky;
  top: 0;
  z-index: 10;
  backdrop-filter: blur(10px);
  box-shadow: var(--shadow-xs);
}

.toolbar-left {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.toolbar-right {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.search-input {
  width: 320px;
}

/* 高级搜索面板 */
.advanced-search {
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-light);
  padding: 1.5rem 2rem;
  box-shadow: var(--shadow-inner);
}

.advanced-search .el-form {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
  align-items: end;
}

/* 文件列表容器 */
.file-list-container {
  flex: 1;
  padding: 2rem;
  overflow: auto;
}

.file-list {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

/* 防溢出的表格样式 */
.modern-file-table {
  border: none !important;
  flex: 1;
  width: 100%;
  table-layout: fixed;
}

.modern-file-table :deep(.el-table__header) {
  background: var(--bg-secondary);
}

.modern-file-table :deep(.el-table__header-wrapper) {
  overflow-x: auto;
}

.modern-file-table :deep(.el-table__body-wrapper) {
  overflow-x: auto;
}

.modern-file-table :deep(.el-table th) {
  background: transparent;
  border-bottom: 1px solid var(--border-light);
  color: var(--neutral-700);
  font-weight: 600;
  font-size: 0.875rem;
  padding: 1rem 0.75rem;
  text-align: left;
  vertical-align: middle;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.modern-file-table :deep(.el-table td) {
  border-bottom: 1px solid var(--border-light);
  padding: 1rem 0.75rem;
  vertical-align: middle;
  overflow: hidden;
  text-overflow: ellipsis;
}

.modern-file-table :deep(.el-table__row:hover) {
  background: var(--bg-secondary);
}

.modern-file-table :deep(.el-table__empty-block) {
  background: var(--bg-primary);
  padding: 3rem 2rem;
}

.modern-file-table :deep(.el-table__empty-text) {
  color: var(--neutral-500);
  font-size: 0.875rem;
}

/* 现代化文件标题样式 */
.modern-file-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.25rem;
  border-radius: var(--radius-md);
  transition: all 0.3s ease;
  min-height: 60px;
}

.file-icon-wrapper {
  position: relative;
  flex-shrink: 0;
}

.file-icon {
  width: 2.5rem;
  height: 2.5rem;
  background: var(--accent-50);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-light);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.file-icon::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.5s;
}

.modern-file-title:hover .file-icon::before {
  left: 100%;
}

.file-icon svg {
  width: 1.25rem;
  height: 1.25rem;
  color: var(--accent-600);
  z-index: 1;
}

.file-type-badge {
  position: absolute;
  top: -0.25rem;
  right: -0.25rem;
  background: var(--accent-600);
  color: white;
  font-size: 0.625rem;
  font-weight: 700;
  padding: 0.125rem 0.375rem;
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  letter-spacing: 0.025em;
  box-shadow: var(--shadow-sm);
}

/* 文件类型样式 */
.file-type-pdf .file-icon {
  background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
  border-color: #fca5a5;
}

.file-type-pdf .file-icon svg {
  color: #dc2626;
}

.file-type-doc .file-icon {
  background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
  border-color: #93c5fd;
}

.file-type-doc .file-icon svg {
  color: #2563eb;
}

.file-type-excel .file-icon {
  background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
  border-color: #86efac;
}

.file-type-excel .file-icon svg {
  color: #16a34a;
}

.file-type-ppt .file-icon {
  background: linear-gradient(135deg, #fed7aa 0%, #fdba74 100%);
  border-color: #fb923c;
}

.file-type-ppt .file-icon svg {
  color: #ea580c;
}

.file-type-image .file-icon {
  background: linear-gradient(135deg, #fae8ff 0%, #f3e8ff 100%);
  border-color: #d8b4fe;
}

.file-type-image .file-icon svg {
  color: #9333ea;
}

.file-details {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  overflow: hidden;
}

.file-name {
  cursor: pointer;
  color: var(--neutral-900);
  font-weight: 600;
  font-size: 0.875rem;
  transition: all 0.3s ease;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 0.25rem;
  line-height: 1.4;
  width: 100%;
  max-width: 100%;
}

.file-name:hover {
  color: var(--accent-600);
}

.file-meta {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: var(--neutral-500);
  line-height: 1.2;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.file-size {
  font-weight: 600;
  color: var(--neutral-600);
}

.file-separator {
  color: var(--neutral-300);
  font-weight: bold;
}

.file-upload-time {
  font-weight: 500;
}

/* 现代化分类标签样式 */
.modern-category-tag {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: var(--accent-50);
  border: 1px solid var(--accent-200);
  border-radius: var(--radius-lg);
  padding: 0.5rem 0.75rem;
  font-weight: 600;
  color: var(--accent-700);
  font-size: 0.75rem;
}

.tag-icon svg {
  width: 0.75rem;
  height: 0.75rem;
}

.no-category {
  color: var(--neutral-400);
  font-style: italic;
  font-size: 0.75rem;
}

/* 发布机关信息样式 */
.authority-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--neutral-700);
}

.authority-icon {
  width: 1.25rem;
  height: 1.25rem;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.authority-icon svg {
  width: 0.75rem;
  height: 0.75rem;
  color: var(--neutral-500);
}

/* 现代化文件统计样式 */
.modern-file-stats {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.modern-file-stats .stat-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: var(--neutral-600);
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.modern-file-stats .stat-item:hover {
  background: var(--bg-tertiary);
  transform: scale(1.05);
}

.stat-icon {
  width: 1.25rem;
  height: 1.25rem;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-icon.view {
  background: var(--info-light);
}

.stat-icon.view svg {
  width: 0.75rem;
  height: 0.75rem;
  color: var(--info);
}

.stat-icon.download {
  background: var(--success-light);
}

.stat-icon.download svg {
  width: 0.75rem;
  height: 0.75rem;
  color: var(--success);
}

.stat-value {
  font-weight: 600;
  color: var(--neutral-700);
}

/* 时间信息样式 */
.time-info {
  font-size: 0.875rem;
  color: var(--neutral-600);
  white-space: nowrap;
}

/* 防溢出的操作按钮样式 */
.modern-action-buttons {
  display: flex;
  gap: 0.25rem;
  align-items: center;
  justify-content: flex-end;
  min-width: 0;
  flex-shrink: 0;
  flex-wrap: wrap;
}

.action-btn {
  padding: 0.375rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
  position: relative;
  overflow: hidden;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  white-space: nowrap;
  flex-shrink: 0;
}

.action-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s;
}

.action-btn:hover::before {
  left: 100%;
}

.action-btn.preview {
  background: linear-gradient(135deg, var(--primary-600) 0%, var(--accent-600) 100%);
  border-color: var(--primary-600);
  color: white;
}

.action-btn.preview:hover {
  box-shadow: var(--shadow-md);
}

.action-btn.download {
  background: var(--bg-primary);
  border: 1px solid var(--border-medium);
  color: var(--neutral-700);
}

.action-btn.download:hover {
  background: var(--success-light);
  border-color: var(--success);
  color: var(--success);
  transform: translateY(-2px);
  box-shadow: var(--shadow-sm);
}

.action-btn.more {
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  color: var(--neutral-600);
}

.action-btn.more:hover {
  background: var(--bg-tertiary);
  border-color: var(--border-medium);
  color: var(--neutral-700);
  transform: translateY(-2px);
}

/* 下拉菜单样式 */
.modern-dropdown-menu {
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
  box-shadow: var(--shadow-lg);
  padding: 0.5rem;
  background: var(--bg-primary);
}

.dropdown-item {
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
  margin-bottom: 0.25rem;
}

.dropdown-item:last-child {
  margin-bottom: 0;
}

.dropdown-item:hover {
  background: var(--bg-secondary);
}

.dropdown-item.danger:hover {
  background: var(--error-light);
  color: var(--error);
}

.dropdown-item-content {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
}

.dropdown-icon {
  font-size: 0.875rem;
}

/* 标签样式 */
.file-list :deep(.el-tag) {
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-light);
  font-weight: 500;
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
}

.file-list :deep(.el-tag--info) {
  background: var(--bg-tertiary);
  color: var(--neutral-700);
  border-color: var(--border-medium);
}

/* 按钮样式 */
.file-list :deep(.el-button) {
  border-radius: var(--radius-md);
  font-weight: 500;
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-light);
}

.file-list :deep(.el-button--primary) {
  background: var(--accent-600);
  border-color: var(--accent-600);
}

.file-list :deep(.el-button--primary:hover) {
  background: var(--accent-700);
  border-color: var(--accent-700);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 分页样式 */
.pagination-container {
  padding: 1.5rem 2rem;
  background: var(--bg-primary);
  border-top: 1px solid var(--border-light);
  display: flex;
  justify-content: flex-end;
  align-items: center;
  position: sticky;
  bottom: 0;
  backdrop-filter: blur(10px);
}

.pagination-container :deep(.el-pagination) {
  gap: 0.5rem;
}

.pagination-container :deep(.el-pager li) {
  border-radius: var(--radius-md);
  min-width: 2.5rem;
  height: 2.5rem;
  font-weight: 500;
  border: 1px solid var(--border-light);
  transition: all 0.2s ease;
}

.pagination-container :deep(.el-pager li:hover) {
  background: var(--bg-secondary);
  border-color: var(--border-medium);
}

.pagination-container :deep(.el-pager li.is-active) {
  background: var(--accent-600);
  color: white;
  border-color: var(--accent-600);
  box-shadow: var(--shadow-sm);
}

.pagination-container :deep(.btn-prev, .btn-next) {
  border-radius: var(--radius-md);
  width: 2.5rem;
  height: 2.5rem;
  border: 1px solid var(--border-light);
  transition: all 0.2s ease;
}

.pagination-container :deep(.btn-prev:hover, .btn-next:hover) {
  background: var(--bg-secondary);
  border-color: var(--border-medium);
}

/* 统计信息 */
.stats-info {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  color: var(--neutral-600);
  font-size: 0.875rem;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.stat-number {
  font-weight: 600;
  color: var(--accent-600);
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: var(--gray-500);
}

.empty-state svg {
  width: 4rem;
  height: 4rem;
  color: var(--gray-300);
  margin-bottom: 1rem;
}

.empty-state h3 {
  margin: 0 0 0.5rem 0;
  color: var(--gray-700);
}

.empty-state p {
  margin: 0;
  font-size: 0.875rem;
}

/* 响应式设计 */
/* 大屏幕优化 */
@media (max-width: 1200px) {
  .file-management-page {
    padding: 0 0.75rem;
    max-width: 100%;
  }

  .main-layout {
    gap: 0.75rem;
  }

  .page-title {
    font-size: 1.375rem;
  }

  .toolbar {
    padding: 0.75rem;
    gap: 0.75rem;
  }

  .toolbar-right {
    gap: 0.5rem;
  }
}

/* 平板端优化 - 防止水平溢出 */
@media (max-width: 1024px) {
  .file-management-page {
    padding: 0 0.5rem;
    overflow-x: hidden;
  }

  .main-layout {
    grid-template-columns: 1fr !important;
    gap: 1rem;
    overflow: hidden;
  }

  .resize-handle {
    display: none;
  }

  .sidebar {
    position: relative;
    top: 0;
    max-height: 250px;
    order: 2;
    width: 100%;
  }

  .main-content {
    order: 1;
    width: 100%;
    overflow: hidden;
  }

  .header-content {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
    padding: 1rem 0;
  }

  .header-left {
    align-items: center;
    text-align: center;
  }

  .header-actions {
    justify-content: center;
    width: 100%;
  }

  .toolbar {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
    padding: 1rem;
  }

  .toolbar-left,
  .toolbar-right {
    justify-content: center;
    width: 100%;
  }

  .search-input {
    max-width: 100%;
  }

  .filter-row {
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }

  .modern-file-table :deep(.el-table th),
  .modern-file-table :deep(.el-table td) {
    padding: 0.75rem 0.5rem;
  }
}

/* 移动端优化 - 严格防止水平溢出 */
@media (max-width: 768px) {
  .file-management-page {
    padding: 0 0.5rem;
    overflow-x: hidden;
    width: 100%;
    max-width: 100vw;
    box-sizing: border-box;
  }

  .page-header {
    margin-bottom: 0.75rem;
    width: 100%;
  }

  .header-content {
    padding: 1rem 0;
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
    width: 100%;
  }

  .header-left {
    align-items: center;
    text-align: center;
    width: 100%;
  }

  .page-title {
    font-size: 1.25rem;
    word-break: break-word;
  }

  .quick-stats {
    font-size: 0.8rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .upload-btn {
    width: 100%;
    justify-content: center;
    box-sizing: border-box;
  }

  .main-layout {
    width: 100%;
    overflow: hidden;
  }

  .main-content {
    width: 100%;
    overflow: hidden;
  }

  .toolbar {
    padding: 1rem;
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
    width: 100%;
    box-sizing: border-box;
  }

  .toolbar-left {
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
  }

  .toolbar-right {
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
  }

  .search-input {
    max-width: 100%;
    width: 100%;
  }

  .advanced-filters {
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
  }

  .filter-row {
    grid-template-columns: 1fr;
    gap: 0.75rem;
    width: 100%;
  }

  .file-list-container {
    width: 100%;
    overflow: hidden;
  }

  .file-list {
    width: 100%;
    overflow-x: auto;
  }

  .modern-file-table {
    min-width: 600px;
  }

  .modern-file-table :deep(.el-table th),
  .modern-file-table :deep(.el-table td) {
    padding: 0.75rem 0.5rem;
    min-width: 80px;
  }

  .modern-file-title {
    gap: 0.5rem;
    min-height: 50px;
    width: 100%;
  }

  .file-icon {
    width: 2rem;
    height: 2rem;
    flex-shrink: 0;
  }

  .file-details {
    min-width: 0;
    overflow: hidden;
  }

  .modern-action-buttons {
    flex-direction: column;
    gap: 0.25rem;
    min-width: 80px;
  }

  .action-btn {
    width: 100%;
    justify-content: center;
    padding: 0.375rem 0.25rem;
    font-size: 0.7rem;
    white-space: nowrap;
  }

  .sidebar {
    max-height: 220px;
    width: 100%;
  }

  .sidebar-header {
    padding: 1rem;
    min-height: 56px;
  }

  .sidebar-content {
    padding: 0.75rem;
  }

  .sidebar-content::before {
    left: 0.75rem;
    right: 0.75rem;
  }

  .category-tree {
    padding-top: 0.25rem;
  }

  .category-node {
    padding: 0.875rem 0.75rem;
    min-height: 44px;
    width: 100%;
    box-sizing: border-box;
  }

  .category-name {
    font-size: 0.875rem;
    margin-right: 0.5rem;
  }

  .category-count {
    height: 26px;
    min-width: 2.25rem;
    padding: 0.25rem 0.625rem;
    font-size: 0.75rem;
    margin-right: 0.5rem;
  }

  .category-actions {
    min-width: 65px;
    gap: 0.1rem;
  }
  
  .category-actions .el-button {
    width: 30px;
    height: 30px;
  }
}

/* 超小屏幕优化 - 320px-480px */
@media (max-width: 480px) {
  .file-management-page {
    padding: 0 0.25rem;
    overflow-x: hidden;
    width: 100%;
    max-width: 100vw;
  }

  .page-title {
    font-size: 1.125rem;
    word-break: break-word;
  }

  .quick-stats {
    font-size: 0.75rem;
    flex-direction: column;
    gap: 0.25rem;
  }

  .upload-btn {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
  }

  .toolbar {
    padding: 0.75rem;
    gap: 0.75rem;
  }

  .toolbar-left {
    gap: 0.5rem;
  }

  .toolbar-right {
    gap: 0.25rem;
  }

  .search-input {
    font-size: 0.875rem;
  }

  .search-btn,
  .filter-btn,
  .refresh-btn {
    padding: 0 0.75rem;
    font-size: 0.8rem;
  }

  .advanced-filters {
    padding: 0.75rem;
  }

  .file-list {
    border-radius: var(--radius-lg);
  }

  .modern-file-table {
    min-width: 500px;
  }

  .modern-file-table :deep(.el-table th),
  .modern-file-table :deep(.el-table td) {
    padding: 0.5rem 0.25rem;
    font-size: 0.8rem;
  }

  .modern-file-title {
    gap: 0.375rem;
    min-height: 44px;
  }

  .file-icon {
    width: 1.75rem;
    height: 1.75rem;
  }

  .file-name {
    font-size: 0.8rem;
  }

  .file-meta {
    font-size: 0.7rem;
    gap: 0.25rem;
  }

  .modern-action-buttons {
    min-width: 70px;
  }

  .action-btn {
    padding: 0.25rem;
    font-size: 0.65rem;
    height: 28px;
  }

  .sidebar {
    max-height: 200px;
  }

  .sidebar-header {
    padding: 0.75rem;
    min-height: 52px;
  }

  .sidebar-title {
    font-size: 0.8rem;
  }

  .sidebar-content {
    padding: 0.5rem;
  }

  .sidebar-content::before {
    left: 0.5rem;
    right: 0.5rem;
  }

  .category-node {
    padding: 0.75rem 0.5rem;
    min-height: 40px;
    border-radius: var(--radius-md);
  }

  .category-node::before {
    width: 2px;
  }

  .category-name {
    font-size: 0.8rem;
    margin-right: 0.375rem;
  }

  .category-count {
    font-size: 0.7rem;
    padding: 0.25rem 0.5rem;
    height: 22px;
    min-width: 2rem;
    margin-right: 0.375rem;
  }

  .category-actions {
    min-width: 60px;
    gap: 0.1rem;
  }
  
  .category-actions .el-button {
    width: 28px;
    height: 28px;
  }

  .category-actions .el-button .el-icon {
    font-size: 0.8rem;
  }
}

/* 极小屏幕优化 - 320px */
@media (max-width: 320px) {
  .file-management-page {
    padding: 0 0.125rem;
  }

  .page-title {
    font-size: 1rem;
  }

  .toolbar {
    padding: 0.5rem;
  }

  .modern-file-table {
    min-width: 450px;
  }

  .modern-file-table :deep(.el-table th),
  .modern-file-table :deep(.el-table td) {
    padding: 0.375rem 0.125rem;
    font-size: 0.75rem;
  }

  .action-btn {
    font-size: 0.6rem;
    padding: 0.125rem;
    height: 24px;
  }

  /* 极小屏幕分类优化 */
  .sidebar {
    max-height: 160px;
  }

  .sidebar-header {
    padding: 0.5rem;
    min-height: 44px;
  }

  .sidebar-title {
    font-size: 0.75rem;
  }

  .sidebar-content {
    padding: 0.375rem;
  }

  .category-node {
    padding: 0.5rem 0.375rem;
    min-height: 36px;
  }

  .category-name {
    font-size: 0.75rem;
    margin-right: 0.25rem;
  }

  .category-count {
    font-size: 0.65rem;
    padding: 0.125rem 0.375rem;
    height: 20px;
    min-width: 1.75rem;
    margin-right: 0.25rem;
  }

  .category-actions {
    min-width: 55px;
    gap: 0.05rem;
  }
  
  .category-actions .el-button {
    width: 24px;
    height: 24px;
  }

  .category-actions .el-button .el-icon {
    font-size: 0.75rem;
  }
}

/* 加载状态 */
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
  z-index: 1000;
}

/* 动画效果 */
.file-card-enter-active {
  transition: all 0.3s ease;
}

.file-card-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.file-card-leave-active {
  transition: all 0.2s ease;
}

.file-card-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* Element Plus 组件自定义 */
.file-list :deep(.el-loading-mask) {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(4px);
}

.file-list :deep(.el-loading-spinner) {
  margin-top: -1rem;
}

.file-list :deep(.el-loading-text) {
  color: var(--gray-600);
  font-weight: 500;
}

/* 树形组件样式 */
.category-tree-container :deep(.el-tree) {
  background: transparent;
  color: var(--gray-700);
}

.category-tree-container :deep(.el-tree-node__content) {
  padding: 0.5rem 0;
  border-radius: var(--radius-md);
  transition: background 0.2s ease;
}

.category-tree-container :deep(.el-tree-node__content:hover) {
  background: var(--gray-50);
}

.category-tree-container :deep(.el-tree-node.is-current > .el-tree-node__content) {
  background: var(--primary-50);
  color: var(--primary-700);
  font-weight: 600;
}

.category-tree-container :deep(.el-tree-node__expand-icon) {
  color: var(--gray-400);
}

.category-tree-container :deep(.el-tree-node__expand-icon.is-leaf) {
  color: transparent;
}
</style>
