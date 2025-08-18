package com.example.dto;

import lombok.Data;

/**
 * 移动分类请求DTO
 */
@Data
public class CategoryMoveRequest {
    
    private Long newParentId;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public Long getNewParentId() { return newParentId; }
    public void setNewParentId(Long newParentId) { this.newParentId = newParentId; }
}