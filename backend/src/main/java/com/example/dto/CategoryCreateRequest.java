package com.example.dto;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Data;

/**
 * 创建分类请求DTO
 */
@Data
public class CategoryCreateRequest {
    
    @NotBlank(message = "分类名称不能为空")
    @Size(min = 1, max = 50, message = "分类名称长度在1-50个字符")
    private String name;
    
    @Size(max = 200, message = "描述长度不能超过200个字符")
    private String description;
    
    private Long parentId;
    
    private Integer sortOrder;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public Long getParentId() { return parentId; }
    public void setParentId(Long parentId) { this.parentId = parentId; }
    
    public Integer getSortOrder() { return sortOrder; }
    public void setSortOrder(Integer sortOrder) { this.sortOrder = sortOrder; }
}