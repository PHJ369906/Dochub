import request from './request'

// 文件上传
export const uploadFile = (formData: FormData) => {
  return request.post('/files/upload', formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
      'Accept': 'application/json'
    }
  })
}

// 批量上传文件
export const batchUploadFiles = (formData: FormData) => {
  return request.post('/files/batch-upload', formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
      'Accept': 'application/json'
    }
  })
}

// 获取文件列表
export const getFileList = (params: any) => {
  return request.get('/files', { params })
}

// 获取文件详情
export const getFileDetail = (id: number) => {
  return request.get(`/files/${id}`)
}

// 更新文件信息
export const updateFile = (id: number, data: any) => {
  return request.put(`/files/${id}`, null, { params: data })
}

// 删除文件
export const deleteFile = (id: number) => {
  return request.delete(`/files/${id}`)
}

// 获取文件预览信息
export const getFilePreview = (id: number) => {
  return request.get(`/preview/info/${id}`)
}

// 获取PDF页面预览
export const getPdfPagePreview = (id: number, pageIndex: number, dpi = 150) => {
  return `/api/preview/pdf/${id}/page/${pageIndex}?dpi=${dpi}`
}

// 获取Office文档预览
export const getOfficePreview = (id: number) => {
  return `/api/preview/office/${id}`
}

// 获取图片预览
export const getImagePreview = (id: number, width?: number, height?: number) => {
  let url = `/api/preview/image/${id}`
  if (width && height) {
    url += `?width=${width}&height=${height}`
  }
  return url
}

// 获取文本预览
export const getTextPreview = (id: number, asHtml = true, maxLines = 1000) => {
  return `/api/preview/text/${id}?asHtml=${asHtml}&maxLines=${maxLines}`
}

// 获取文件缩略图
export const getFileThumbnail = (id: number, width = 200, height = 200) => {
  return `/api/preview/thumbnail/${id}?width=${width}&height=${height}`
}

// 获取热门文件
export const getPopularFiles = () => {
  return request.get('/files/popular')
}

// 获取最新文件
export const getLatestFiles = () => {
  return request.get('/files/latest')
}

// 获取我的文件
export const getMyFiles = (params: any) => {
  return request.get('/files/my', { params })
}

// 获取文件分类
export const getCategories = () => {
  return request.get('/categories')
}

// 获取顶级分类
export const getRootCategories = () => {
  return request.get('/categories/root')
}

// 获取子分类
export const getChildCategories = (parentId: number) => {
  return request.get(`/categories/${parentId}/children`)
}

// 创建分类
export const createCategory = (data: any) => {
  return request.post('/categories', data)
}

// 更新分类
export const updateCategory = (id: number, data: any) => {
  return request.put(`/categories/${id}`, data)
}

// 删除分类
export const deleteCategory = (id: number) => {
  return request.delete(`/categories/${id}`)
}

// 移动分类
export const moveCategory = (id: number, newParentId?: number) => {
  return request.put(`/categories/${id}/move`, { newParentId })
}

// 获取分类统计
export const getCategoryStats = (id: number) => {
  return request.get(`/categories/${id}/stats`)
}
