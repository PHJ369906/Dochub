package com.example.dto;

import jakarta.validation.constraints.Size;
import lombok.Data;

/**
 * 更新分类请求DTO
 */
@Data
public class CategoryUpdateRequest {
    
    @Size(min = 1, max = 50, message = "分类名称长度在1-50个字符")
    private String name;
    
    @Size(max = 200, message = "描述长度不能超过200个字符")
    private String description;
    
    private Integer sortOrder;
    
    private Boolean enabled;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public Integer getSortOrder() { return sortOrder; }
    public void setSortOrder(Integer sortOrder) { this.sortOrder = sortOrder; }
    
    public Boolean getEnabled() { return enabled; }
    public void setEnabled(Boolean enabled) { this.enabled = enabled; }
}