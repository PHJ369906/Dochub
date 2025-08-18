package com.example.controller;

import com.example.common.Result;
import com.example.entity.PolicyFile;
import com.example.service.FileStorageService;
import com.example.service.PolicyFileService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.core.io.Resource;
import org.springframework.core.io.UrlResource;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;

/**
 * 文件预览控制器
 */
@Tag(name = "文件预览", description = "文件预览相关接口")
@RestController
@RequestMapping("/api/preview")
public class PreviewController {

    private static final Logger log = LoggerFactory.getLogger(PreviewController.class);
    
    private final PolicyFileService fileService;
    private final FileStorageService storageService;

    public PreviewController(PolicyFileService fileService, FileStorageService storageService) {
        this.fileService = fileService;
        this.storageService = storageService;
    }

    /**
     * 获取文件预览信息
     */
    @Operation(summary = "获取文件预览信息")
    @GetMapping("/info/{id}")
    public Result<Map<String, Object>> getPreviewInfo(@PathVariable Long id) {
        try {
            PolicyFile file = fileService.getFileById(id);
            if (file == null) {
                return Result.error("文件不存在");
            }

            Map<String, Object> previewInfo = new HashMap<>();
            previewInfo.put("id", file.getId());
            previewInfo.put("title", file.getTitle());
            previewInfo.put("originalName", file.getOriginalName());
            previewInfo.put("fileType", file.getFileType());
            previewInfo.put("mimeType", file.getMimeType());
            previewInfo.put("fileSize", file.getFileSize());
            previewInfo.put("canPreview", canPreview(file.getFileType()));
            previewInfo.put("previewType", getPreviewType(file.getFileType()));

            return Result.success(previewInfo);
        } catch (Exception e) {
            log.error("获取文件预览信息失败: {}", e.getMessage());
            return Result.error("获取文件预览信息失败: " + e.getMessage());
        }
    }

    /**
     * 获取PDF页面预览
     */
    @Operation(summary = "获取PDF页面预览")
    @GetMapping("/pdf/{id}/page/{pageIndex}")
    public ResponseEntity<String> getPdfPagePreview(
            @PathVariable Long id,
            @PathVariable int pageIndex,
            @RequestParam(value = "dpi", defaultValue = "150") int dpi) {
        
        try {
            PolicyFile file = fileService.getFileById(id);
            if (file == null) {
                return ResponseEntity.notFound().build();
            }

            // 使用PDF.js进行预览
            String pdfViewerHtml = """
                <!DOCTYPE html>
                <html>
                <head>
                    <meta charset="UTF-8">
                    <title>PDF预览 - %s</title>
                    <style>
                        body { margin: 0; padding: 0; font-family: Arial, sans-serif; }
                        .pdf-container { width: 100%%; height: 100vh; }
                        iframe { width: 100%%; height: 100%%; border: none; }
                        .error { padding: 20px; text-align: center; color: #666; }
                    </style>
                </head>
                <body>
                    <div class="pdf-container">
                        <iframe src="/api/files/%d/download#toolbar=1&navpanes=1&scrollbar=1"
                                type="application/pdf"
                                title="PDF预览">
                        </iframe>
                    </div>
                </body>
                </html>
                """.formatted(file.getTitle(), file.getId());

            return ResponseEntity.ok()
                .contentType(MediaType.TEXT_HTML)
                .body(pdfViewerHtml);
                
        } catch (Exception e) {
            log.error("PDF预览失败: {}", e.getMessage());
            return ResponseEntity.status(500)
                .contentType(MediaType.TEXT_HTML)
                .body("<html><body><div class='error'><h3>PDF预览失败: " + e.getMessage() + "</h3></div></body></html>");
        }
    }

    /**
     * 获取Office文档预览
     */
    @Operation(summary = "获取Office文档预览")
    @GetMapping("/office/{id}")
    public ResponseEntity<String> getOfficePreview(@PathVariable Long id) {
        try {
            PolicyFile file = fileService.getFileById(id);
            if (file == null) {
                return ResponseEntity.notFound().build();
            }

            String fileType = file.getFileType().toLowerCase();
            String downloadUrl = "/api/files/" + id + "/download";
            
            // 简单的预览页面
            String officeViewerHtml = """
                <!DOCTYPE html>
                <html>
                <head>
                    <meta charset="UTF-8">
                    <title>文档预览 - %s</title>
                    <style>
                        body { margin: 0; padding: 20px; font-family: Arial, sans-serif; background: #f5f5f5; }
                        .container { max-width: 800px; margin: 0 auto; background: #fff; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
                        .file-info { margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #eee; }
                        .file-title { font-size: 24px; color: #333; margin-bottom: 10px; }
                        .file-details { color: #666; font-size: 14px; }
                        .preview-message { text-align: center; padding: 40px; color: #666; }
                        .download-btn { 
                            display: inline-block; background: #007bff; color: white; padding: 12px 24px; 
                            text-decoration: none; border-radius: 4px; margin: 10px 5px;
                        }
                        .download-btn:hover { background: #0056b3; }
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div class="file-info">
                            <h1 class="file-title">%s</h1>
                            <div class="file-details">文件类型: %s · 大小: %s</div>
                        </div>
                        <div class="preview-message">
                            <p>文档预览功能正在开发中</p>
                            <p>请下载文件后使用本地软件查看</p>
                            <a href="%s" class="download-btn" download>下载文件</a>
                        </div>
                    </div>
                </body>
                </html>
                """.formatted(
                    file.getTitle(),
                    file.getTitle(),
                    fileType.toUpperCase(), 
                    formatFileSize(file.getFileSize()),
                    downloadUrl
                );

            return ResponseEntity.ok()
                .contentType(MediaType.TEXT_HTML)
                .body(officeViewerHtml);
                
        } catch (Exception e) {
            log.error("Office文档预览失败: {}", e.getMessage());
            return ResponseEntity.status(500)
                .contentType(MediaType.TEXT_HTML)
                .body("<html><body><div class='error'><h3>Office文档预览失败: " + e.getMessage() + "</h3></div></body></html>");
        }
    }

    /**
     * 获取图片预览
     */
    @Operation(summary = "获取图片预览")
    @GetMapping("/image/{id}")
    public ResponseEntity<Resource> getImagePreview(
            @PathVariable Long id,
            @RequestParam(value = "width", required = false) Integer width,
            @RequestParam(value = "height", required = false) Integer height) {
        
        try {
            PolicyFile file = fileService.getFileById(id);
            if (file == null) {
                return ResponseEntity.notFound().build();
            }

            // 检查是否为图片类型
            String fileType = file.getFileType().toLowerCase();
            if (!isImageFile(fileType)) {
                return ResponseEntity.badRequest().build();
            }

            Path filePath = storageService.getFilePath(file.getFilePath());
            Resource resource = new UrlResource(filePath.toUri());
            
            if (!resource.exists()) {
                return ResponseEntity.notFound().build();
            }

            String contentType = file.getMimeType();
            if (contentType == null) {
                contentType = "image/" + fileType;
            }

            return ResponseEntity.ok()
                    .contentType(MediaType.parseMediaType(contentType))
                    .header(HttpHeaders.CONTENT_DISPOSITION, "inline; filename=\"" + file.getOriginalName() + "\"")
                    .body(resource);
        } catch (Exception e) {
            log.error("获取图片预览失败: {}", e.getMessage());
            return ResponseEntity.status(500).build();
        }
    }

    /**
     * 获取文本预览
     */
    @Operation(summary = "获取文本预览")
    @GetMapping("/text/{id}")
    public ResponseEntity<String> getTextPreview(
            @PathVariable Long id,
            @RequestParam(value = "asHtml", defaultValue = "true") boolean asHtml,
            @RequestParam(value = "maxLines", defaultValue = "1000") int maxLines) {
        
        try {
            PolicyFile file = fileService.getFileById(id);
            if (file == null) {
                return ResponseEntity.notFound().build();
            }

            // 检查是否为文本类型
            String fileType = file.getFileType().toLowerCase();
            if (!isTextFile(fileType)) {
                return ResponseEntity.badRequest().body("不支持的文本文件类型");
            }

            Path filePath = storageService.getFilePath(file.getFilePath());
            if (!Files.exists(filePath)) {
                return ResponseEntity.notFound().build();
            }

            try {
                // 读取文件内容
                String content = Files.readString(filePath, StandardCharsets.UTF_8);
                
                // 限制行数
                if (maxLines > 0) {
                    String[] lines = content.split("\n");
                    if (lines.length > maxLines) {
                        StringBuilder sb = new StringBuilder();
                        for (int i = 0; i < maxLines; i++) {
                            sb.append(lines[i]).append("\n");
                        }
                        sb.append("\n... (共 ").append(lines.length).append(" 行，仅显示前 ").append(maxLines).append(" 行)");
                        content = sb.toString();
                    }
                }
                
                if (asHtml) {
                    // HTML格式返回
                    String htmlContent = "<html><head><meta charset=\"UTF-8\"></head><body><pre style=\"font-family: monospace; white-space: pre-wrap; word-wrap: break-word;\">" 
                        + escapeHtml(content) + "</pre></body></html>";
                    return ResponseEntity.ok()
                        .contentType(MediaType.TEXT_HTML)
                        .body(htmlContent);
                } else {
                    // 纯文本返回
                    return ResponseEntity.ok()
                        .contentType(MediaType.TEXT_PLAIN)
                        .body(content);
                }
            } catch (IOException e) {
                log.error("读取文件内容失败: {}", e.getMessage());
                return ResponseEntity.status(500).body("读取文件内容失败");
            }
        } catch (Exception e) {
            log.error("获取文本预览失败: {}", e.getMessage());
            return ResponseEntity.status(500).body("获取文本预览失败");
        }
    }

    /**
     * 获取文件缩略图
     */
    @Operation(summary = "获取文件缩略图")
    @GetMapping("/thumbnail/{id}")
    public ResponseEntity<Resource> getThumbnail(
            @PathVariable Long id,
            @RequestParam(value = "width", defaultValue = "200") int width,
            @RequestParam(value = "height", defaultValue = "200") int height) {
        
        return ResponseEntity.notFound().build();
    }

    /**
     * 判断文件是否可以预览
     */
    private boolean canPreview(String fileType) {
        if (fileType == null) {
            return false;
        }
        
        String lowerType = fileType.toLowerCase();
        // 支持的预览类型
        return lowerType.equals("pdf") || 
               lowerType.equals("txt") || lowerType.equals("md") ||
               lowerType.equals("doc") || lowerType.equals("docx") ||
               lowerType.equals("xls") || lowerType.equals("xlsx") ||
               lowerType.equals("ppt") || lowerType.equals("pptx") ||
               lowerType.equals("jpg") || lowerType.equals("jpeg") ||
               lowerType.equals("png") || lowerType.equals("gif") ||
               lowerType.equals("html") || lowerType.equals("htm");
    }

    /**
     * 获取预览类型
     */
    private String getPreviewType(String fileType) {
        if (fileType == null) {
            return "none";
        }
        
        String lowerType = fileType.toLowerCase();
        
        if (lowerType.equals("pdf")) {
            return "pdf";
        } else if (isTextFile(lowerType)) {
            return "text";
        } else if (lowerType.equals("doc") || lowerType.equals("docx") ||
                   lowerType.equals("xls") || lowerType.equals("xlsx") ||
                   lowerType.equals("ppt") || lowerType.equals("pptx")) {
            return "office";
        } else if (isImageFile(lowerType)) {
            return "image";
        } else {
            return "none";
        }
    }

    /**
     * 检查是否为图片文件
     */
    private boolean isImageFile(String fileType) {
        if (fileType == null) {
            return false;
        }
        
        String lowerType = fileType.toLowerCase();
        return lowerType.equals("jpg") || lowerType.equals("jpeg") ||
               lowerType.equals("png") || lowerType.equals("gif") ||
               lowerType.equals("bmp") || lowerType.equals("webp") ||
               lowerType.equals("svg");
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

    /**
     * HTML转义
     */
    private String escapeHtml(String text) {
        if (text == null) {
            return null;
        }
        return text.replace("&", "&amp;")
                   .replace("<", "&lt;")
                   .replace(">", "&gt;")
                   .replace("\"", "&quot;")
                   .replace("'", "&#39;");
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
            return String.format("%.1f MB", size / (1024.0 * 1024));
        } else {
            return String.format("%.1f GB", size / (1024.0 * 1024 * 1024));
        }
    }
}