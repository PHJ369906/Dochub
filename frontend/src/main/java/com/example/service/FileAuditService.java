package com.example.service;

import cn.dev33.satoken.stp.StpUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

/**
 * 文件操作审计服务
 */
@Service
public class FileAuditService {
    
    private static final Logger auditLogger = LoggerFactory.getLogger("FILE_AUDIT");
    private static final DateTimeFormatter FORMATTER = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
    
    /**
     * 记录文件上传审计日志
     */
    public void logFileUpload(String fileName, long fileSize, String filePath, boolean success, String errorMsg) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        if (success) {
            auditLogger.info("文件上传成功 - 时间: {}, 用户: {}, 文件名: {}, 大小: {}, 路径: {}", 
                timestamp, userId, fileName, formatFileSize(fileSize), filePath);
        } else {
            auditLogger.warn("文件上传失败 - 时间: {}, 用户: {}, 文件名: {}, 错误: {}", 
                timestamp, userId, fileName, errorMsg);
        }
    }
    
    /**
     * 记录文件下载审计日志
     */
    public void logFileDownload(Long fileId, String fileName, boolean success, String errorMsg) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        if (success) {
            auditLogger.info("文件下载成功 - 时间: {}, 用户: {}, 文件ID: {}, 文件名: {}", 
                timestamp, userId, fileId, fileName);
        } else {
            auditLogger.warn("文件下载失败 - 时间: {}, 用户: {}, 文件ID: {}, 错误: {}", 
                timestamp, userId, fileId, errorMsg);
        }
    }
    
    /**
     * 记录文件删除审计日志
     */
    public void logFileDelete(Long fileId, String fileName, boolean success, String errorMsg) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        if (success) {
            auditLogger.info("文件删除成功 - 时间: {}, 用户: {}, 文件ID: {}, 文件名: {}", 
                timestamp, userId, fileId, fileName);
        } else {
            auditLogger.warn("文件删除失败 - 时间: {}, 用户: {}, 文件ID: {}, 错误: {}", 
                timestamp, userId, fileId, errorMsg);
        }
    }
    
    /**
     * 记录文件预览审计日志
     */
    public void logFilePreview(Long fileId, String fileName, String previewType) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        auditLogger.info("文件预览 - 时间: {}, 用户: {}, 文件ID: {}, 文件名: {}, 预览类型: {}", 
            timestamp, userId, fileId, fileName, previewType);
    }
    
    /**
     * 记录文件更新审计日志
     */
    public void logFileUpdate(Long fileId, String fileName, String updateFields, boolean success, String errorMsg) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        if (success) {
            auditLogger.info("文件更新成功 - 时间: {}, 用户: {}, 文件ID: {}, 文件名: {}, 更新字段: {}", 
                timestamp, userId, fileId, fileName, updateFields);
        } else {
            auditLogger.warn("文件更新失败 - 时间: {}, 用户: {}, 文件ID: {}, 错误: {}", 
                timestamp, userId, fileId, errorMsg);
        }
    }
    
    /**
     * 记录权限拒绝审计日志
     */
    public void logAccessDenied(String operation, Long fileId, String fileName, String reason) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        auditLogger.warn("访问被拒绝 - 时间: {}, 用户: {}, 操作: {}, 文件ID: {}, 文件名: {}, 原因: {}", 
            timestamp, userId, operation, fileId, fileName, reason);
    }
    
    /**
     * 记录安全事件审计日志
     */
    public void logSecurityEvent(String eventType, String description, String details) {
        String userId = getCurrentUserId();
        String timestamp = LocalDateTime.now().format(FORMATTER);
        
        auditLogger.error("安全事件 - 时间: {}, 用户: {}, 事件类型: {}, 描述: {}, 详情: {}", 
            timestamp, userId, eventType, description, details);
    }
    
    /**
     * 获取当前用户ID
     */
    private String getCurrentUserId() {
        try {
            if (StpUtil.isLogin()) {
                return StpUtil.getLoginIdAsString();
            }
            return "anonymous";
        } catch (Exception e) {
            return "unknown";
        }
    }
    
    /**
     * 格式化文件大小
     */
    private String formatFileSize(long size) {
        if (size < 1024) {
            return size + " B";
        } else if (size < 1024 * 1024) {
            return String.format("%.1f KB", size / 1024.0);
        } else if (size < 1024 * 1024 * 1024) {
            return String.format("%.1f MB", size / (1024.0 * 1024.0));
        } else {
            return String.format("%.1f GB", size / (1024.0 * 1024.0 * 1024.0));
        }
    }
}