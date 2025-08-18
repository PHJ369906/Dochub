package com.example.entity;

import com.baomidou.mybatisplus.annotation.*;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import lombok.Data;

import java.time.LocalDateTime;
import java.util.Set;

/**
 * 文件标签实体
 */
@Data
@TableName("file_tag")
@JsonIgnoreProperties({"hibernateLazyInitializer", "handler"})
public class FileTag {
    
    @TableId(value = "id", type = IdType.AUTO)
    private Long id;

    @TableField("name")
    private String name;

    @TableField("color")
    private String color; // 标签颜色，如 #FF5722

    @TableField("description")
    private String description;

    @TableField(value = "create_time", fill = FieldFill.INSERT)
    private LocalDateTime createTime;

    @TableField("created_by")
    private Long createdBy;

    // 使用该标签的文件（关联对象，不映射到数据库）
    @TableField(exist = false)
    private Set<PolicyFile> files;

    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    
    public String getColor() { return color; }
    public void setColor(String color) { this.color = color; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public LocalDateTime getCreateTime() { return createTime; }
    public void setCreateTime(LocalDateTime createTime) { this.createTime = createTime; }
    
    public Long getCreatedBy() { return createdBy; }
    public void setCreatedBy(Long createdBy) { this.createdBy = createdBy; }
    
    public Set<PolicyFile> getFiles() { return files; }
    public void setFiles(Set<PolicyFile> files) { this.files = files; }
}
