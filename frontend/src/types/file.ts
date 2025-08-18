export interface PolicyFile {
  id: number
  title: string
  originalName: string
  fileName: string
  filePath: string
  fileSize: number
  fileType: string
  mimeType?: string
  description?: string
  documentNumber?: string
  issueDate?: string
  effectiveDate?: string
  issuingAuthority?: string
  downloadCount: number
  viewCount: number
  isPublic: boolean
  enabled: boolean
  createTime: string
  updateTime?: string
  createdBy: number
  updatedBy?: number
  category?: FileCategory
  tags?: FileTag[]
  creator?: {
    id: number
    username: string
  }
}

export interface FileCategory {
  id: number
  name: string
  description?: string
  parentId?: number
  sortOrder: number
  enabled: boolean
  createTime: string
  updateTime?: string
  createdBy: number
  children?: FileCategory[]
  files?: PolicyFile[]
  fileCount?: number
}

export interface FileTag {
  id: number
  name: string
  color?: string
  description?: string
  createTime: string
  createdBy: number
}

export interface FileUploadForm {
  title?: string
  description?: string
  categoryId?: number
  tags?: string[]
  documentNumber?: string
  issueDate?: string
  effectiveDate?: string
  issuingAuthority?: string
}

export interface FileSearchParams {
  keyword?: string
  categoryId?: number
  tags?: string[]
  startDate?: string
  endDate?: string
  issuingAuthority?: string
  isPublic?: boolean
  page?: number
  size?: number
  sort?: string
  direction?: string
}

export interface FilePreviewInfo {
  id: number
  title: string
  fileName: string
  fileType: string
  previewType: string
  fileSize: number
  canPreview: boolean
  previewUrl: string
  downloadUrl: string
}

export interface CategoryForm {
  name: string
  description?: string
  parentId?: number
  sortOrder?: number
}

export interface FileStats {
  totalFiles: number
  totalSize: number
  categoryStats: {
    categoryId: number
    categoryName: string
    fileCount: number
  }[]
  popularFiles: PolicyFile[]
  latestFiles: PolicyFile[]
}
