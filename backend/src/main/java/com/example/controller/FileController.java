package com.example.controller;

import cn.dev33.satoken.annotation.SaCheckLogin;
import cn.dev33.satoken.annotation.SaCheckRole;
import cn.dev33.satoken.stp.StpUtil;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.example.common.Result;
import com.example.entity.PolicyFile;
import com.example.entity.FileTag;

import com.example.service.FileStorageService;
import com.example.service.PolicyFileService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.core.io.Resource;
import org.springframework.core.io.UrlResource;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.file.Path;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * 文件管理控制器
 */
@Tag(name = "文件管理", description = "政策法规文件管理相关接口")
@RestController
@RequestMapping("/api/files")
public class FileController {
    
    private static final Logger log = LoggerFactory.getLogger(FileController.class);
    
    private final PolicyFileService fileService;
    private final FileStorageService storageService;

    public FileController(PolicyFileService fileService, FileStorageService storageService) {
        this.fileService = fileService;
        this.storageService = storageService;
    }
    
    /**
     * 上传文件
     */
    @Operation(summary = "上传文件")
    @PostMapping("/upload")
    @SaCheckLogin
    public Result<PolicyFile> uploadFile(
            @RequestParam("file") MultipartFile file,
            @RequestParam(value = "title", required = false) String title,
            @RequestParam(value = "description", required = false) String description,
            @RequestParam(value = "categoryId", required = false) Long categoryId,
            @RequestParam(value = "tags", required = false) Set<String> tags,
            @RequestParam(value = "documentNumber", required = false) String documentNumber,
            @RequestParam(value = "issueDate", required = false) String issueDateStr,
            @RequestParam(value = "effectiveDate", required = false) String effectiveDateStr,
            @RequestParam(value = "issuingAuthority", required = false) String issuingAuthority) {
        try {
            // 转换日期字符串为LocalDateTime
            LocalDateTime issueDate = null;
            LocalDateTime effectiveDate = null;
            
            if (issueDateStr != null && !issueDateStr.trim().isEmpty()) {
                try {
                    // 处理JavaScript日期格式
                    issueDate = parseJavaScriptDate(issueDateStr);
                } catch (Exception e) {
                    return Result.error("发布日期格式错误: " + e.getMessage());
                }
            }
            
            if (effectiveDateStr != null && !effectiveDateStr.trim().isEmpty()) {
                try {
                    // 处理JavaScript日期格式
                    effectiveDate = parseJavaScriptDate(effectiveDateStr);
                } catch (Exception e) {
                    return Result.error("生效日期格式错误: " + e.getMessage());
                }
            }
            
            PolicyFile uploadedFile = fileService.uploadFile(file, title, description, categoryId, 
                tags, documentNumber, issueDate, effectiveDate, issuingAuthority);
            return Result.success("文件上传成功", uploadedFile);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 批量上传文件
     */
    @Operation(summary = "批量上传文件")
    @PostMapping("/batch-upload")
    @SaCheckLogin
    public Result<List<PolicyFile>> batchUploadFiles(
            @RequestParam("files") MultipartFile[] files,
            @RequestParam(value = "categoryId", required = false) Long categoryId,
            @RequestParam(value = "tags", required = false) Set<String> tags) {
        try {
            // 检查批量上传权限
            if (!hasBatchUploadPermission()) {
                return Result.error("没有批量上传权限");
            }
            
            // 限制批量上传数量
            if (files.length > 20) {
                return Result.error("批量上传文件数量不能超过20个");
            }
            
            List<PolicyFile> uploadedFiles = new ArrayList<>();

            for (MultipartFile file : files) {
                if (!file.isEmpty()) {
                    PolicyFile uploadedFile = fileService.uploadFile(
                        file,
                        null, // 使用文件名作为标题
                        null, // 描述为空
                        categoryId,
                        tags,
                        null, // 文件编号为空
                        null, // 发布日期为空
                        null, // 生效日期为空
                        null  // 发布机关为空
                    );
                    uploadedFiles.add(uploadedFile);
                }
            }

            return Result.success("批量上传成功，共上传 " + uploadedFiles.size() + " 个文件", uploadedFiles);
        } catch (Exception e) {
            log.error("批量上传失败: {}", e.getMessage());
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取文件列表
     */
    @Operation(summary = "获取文件列表")
    @GetMapping
    public Result<IPage<PolicyFile>> getFiles(
            @RequestParam(value = "keyword", required = false) String keyword,
            @RequestParam(value = "categoryId", required = false) Long categoryId,
            @RequestParam(value = "tags", required = false) Set<String> tags,
            @RequestParam(value = "startDate", required = false) String startDateStr,
            @RequestParam(value = "endDate", required = false) String endDateStr,
            @RequestParam(value = "issuingAuthority", required = false) String issuingAuthority,
            @RequestParam(value = "isPublic", required = false) Boolean isPublic,
            @RequestParam(value = "page", defaultValue = "0") int page,
            @RequestParam(value = "size", defaultValue = "10") int size,
            @RequestParam(value = "sort", defaultValue = "createTime") String sort,
            @RequestParam(value = "direction", defaultValue = "desc") String direction) {
        try {
            // 转换日期字符串为LocalDateTime
            LocalDateTime startDate = null;
            LocalDateTime endDate = null;
            
            if (startDateStr != null && !startDateStr.trim().isEmpty()) {
                try {
                    startDate = parseJavaScriptDate(startDateStr);
                } catch (Exception e) {
                    return Result.error("开始日期格式错误: " + e.getMessage());
                }
            }
            
            if (endDateStr != null && !endDateStr.trim().isEmpty()) {
                try {
                    endDate = parseJavaScriptDate(endDateStr);
                } catch (Exception e) {
                    return Result.error("结束日期格式错误: " + e.getMessage());
                }
            }
            
            IPage<PolicyFile> files = fileService.getFiles(keyword, categoryId, tags, 
                startDate, endDate, issuingAuthority, isPublic, page, size);
            return Result.success(files);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取文件详情
     */
    @Operation(summary = "获取文件详情")
    @GetMapping("/{id}")
    public Result<PolicyFile> getFileById(@PathVariable Long id) {
        try {
            PolicyFile file = fileService.getFileById(id);
            
            // 检查文件访问权限
            if (!hasFileAccessPermission(file)) {
                return Result.error("没有权限访问该文件");
            }
            
            // 增加查看次数
            fileService.incrementViewCount(id);
            return Result.success(file);
        } catch (Exception e) {
            log.error("获取文件详情失败: {}", e.getMessage());
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 更新文件信息
     */
    @Operation(summary = "更新文件信息")
    @PutMapping("/{id}")
    @SaCheckLogin
    public Result<PolicyFile> updateFile(
            @PathVariable Long id,
            @RequestParam(value = "title", required = false) String title,
            @RequestParam(value = "description", required = false) String description,
            @RequestParam(value = "categoryId", required = false) Long categoryId,
            @RequestParam(value = "tags", required = false) Set<String> tags,
            @RequestParam(value = "documentNumber", required = false) String documentNumber,
            @RequestParam(value = "issueDate", required = false) String issueDateStr,
            @RequestParam(value = "effectiveDate", required = false) String effectiveDateStr,
            @RequestParam(value = "issuingAuthority", required = false) String issuingAuthority,
            @RequestParam(value = "isPublic", required = false) Boolean isPublic) {
        try {
            // 转换日期字符串为LocalDateTime
            LocalDateTime issueDate = null;
            LocalDateTime effectiveDate = null;
            
            if (issueDateStr != null && !issueDateStr.trim().isEmpty()) {
                try {
                    // 处理JavaScript日期格式
                    issueDate = parseJavaScriptDate(issueDateStr);
                } catch (Exception e) {
                    return Result.error("发布日期格式错误: " + e.getMessage());
                }
            }
            
            if (effectiveDateStr != null && !effectiveDateStr.trim().isEmpty()) {
                try {
                    // 处理JavaScript日期格式
                    effectiveDate = parseJavaScriptDate(effectiveDateStr);
                } catch (Exception e) {
                    return Result.error("生效日期格式错误: " + e.getMessage());
                }
            }
            
            PolicyFile updatedFile = fileService.updateFile(id, title, description, categoryId, 
                tags, documentNumber, issueDate, effectiveDate, issuingAuthority, isPublic);
            return Result.success("文件更新成功", updatedFile);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 删除文件
     */
    @Operation(summary = "删除文件")
    @DeleteMapping("/{id}")
    @SaCheckLogin
    public Result<String> deleteFile(@PathVariable Long id) {
        try {
            fileService.deleteFile(id);
            return Result.success("文件删除成功");
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 下载文件
     */
    @Operation(summary = "下载文件")
    @GetMapping("/{id}/download")
    public ResponseEntity<Resource> downloadFile(@PathVariable Long id, HttpServletRequest request) {
        try {
            PolicyFile file = fileService.getFileById(id);
            
            // 检查文件访问权限
            if (!hasFileAccessPermission(file)) {
                return ResponseEntity.status(403).build(); // Forbidden
            }
            
            Path filePath = storageService.getFilePath(file.getFilePath());
            Resource resource = new UrlResource(filePath.toUri());
            
            if (!resource.exists()) {
                throw new RuntimeException("文件不存在");
            }
            
            // 增加下载次数
            fileService.incrementDownloadCount(id);
            
            String contentType = null;
            try {
                contentType = request.getServletContext().getMimeType(resource.getFile().getAbsolutePath());
            } catch (IOException ex) {
                // 忽略
            }
            
            if (contentType == null) {
                contentType = "application/octet-stream";
            }
            
            // 处理中文文件名编码
            String encodedFileName = java.net.URLEncoder.encode(file.getOriginalName(), "UTF-8")
                .replaceAll("\\+", "%20");
            
            return ResponseEntity.ok()
                    .contentType(MediaType.parseMediaType(contentType))
                    .header(HttpHeaders.CONTENT_DISPOSITION, 
                        "attachment; filename*=UTF-8''" + encodedFileName)
                    .body(resource);
        } catch (MalformedURLException e) {
            log.error("文件下载失败 - URL格式错误: {}", e.getMessage());
            return ResponseEntity.badRequest().build();
        } catch (Exception e) {
            log.error("文件下载失败: {}", e.getMessage());
            return ResponseEntity.notFound().build();
        }
    }

    /**
     * 预览文件
     */
    @Operation(summary = "预览文件")
    @GetMapping("/{id}/preview")
    public ResponseEntity<Resource> previewFile(@PathVariable Long id, HttpServletRequest request) {
        try {
            PolicyFile file = fileService.getFileById(id);
            
            // 检查文件访问权限
            if (!hasFileAccessPermission(file)) {
                return ResponseEntity.status(403).build(); // Forbidden
            }
            
            Path filePath = storageService.getFilePath(file.getFilePath());
            Resource resource = new UrlResource(filePath.toUri());
            
            if (!resource.exists()) {
                throw new RuntimeException("文件不存在");
            }
            
            // 增加查看次数
            fileService.incrementViewCount(id);
            
            String contentType = file.getMimeType();
            if (contentType == null) {
                try {
                    contentType = request.getServletContext().getMimeType(resource.getFile().getAbsolutePath());
                } catch (IOException ex) {
                    // 忽略
                }
            }
            
            if (contentType == null) {
                contentType = "application/octet-stream";
            }
            
            // 处理中文文件名编码
            String encodedFileName = java.net.URLEncoder.encode(file.getOriginalName(), "UTF-8")
                .replaceAll("\\+", "%20");
            
            return ResponseEntity.ok()
                    .contentType(MediaType.parseMediaType(contentType))
                    .header(HttpHeaders.CONTENT_DISPOSITION, "inline; filename*=UTF-8''" + encodedFileName)
                    .body(resource);
        } catch (MalformedURLException e) {
            log.error("文件预览失败 - URL格式错误: {}", e.getMessage());
            return ResponseEntity.badRequest().build();
        } catch (Exception e) {
            log.error("文件预览失败: {}", e.getMessage());
            return ResponseEntity.notFound().build();
        }
    }
    
    /**
     * 获取文件内容
     */
    @Operation(summary = "获取文件内容")
    @GetMapping("/{id}/content")
    public ResponseEntity<String> getFileContent(@PathVariable Long id) {
        try {
            PolicyFile file = fileService.getFileById(id);
            
            // 检查文件访问权限
            if (!hasFileAccessPermission(file)) {
                return ResponseEntity.status(403).build(); // Forbidden
            }
            
            // 只允许文本类型文件获取内容
            String fileType = file.getFileType();
            if (!isTextFile(fileType)) {
                return ResponseEntity.badRequest().body("不支持获取该文件类型的内容");
            }
            
            Path filePath = storageService.getFilePath(file.getFilePath());
            if (!storageService.fileExists(file.getFilePath())) {
                throw new RuntimeException("文件不存在");
            }
            
            // 读取文件内容
            try {
                String content = java.nio.file.Files.readString(filePath, java.nio.charset.StandardCharsets.UTF_8);
                
                // 增加查看次数
                fileService.incrementViewCount(id);
                
                return ResponseEntity.ok()
                        .contentType(MediaType.TEXT_PLAIN)
                        .body(content);
            } catch (IOException e) {
                log.error("读取文件内容失败: {}", e.getMessage());
                return ResponseEntity.status(500).body("读取文件内容失败");
            }
        } catch (Exception e) {
            log.error("获取文件内容失败: {}", e.getMessage());
            return ResponseEntity.status(500).body("获取文件内容失败: " + e.getMessage());
        }
    }

    
    /**
     * 获取热门文件
     */
    @Operation(summary = "获取热门文件")
    @GetMapping("/popular")
    public Result<List<PolicyFile>> getPopularFiles() {
        try {
            List<PolicyFile> files = fileService.getPopularFiles();
            return Result.success(files);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取最新文件
     */
    @Operation(summary = "获取最新文件")
    @GetMapping("/latest")
    public Result<List<PolicyFile>> getLatestFiles() {
        try {
            List<PolicyFile> files = fileService.getLatestFiles();
            return Result.success(files);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取我的文件
     */
    @Operation(summary = "获取我的文件")
    @GetMapping("/my")
    @SaCheckLogin
    public Result<IPage<PolicyFile>> getMyFiles(
            @RequestParam(value = "page", defaultValue = "0") int page,
            @RequestParam(value = "size", defaultValue = "10") int size) {
        try {
            Long userId = StpUtil.getLoginIdAsLong();
            IPage<PolicyFile> files = fileService.getUserFiles(userId, page, size);
            return Result.success(files);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 解析JavaScript日期字符串为LocalDateTime
     * 支持多种日期格式的解析
     */
    private LocalDateTime parseJavaScriptDate(String dateStr) throws DateTimeParseException {
        if (dateStr == null || dateStr.trim().isEmpty()) {
            return null;
        }
        
        dateStr = dateStr.trim();
        
        // 尝试不同的日期格式解析
        DateTimeFormatter[] formatters = {
            DateTimeFormatter.ISO_DATE_TIME,
            DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"),
            DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss'Z'"),
            DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss.SSS"),
            DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss"),
            DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss"),
            DateTimeFormatter.ofPattern("yyyy-MM-dd")
        };
        
        // 预处理日期字符串，移除末尾的Z
        String processedDateStr = dateStr.replace("Z", "");
        
        // 处理JavaScript Date.toString()格式 "Mon Aug 11 2025 00:00:00 GMT+0800 (中国标准时间)"
        if (dateStr.contains("GMT")) {
            // 提取日期部分并重新格式化
            try {
                // 解析为Instant然后转换为LocalDateTime
                java.time.Instant instant = java.time.Instant.parse(dateStr.replaceAll("\\s*\\([^)]*\\)\\s*$", "")
                    .replaceAll("GMT\\+\\d{4}", "Z"));
                return LocalDateTime.ofInstant(instant, java.time.ZoneId.systemDefault());
            } catch (Exception e) {
                // 如果Instant解析失败，尝试手动解析
                String[] parts = dateStr.split("\\s+");
                if (parts.length >= 4) {
                    String year = parts[3];
                    String month = getMonthNumber(parts[1]);
                    String day = String.format("%02d", Integer.parseInt(parts[2]));
                    String time = parts[4];
                    
                    String isoFormat = String.format("%s-%s-%sT%s", year, month, day, time);
                    return LocalDateTime.parse(isoFormat);
                }
            }
        }
        
        // 尝试使用预定义的格式器
        for (DateTimeFormatter formatter : formatters) {
            try {
                return LocalDateTime.parse(processedDateStr, formatter);
            } catch (DateTimeParseException e) {
                // 继续尝试下一个格式
            }
        }
        
        throw new DateTimeParseException("无法解析日期格式: " + dateStr, dateStr, 0);
    }
    
    /**
     * 将月份英文名转换为数字
     */
    private String getMonthNumber(String monthName) {
        switch (monthName.toLowerCase()) {
            case "jan": return "01";
            case "feb": return "02";
            case "mar": return "03";
            case "apr": return "04";
            case "may": return "05";
            case "jun": return "06";
            case "jul": return "07";
            case "aug": return "08";
            case "sep": return "09";
            case "oct": return "10";
            case "nov": return "11";
            case "dec": return "12";
            default: throw new IllegalArgumentException("无效的月份名称: " + monthName);
        }
    }
    
    /**
     * 检查文件访问权限
     */
    private boolean hasFileAccessPermission(PolicyFile file) {
        try {
            // 公开文件可以直接访问
            if (Boolean.TRUE.equals(file.getIsPublic())) {
                return true;
            }
            
            // 未登录用户不能访问非公开文件
            if (!StpUtil.isLogin()) {
                return false;
            }
            
            Long currentUserId = StpUtil.getLoginIdAsLong();
            
            // 文件作者可以访问
            if (file.getCreatedBy() != null && file.getCreatedBy().equals(currentUserId)) {
                return true;
            }
            
            // 管理员可以访问所有文件
            if (StpUtil.hasRole("admin")) {
                return true;
            }
            
            // 其他情况下拒绝访问
            return false;
        } catch (Exception e) {
            log.error("检查文件访问权限失败: {}", e.getMessage());
            return false;
        }
    }
    
    /**
     * 检查批量上传权限
     */
    private boolean hasBatchUploadPermission() {
        try {
            // 必须登录
            if (!StpUtil.isLogin()) {
                return false;
            }
            
            // 管理员或编辑者可以批量上传
            return StpUtil.hasRole("admin") || StpUtil.hasRole("editor");
        } catch (Exception e) {
            log.error("检查批量上传权限失败: {}", e.getMessage());
            return false;
        }
    }
    
    /**
     * 获取所有标签
     */
    @Operation(summary = "获取所有标签")
    @GetMapping("/all-tags")
    public Result<List<FileTag>> getAllTags() {
        try {
            List<FileTag> tags = fileService.getAllTags();
            return Result.success(tags);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 检查是否为文本文件
     */
    private boolean isTextFile(String fileType) {
        if (fileType == null) {
            return false;
        }
        
        String lowerType = fileType.toLowerCase();
        return lowerType.equals("txt") || lowerType.equals("md") || 
               lowerType.equals("html") || lowerType.equals("htm") ||
               lowerType.equals("xml") || lowerType.equals("json") ||
               lowerType.equals("css") || lowerType.equals("js") ||
               lowerType.equals("java") || lowerType.equals("py") ||
               lowerType.equals("sql") || lowerType.equals("yml") ||
               lowerType.equals("yaml") || lowerType.equals("properties");
    }
}
