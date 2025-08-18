package com.example.config;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

import java.util.Arrays;
import java.util.List;

/**
 * 文件存储配置
 */
@Data
@Configuration
@ConfigurationProperties(prefix = "file.storage")
public class FileStorageConfig {
    
    /**
     * 文件上传根目录
     */
    private String uploadDir = "uploads";
    
    /**
     * 最大文件大小（字节）默认100MB
     */
    private Long maxFileSize = 100 * 1024 * 1024L;
    
    /**
     * 最大请求大小（字节）默认100MB
     */
    private Long maxRequestSize = 100 * 1024 * 1024L;
    
    /**
     * 允许上传的文件类型
     */
    private List<String> allowedTypes = Arrays.asList(
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "txt", "md", "html", "htm", "xml", "json",
        "jpg", "jpeg", "png", "gif", "bmp", "webp",
        "mp4", "avi", "mov", "wmv", "flv", "mkv",
        "mp3", "wav", "flac", "aac", "ogg", "zip", "rar", "7z"
    );
    
    /**
     * 文件名生成策略
     */
    private String fileNameStrategy = "uuid"; // uuid, timestamp, original
    
    /**
     * 是否保留原始文件名
     */
    private Boolean keepOriginalName = false;
    
    /**
     * 文件存储策略
     */
    private String storageStrategy = "local"; // local, oss, minio
    
    /**
     * 按日期分目录存储
     */
    private Boolean dateFolder = true;
    
    // Getter and Setter methods (手动添加以解决Lombok编译问题)
    public String getUploadDir() { return uploadDir; }
    public void setUploadDir(String uploadDir) { this.uploadDir = uploadDir; }
    
    public Long getMaxFileSize() { return maxFileSize; }
    public void setMaxFileSize(Long maxFileSize) { this.maxFileSize = maxFileSize; }
    
    public Long getMaxRequestSize() { return maxRequestSize; }
    public void setMaxRequestSize(Long maxRequestSize) { this.maxRequestSize = maxRequestSize; }
    
    public List<String> getAllowedTypes() { return allowedTypes; }
    public void setAllowedTypes(List<String> allowedTypes) { this.allowedTypes = allowedTypes; }
    
    public String getFileNameStrategy() { return fileNameStrategy; }
    public void setFileNameStrategy(String fileNameStrategy) { this.fileNameStrategy = fileNameStrategy; }
    
    public Boolean getKeepOriginalName() { return keepOriginalName; }
    public void setKeepOriginalName(Boolean keepOriginalName) { this.keepOriginalName = keepOriginalName; }
    
    public String getStorageStrategy() { return storageStrategy; }
    public void setStorageStrategy(String storageStrategy) { this.storageStrategy = storageStrategy; }
    
    public Boolean getDateFolder() { return dateFolder; }
    public void setDateFolder(Boolean dateFolder) { this.dateFolder = dateFolder; }
    
    /**
     * 检查文件类型是否允许
     */
    public boolean isAllowedType(String fileType) {
        return allowedTypes.contains(fileType.toLowerCase());
    }
    
    /**
     * 检查文件大小是否超限
     */
    public boolean isFileSizeValid(long fileSize) {
        return fileSize <= maxFileSize;
    }
}
