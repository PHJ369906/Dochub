package com.example.service;

import cn.dev33.satoken.stp.StpUtil;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.example.entity.FileCategory;
import com.example.entity.FileTag;
import com.example.entity.PolicyFile;
import com.example.repository.FileCategoryMapper;
import com.example.repository.FileTagMapper;
import com.example.repository.PolicyFileMapper;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.util.StringUtils;
import org.springframework.web.multipart.MultipartFile;

import java.time.LocalDateTime;
import java.util.*;

/**
 * 政策文件服务 - 简化版本
 */
@Service
public class PolicyFileService {
    
    private static final Logger log = LoggerFactory.getLogger(PolicyFileService.class);
    
    @Autowired
    private PolicyFileMapper fileMapper;
    
    @Autowired
    private FileCategoryMapper categoryMapper;
    
    @Autowired
    private FileTagMapper tagMapper;
    
    @Autowired
    private FileStorageService storageService;
    
    @Autowired
    private FileAuditService auditService;
    
    /**
     * 上传文件
     */
    @Transactional
    public PolicyFile uploadFile(MultipartFile file, String title, String description, 
                                Long categoryId, Set<String> tagNames, String documentNumber,
                                LocalDateTime issueDate, LocalDateTime effectiveDate, 
                                String issuingAuthority) {
        String filePath = null;
        try {
            // 存储文件
            filePath = storageService.storeFile(file);
            
            // 创建简化的文件记录，避免复杂映射
            PolicyFile policyFile = new PolicyFile();
            // 只设置基本必需字段
            policyFile.setTitle(title != null ? title : file.getOriginalFilename());
            policyFile.setOriginalName(file.getOriginalFilename());
            
            // 确保存储文件名不为null，使用安全的文件名提取方式
            String storedFileName;
            try {
                storedFileName = java.nio.file.Paths.get(filePath).getFileName().toString();
            } catch (Exception e) {
                // 如果Path操作失败，使用字符串操作提取文件名
                log.warn("无法使用Path提取文件名，使用字符串方式: {}", e.getMessage());
                storedFileName = extractFileNameFromPath(filePath);
            }
            policyFile.setFileName(storedFileName != null && !storedFileName.isEmpty() ? storedFileName : file.getOriginalFilename());
            
            policyFile.setFilePath(filePath);
            policyFile.setFileSize(file.getSize());
            
            // 确保文件类型不为null
            String fileType = storageService.getFileExtension(file.getOriginalFilename());
            policyFile.setFileType(fileType != null && !fileType.isEmpty() ? fileType : "unknown");
            
            // 确保MIME类型不为null
            String mimeType = file.getContentType();
            policyFile.setMimeType(mimeType != null ? mimeType : "application/octet-stream");
            
            policyFile.setCreatedBy(StpUtil.getLoginIdAsLong());
            policyFile.setEnabled(true);
            policyFile.setIsPublic(true);
            policyFile.setDownloadCount(0);
            policyFile.setViewCount(0);
            
            // 设置其他可选字段为null而不是复杂对象
            policyFile.setDescription(description);
            policyFile.setCategoryId(categoryId);
            policyFile.setDocumentNumber(documentNumber);
            policyFile.setIssueDate(issueDate);
            policyFile.setEffectiveDate(effectiveDate);
            policyFile.setIssuingAuthority(issuingAuthority);
            
            // 使用MyBatis-Plus的BaseMapper insert方法
            int insertResult = fileMapper.insert(policyFile);
            PolicyFile result = insertResult > 0 ? policyFile : null;
            
            // 记录审计日志
            auditService.logFileUpload(file.getOriginalFilename(), file.getSize(), filePath, true, null);
            
            return result;
        } catch (Exception e) {
            log.error("文件上传失败: {}", e.getMessage());
            
            // 清理已上传的文件
            if (filePath != null) {
                try {
                    storageService.deleteFile(filePath);
                } catch (Exception cleanupException) {
                    log.error("清理文件失败: {}", cleanupException.getMessage());
                }
            }
            
            // 记录审计日志
            auditService.logFileUpload(file.getOriginalFilename(), file.getSize(), filePath, false, e.getMessage());
            
            throw new RuntimeException("文件上传失败: " + e.getMessage());
        }
    }
    
    /**
     * 分页查询文件
     */
    public IPage<PolicyFile> getFiles(String keyword, Long categoryId, Set<String> tagNames,
                                   LocalDateTime startDate, LocalDateTime endDate,
                                   String issuingAuthority, Boolean isPublic, int page, int size) {
        
        QueryWrapper<PolicyFile> queryWrapper = new QueryWrapper<>();
        queryWrapper.eq("is_enabled", true);
        
        // 关键词查询（标题、描述、发布机关）
        if (StringUtils.hasText(keyword)) {
            queryWrapper.and(wrapper -> wrapper
                .like("title", keyword)
                .or().like("description", keyword)
                .or().like("issuing_authority", keyword)
            );
        }
        
        // 分类过滤（包含子分类）
        if (categoryId != null) {
            List<Long> categoryIds = categoryMapper.findCategoryAndChildrenIds(categoryId);
            if (!categoryIds.isEmpty()) {
                queryWrapper.in("category_id", categoryIds);
            }
        }
        
        // 发布机关过滤
        if (StringUtils.hasText(issuingAuthority)) {
            queryWrapper.like("issuing_authority", issuingAuthority);
        }
        
        // 公开状态过滤
        if (isPublic != null) {
            queryWrapper.eq("is_public", isPublic);
        }
        
        // 日期范围过滤
        if (startDate != null) {
            queryWrapper.ge("create_time", startDate);
        }
        if (endDate != null) {
            queryWrapper.le("create_time", endDate);
        }
        
        // 排序
        queryWrapper.orderByDesc("create_time");
        
        // 分页参数验证
        if (page < 0) page = 0;
        if (size <= 0 || size > 100) size = 10; // 限制最大分页大小
        
        Page<PolicyFile> pageParam = new Page<>(page + 1, size); // MyBatis-Plus的page从1开始
        IPage<PolicyFile> result = fileMapper.selectPage(pageParam, queryWrapper);
        
        // 设置分类信息
        if (result.getRecords() != null) {
            for (PolicyFile file : result.getRecords()) {
                if (file.getCategoryId() != null) {
                    FileCategory category = categoryMapper.selectById(file.getCategoryId());
                    file.setCategory(category);
                }
            }
        }
        
        return result;
    }
    
    /**
     * 根据ID获取文件
     */
    public PolicyFile getFileById(Long id) {
        PolicyFile file = fileMapper.selectById(id);
        if (file != null && file.getEnabled()) {
            return file;
        }
        throw new RuntimeException("文件不存在");
    }
    
    /**
     * 增加查看次数
     */
    @Transactional
    public void incrementViewCount(Long id) {
        fileMapper.incrementViewCount(id);
    }
    
    /**
     * 增加下载次数
     */
    @Transactional
    public void incrementDownloadCount(Long id) {
        fileMapper.incrementDownloadCount(id);
    }
    
    /**
     * 查找热门文件
     */
    public List<PolicyFile> getPopularFiles() {
        return fileMapper.findTop10ByEnabledTrueOrderByViewCountDesc();
    }
    
    /**
     * 查找最新文件
     */
    public List<PolicyFile> getLatestFiles() {
        return fileMapper.findTop10ByEnabledTrueOrderByCreateTimeDesc();
    }
    
    /**
     * 更新文件信息
     */
    @Transactional
    public PolicyFile updateFile(Long id, String title, String description, Long categoryId, 
                               Set<String> tagNames, String documentNumber, LocalDateTime issueDate, 
                               LocalDateTime effectiveDate, String issuingAuthority, Boolean isPublic) {
        PolicyFile file = fileMapper.selectById(id);
        if (file == null) {
            auditService.logFileUpdate(id, "unknown", "all", false, "文件不存在");
            throw new RuntimeException("文件不存在");
        }
        
        // 检查更新权限
        if (!hasUpdatePermission(file)) {
            auditService.logAccessDenied("update", id, file.getTitle(), "用户没有更新权限");
            throw new RuntimeException("没有权限更新该文件");
        }
        
        try {
            // 记录更新字段
            StringBuilder updateFields = new StringBuilder();
            if (title != null) updateFields.append("title,");
            if (description != null) updateFields.append("description,");
            if (categoryId != null) updateFields.append("categoryId,");
            if (documentNumber != null) updateFields.append("documentNumber,");
            if (issueDate != null) updateFields.append("issueDate,");
            if (effectiveDate != null) updateFields.append("effectiveDate,");
            if (issuingAuthority != null) updateFields.append("issuingAuthority,");
            if (isPublic != null) updateFields.append("isPublic,");
            
            file.setTitle(title);
            file.setDescription(description);
            file.setDocumentNumber(documentNumber);
            file.setIssueDate(issueDate);
            file.setEffectiveDate(effectiveDate);
            file.setIssuingAuthority(issuingAuthority);
            file.setIsPublic(isPublic);
            file.setUpdatedBy(StpUtil.getLoginIdAsLong());
            
            if (categoryId != null) {
                file.setCategoryId(categoryId);
            }
            
            PolicyFile result = fileMapper.updateById(file) > 0 ? file : null;
            
            // 记录审计日志
            auditService.logFileUpdate(id, file.getTitle(), updateFields.toString(), true, null);
            
            return result;
        } catch (Exception e) {
            log.error("更新文件失败: {}", e.getMessage());
            auditService.logFileUpdate(id, file.getTitle(), "all", false, e.getMessage());
            throw new RuntimeException("更新文件失败: " + e.getMessage());
        }
    }
    
    /**
     * 删除文件
     */
    @Transactional
    public void deleteFile(Long id) {
        PolicyFile file = fileMapper.selectById(id);
        if (file == null) {
            auditService.logFileDelete(id, "unknown", false, "文件不存在");
            throw new RuntimeException("文件不存在");
        }
        
        // 检查删除权限
        if (!hasDeletePermission(file)) {
            auditService.logAccessDenied("delete", id, file.getTitle(), "用户没有删除权限");
            throw new RuntimeException("没有权限删除该文件");
        }
        
        try {
            // 软删除
            file.setEnabled(false);
            file.setUpdatedBy(StpUtil.getLoginIdAsLong());
            fileMapper.updateById(file);
            
            log.info("文件已删除: id={}, title={}, operator={}", 
                id, file.getTitle(), StpUtil.getLoginIdAsLong());
            
            // 记录审计日志
            auditService.logFileDelete(id, file.getTitle(), true, null);
        } catch (Exception e) {
            log.error("删除文件失败: {}", e.getMessage());
            auditService.logFileDelete(id, file.getTitle(), false, e.getMessage());
            throw new RuntimeException("删除文件失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取用户文件
     */
    public IPage<PolicyFile> getUserFiles(Long userId, int page, int size) {
        QueryWrapper<PolicyFile> queryWrapper = new QueryWrapper<>();
        queryWrapper.eq("created_by", userId);
        queryWrapper.eq("is_enabled", true);
        queryWrapper.orderByDesc("create_time");
        
        Page<PolicyFile> pageParam = new Page<>(page, size);
        return fileMapper.selectPage(pageParam, queryWrapper);
    }
    
    /**
     * 从路径字符串中提取文件名的安全方法
     */
    private String extractFileNameFromPath(String filePath) {
        if (filePath == null || filePath.trim().isEmpty()) {
            return null;
        }
        
        // 统一使用正斜杠处理路径
        String normalizedPath = filePath.replace("\\", "/");
        
        // 提取最后一个斜杠后的文件名
        int lastSlashIndex = normalizedPath.lastIndexOf("/");
        if (lastSlashIndex >= 0 && lastSlashIndex < normalizedPath.length() - 1) {
            return normalizedPath.substring(lastSlashIndex + 1);
        }
        
        // 如果没有找到斜杠，说明整个字符串就是文件名
        return normalizedPath;
    }

    /**
     * 获取文件扩展名
     */
    private String getFileExtension(String fileName) {
        if (fileName == null || !fileName.contains(".")) {
            return "";
        }
        return fileName.substring(fileName.lastIndexOf(".") + 1).toLowerCase();
    }
    
    /**
     * 检查是否有更新文件的权限
     */
    private boolean hasUpdatePermission(PolicyFile file) {
        try {
            Long currentUserId = StpUtil.getLoginIdAsLong();
            
            // 文件作者可以更新
            if (file.getCreatedBy() != null && file.getCreatedBy().equals(currentUserId)) {
                return true;
            }
            
            // 管理员可以更新所有文件
            if (StpUtil.hasRole("admin")) {
                return true;
            }
            
            return false;
        } catch (Exception e) {
            log.error("检查更新权限失败: {}", e.getMessage());
            return false;
        }
    }
    
    /**
     * 检查是否有删除文件的权限
     */
    private boolean hasDeletePermission(PolicyFile file) {
        try {
            Long currentUserId = StpUtil.getLoginIdAsLong();
            
            // 文件作者可以删除
            if (file.getCreatedBy() != null && file.getCreatedBy().equals(currentUserId)) {
                return true;
            }
            
            // 管理员可以删除所有文件
            if (StpUtil.hasRole("admin")) {
                return true;
            }
            
            return false;
        } catch (Exception e) {
            log.error("检查删除权限失败: {}", e.getMessage());
            return false;
        }
    }
}