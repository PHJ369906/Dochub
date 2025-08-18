package com.example.dto;

import com.example.entity.User;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Data;

import java.time.LocalDateTime;

/**
 * 用户响应DTO
 */
@Data
@JsonInclude(JsonInclude.Include.ALWAYS)
public class UserResponse {
    
    private Long id;
    private String username;
    private String email;
    private String role;
    private String avatar;
    
    @JsonProperty("enabled")
    private Boolean enabled;
    private LocalDateTime createTime;
    
    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    
    public String getUsername() { return username; }
    public void setUsername(String username) { this.username = username; }
    
    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }
    
    public String getRole() { return role; }
    public void setRole(String role) { this.role = role; }
    
    public String getAvatar() { return avatar; }
    public void setAvatar(String avatar) { this.avatar = avatar; }
    
    public Boolean getEnabled() { return enabled; }
    public void setEnabled(Boolean enabled) { this.enabled = enabled; }
    
    public LocalDateTime getCreateTime() { return createTime; }
    public void setCreateTime(LocalDateTime createTime) { this.createTime = createTime; }
    
    public static UserResponse fromUser(User user) {
        UserResponse response = new UserResponse();
        response.setId(user.getId());
        response.setUsername(user.getUsername());
        response.setEmail(user.getEmail());
        response.setRole(user.getRole());
        response.setAvatar(user.getAvatar());
        // 确保enabled字段不为null，默认为true
        response.setEnabled(user.getEnabled() != null ? user.getEnabled() : true);
        response.setCreateTime(user.getCreateTime());
        return response;
    }
}
