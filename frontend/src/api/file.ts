import { tauriInvoke } from './request'

// 文件上传（桌面应用通过文件路径上传）
export const uploadFile = (params: {
  filePath: string
  title?: string
  description?: string
  categoryId?: number
  tags?: string[]
  documentNumber?: string
  issueDate?: string
  effectiveDate?: string
  issuingAuthority?: string
  isPublic?: boolean
}) => {
  return tauriInvoke('upload_file', {
    filePath: params.filePath,
    title: params.title || null,
    description: params.description || null,
    categoryId: params.categoryId || null,
    tags: params.tags || null,
    documentNumber: params.documentNumber || null,
    issueDate: params.issueDate || null,
    effectiveDate: params.effectiveDate || null,
    issuingAuthority: params.issuingAuthority || null,
    isPublic: params.isPublic ?? true,
  })
}

// 批量上传文件
export const batchUploadFiles = (filePaths: string[], categoryId?: number) => {
  return tauriInvoke('batch_upload_files', {
    filePaths,
    categoryId: categoryId || null,
  })
}

// 获取文件列表
export const getFileList = (params: any) => {
  return tauriInvoke('get_files', {
    keyword: params.keyword || null,
    categoryId: params.categoryId || null,
    tags: params.tags && params.tags.length > 0 ? params.tags : null,
    startDate: params.startDate || null,
    endDate: params.endDate || null,
    issuingAuthority: params.issuingAuthority || null,
    isPublic: params.isPublic ?? null,
    page: params.page ?? 0,
    size: params.size ?? 10,
    sort: params.sort || null,
    direction: params.direction || null,
  })
}

// 获取文件详情
export const getFileDetail = (id: number) => {
  return tauriInvoke('get_file_by_id', { id })
}

// 更新文件信息
export const updateFile = (id: number, data: any) => {
  return tauriInvoke('update_file', {
    id,
    title: data.title || null,
    originalName: data.originalName || null,
    description: data.description || null,
    categoryId: data.categoryId || null,
    tags: data.tags && data.tags.length > 0 ? data.tags : null,
    documentNumber: data.documentNumber || null,
    issueDate: data.issueDate || null,
    effectiveDate: data.effectiveDate || null,
    issuingAuthority: data.issuingAuthority || null,
    isPublic: data.isPublic ?? null,
  })
}

// 删除文件
export const deleteFile = (id: number) => {
  return tauriInvoke('delete_file', { id })
}

// 获取文件预览信息
export const getFilePreview = (id: number) => {
  return tauriInvoke('get_file_preview_info', { id })
}

// 预览文件（返回内容）
export const previewFile = (id: number) => {
  return tauriInvoke('preview_file', { id })
}

// 获取文件路径（用于下载/打开）
export const getFilePath = (id: number) => {
  return tauriInvoke('get_file_path', { id })
}

// 用系统默认应用打开文件
export const openFileWithSystem = (id: number) => {
  return tauriInvoke('open_file_with_system', { id })
}

// 打开文件所在目录
export const openFileFolder = (id: number) => {
  return tauriInvoke('open_file_folder', { id })
}

// 获取文件内容（文本文件）
export const getFileContent = (id: number) => {
  return tauriInvoke('get_file_content', { id })
}

// 获取热门文件
export const getPopularFiles = () => {
  return tauriInvoke('get_popular_files', {})
}

// 获取最新文件
export const getLatestFiles = () => {
  return tauriInvoke('get_latest_files', {})
}

// 获取我的文件
export const getMyFiles = (params: any) => {
  return tauriInvoke('get_my_files', {
    page: params.page ?? 0,
    size: params.size ?? 10,
  })
}

// 获取文件分类
export const getCategories = () => {
  return tauriInvoke('get_categories', {})
}

// 获取顶级分类
export const getRootCategories = () => {
  return tauriInvoke('get_root_categories', {})
}

// 获取子分类
export const getChildCategories = (parentId: number) => {
  return tauriInvoke('get_child_categories', { parentId })
}

// 创建分类
export const createCategory = (data: any) => {
  return tauriInvoke('create_category', {
    name: data.name,
    description: data.description || null,
    parentId: data.parentId || null,
    sortOrder: data.sortOrder ?? 0,
  })
}

// 更新分类
export const updateCategory = (id: number, data: any) => {
  return tauriInvoke('update_category', {
    id,
    name: data.name || null,
    description: data.description || null,
    parentId: data.parentId || null,
    sortOrder: data.sortOrder ?? null,
    enabled: data.enabled ?? null,
  })
}

// 删除分类
export const deleteCategory = (id: number) => {
  return tauriInvoke('delete_category', { id })
}

// 移动分类
export const moveCategory = (id: number, newParentId?: number) => {
  return tauriInvoke('move_category', {
    id,
    newParentId: newParentId || null,
  })
}

// 获取分类统计
export const getCategoryStats = (id: number) => {
  return tauriInvoke('get_category_stats', { id })
}

// 获取所有标签
export const getAllTags = () => {
  return tauriInvoke('get_all_tags', {})
}

// ====== 数据导入导出 ======

// 导出数据
export const exportData = (params: {
  fileIds: number[]
  categoryIds: number[]
  exportPath: string
}) => {
  return tauriInvoke('export_data', {
    fileIds: params.fileIds,
    categoryIds: params.categoryIds,
    exportPath: params.exportPath,
  })
}

// 预览导入
export const previewImport = (importPath: string) => {
  return tauriInvoke('preview_import', { importPath })
}

// 导入数据
export const importData = (params: {
  importPath: string
  strategy: string
}) => {
  return tauriInvoke('import_data', {
    importPath: params.importPath,
    strategy: params.strategy,
  })
}
