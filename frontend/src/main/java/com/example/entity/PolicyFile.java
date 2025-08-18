package com.example.entity;

import com.baomidou.mybatisplus.annotation.*;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import java.time.LocalDateTime;
import java.util.Set;

/**
 * 政策法规文件实体
 */
@TableName("policy_file")
@JsonIgnoreProperties({"hibernateLazyInitializer", "handler"})
public class PolicyFile {
    
    @TableId(value = "id", type = IdType.AUTO)
    private Long id;

    @TableField("title")
    private String title; // 文件标题

    @TableField("original_name")
    private String originalName; // 原始文件名

    @TableField("file_name")
    private String fileName; // 存储文件名

    @TableField("file_path")
    private String filePath; // 文件存储路径

    @TableField("file_size")
    private Long fileSize; // 文件大小（字节）

    @TableField("file_type")
    private String fileType; // 文件类型（扩展名）

    @TableField("mime_type")
    private String mimeType; // MIME类型

    @TableField("description")
    private String description; // 文件描述

    @TableField("document_number")
    private String documentNumber; // 文件编号

    @TableField("issue_date")
    private LocalDateTime issueDate; // 发布日期

    @TableField("effective_date")
    private LocalDateTime effectiveDate; // 生效日期

    @TableField("issuing_authority")
    private String issuingAuthority; // 发布机关

    @TableField("download_count")
    private Integer downloadCount = 0; // 下载次数

    @TableField("view_count")
    private Integer viewCount = 0; // 查看次数

    @TableField("is_public")
    private Boolean isPublic = true; // 是否公开

    @TableField("is_enabled")
    private Boolean enabled = true; // 是否启用

    @TableField(value = "create_time", fill = FieldFill.INSERT)
    private LocalDateTime createTime;

    @TableField(value = "update_time", fill = FieldFill.INSERT_UPDATE)
    private LocalDateTime updateTime;

    @TableField("created_by")
    private Long createdBy;

    @TableField("updated_by")
    private Long updatedBy;
    
    // 分类ID（外键）
    @TableField("category_id")
    private Long categoryId;

    // 文件分类（关联对象，不映射到数据库）
    @TableField(exist = false)
    @JsonIgnore
    private FileCategory category;

    // 文件标签（关联对象，不映射到数据库）
    @TableField(exist = false)
    private Set<FileTag> tags;

    // 上传用户（关联对象，不映射到数据库）
    @TableField(exist = false)
    private User creator;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    
    public String getTitle() { return title; }
    public void setTitle(String title) { this.title = title; }
    
    public String getOriginalName() { return originalName; }
    public void setOriginalName(String originalName) { this.originalName = originalName; }
    
    public String getFileName() { return fileName; }
    public void setFileName(String fileName) { this.fileName = fileName; }
    
    public String getFilePath() { return filePath; }
    public void setFilePath(String filePath) { this.filePath = filePath; }
    
    public Long getFileSize() { return fileSize; }
    public void setFileSize(Long fileSize) { this.fileSize = fileSize; }
    
    public String getFileType() { return fileType; }
    public void setFileType(String fileType) { this.fileType = fileType; }
    
    public String getMimeType() { return mimeType; }
    public void setMimeType(String mimeType) { this.mimeType = mimeType; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public String getDocumentNumber() { return documentNumber; }
    public void setDocumentNumber(String documentNumber) { this.documentNumber = documentNumber; }
    
    public LocalDateTime getIssueDate() { return issueDate; }
    public void setIssueDate(LocalDateTime issueDate) { this.issueDate = issueDate; }
    
    public LocalDateTime getEffectiveDate() { return effectiveDate; }
    public void setEffectiveDate(LocalDateTime effectiveDate) { this.effectiveDate = effectiveDate; }
    
    public String getIssuingAuthority() { return issuingAuthority; }
    public void setIssuingAuthority(String issuingAuthority) { this.issuingAuthority = issuingAuthority; }
    
    public Integer getDownloadCount() { return downloadCount; }
    public void setDownloadCount(Integer downloadCount) { this.downloadCount = downloadCount; }
    
    public Integer getViewCount() { return viewCount; }
    public void setViewCount(Integer viewCount) { this.viewCount = viewCount; }
    
    public Boolean getIsPublic() { return isPublic; }
    public void setIsPublic(Boolean isPublic) { this.isPublic = isPublic; }
    
    public Boolean getEnabled() { return enabled; }
    public void setEnabled(Boolean enabled) { this.enabled = enabled; }
    
    public LocalDateTime getCreateTime() { return createTime; }
    public void setCreateTime(LocalDateTime createTime) { this.createTime = createTime; }
    
    public LocalDateTime getUpdateTime() { return updateTime; }
    public void setUpdateTime(LocalDateTime updateTime) { this.updateTime = updateTime; }
    
    public Long getCreatedBy() { return createdBy; }
    public void setCreatedBy(Long createdBy) { this.createdBy = createdBy; }
    
    public Long getUpdatedBy() { return updatedBy; }
    public void setUpdatedBy(Long updatedBy) { this.updatedBy = updatedBy; }

    public Long getCategoryId() { return categoryId; }
    public void setCategoryId(Long categoryId) { this.categoryId = categoryId; }

    public FileCategory getCategory() { return category; }
    public void setCategory(FileCategory category) { this.category = category; }
    
    public Set<FileTag> getTags() { return tags; }
    public void setTags(Set<FileTag> tags) { this.tags = tags; }
    
    public User getCreator() { return creator; }
    public void setCreator(User creator) { this.creator = creator; }
}
