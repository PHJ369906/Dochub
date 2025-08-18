package com.example.entity;

import com.baomidou.mybatisplus.annotation.*;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import lombok.Data;

import java.time.LocalDateTime;
import java.util.List;

/**
 * 文件分类实体
 */
@Data
@TableName("file_category")
@JsonIgnoreProperties({"hibernateLazyInitializer", "handler"})
public class FileCategory {
    
    @TableId(value = "id", type = IdType.AUTO)
    private Long id;

    @TableField("name")
    private String name;

    @TableField("description")
    private String description;

    @TableField("parent_id")
    private Long parentId;

    @TableField("sort_order")
    private Integer sortOrder = 0;

    @TableField("is_enabled")
    private Boolean enabled = true;

    @TableField(value = "create_time", fill = FieldFill.INSERT)
    private LocalDateTime createTime;

    @TableField(value = "update_time", fill = FieldFill.INSERT_UPDATE)
    private LocalDateTime updateTime;

    @TableField("created_by")
    private Long createdBy;

    // 子分类（关联对象，不映射到数据库）
    @TableField(exist = false)
    @JsonIgnore
    private List<FileCategory> children;

    // 该分类下的文件（关联对象，不映射到数据库）
    @TableField(exist = false)
    @JsonIgnore
    private List<PolicyFile> files;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public Long getParentId() { return parentId; }
    public void setParentId(Long parentId) { this.parentId = parentId; }
    
    public Integer getSortOrder() { return sortOrder; }
    public void setSortOrder(Integer sortOrder) { this.sortOrder = sortOrder; }
    
    public Boolean getEnabled() { return enabled; }
    public void setEnabled(Boolean enabled) { this.enabled = enabled; }
    
    public LocalDateTime getCreateTime() { return createTime; }
    public void setCreateTime(LocalDateTime createTime) { this.createTime = createTime; }
    
    public LocalDateTime getUpdateTime() { return updateTime; }
    public void setUpdateTime(LocalDateTime updateTime) { this.updateTime = updateTime; }
    
    public Long getCreatedBy() { return createdBy; }
    public void setCreatedBy(Long createdBy) { this.createdBy = createdBy; }
    
    public List<FileCategory> getChildren() { return children; }
    public void setChildren(List<FileCategory> children) { this.children = children; }
    
    public List<PolicyFile> getFiles() { return files; }
    public void setFiles(List<PolicyFile> files) { this.files = files; }
}
