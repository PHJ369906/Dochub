package com.example.service;

import com.example.config.FileStorageConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.util.Arrays;
import java.util.List;
import java.util.UUID;
import java.util.regex.Pattern;

/**
 * 文件存储服务
 */
@Service
public class FileStorageService {
    
    private static final Logger log = LoggerFactory.getLogger(FileStorageService.class);
    
    @Autowired
    private FileStorageConfig storageConfig;
    
    /**
     * 存储文件
     */
    public String storeFile(MultipartFile file) throws IOException {
        // 验证文件
        validateFile(file);
        
        // 验证文件内容
        validateFileContent(file);
        
        // 生成安全的文件名
        String fileName = generateSecureFileName(file);
        
        // 创建存储路径
        Path targetLocation = createSecureStoragePath(fileName);
        
        // 确保目录存在且在允许的范围内
        Path parentDir = targetLocation.getParent();
        if (!isPathSecure(parentDir)) {
            throw new RuntimeException("不安全的文件路径");
        }
        Files.createDirectories(parentDir);
        
        // 存储文件
        Files.copy(file.getInputStream(), targetLocation, StandardCopyOption.REPLACE_EXISTING);
        
        log.info("文件存储成功: {}", targetLocation.toString());
        return getRelativePath(targetLocation);
    }
    
    /**
     * 删除文件
     */
    public boolean deleteFile(String filePath) {
        try {
            Path path = Paths.get(storageConfig.getUploadDir()).resolve(filePath);
            return Files.deleteIfExists(path);
        } catch (IOException e) {
            log.error("删除文件失败: {}", e.getMessage());
            return false;
        }
    }
    
    /**
     * 获取文件路径
     */
    public Path getFilePath(String filePath) {
        return Paths.get(storageConfig.getUploadDir()).resolve(filePath);
    }
    
    /**
     * 检查文件是否存在
     */
    public boolean fileExists(String filePath) {
        return Files.exists(getFilePath(filePath));
    }
    
    /**
     * 验证文件
     */
    private void validateFile(MultipartFile file) {
        if (file.isEmpty()) {
            throw new RuntimeException("文件不能为空");
        }
        
        if (file.getSize() > storageConfig.getMaxFileSize()) {
            throw new RuntimeException("文件大小超过限制: " + formatFileSize(storageConfig.getMaxFileSize()));
        }
        
        // 验证文件名安全性
        String originalFilename = file.getOriginalFilename();
        if (!isFileNameSecure(originalFilename)) {
            throw new RuntimeException("文件名包含非法字符");
        }
        
        String fileType = getFileExtension(originalFilename);
        if (!storageConfig.isAllowedType(fileType)) {
            throw new RuntimeException("不支持的文件类型: " + fileType);
        }
    }
    
    /**
     * 生成安全的文件名
     */
    private String generateSecureFileName(MultipartFile file) {
        String originalFilename = file.getOriginalFilename();
        if (originalFilename == null || originalFilename.trim().isEmpty()) {
            throw new RuntimeException("文件名不能为空");
        }
        
        // 清理文件名，移除危险字符
        String cleanFilename = sanitizeFileName(originalFilename);
        String fileExtension = getFileExtension(cleanFilename);
        
        switch (storageConfig.getFileNameStrategy()) {
            case "uuid":
                return UUID.randomUUID().toString() + "." + fileExtension;
            case "timestamp":
                return System.currentTimeMillis() + "_" + sanitizeFileName(cleanFilename);
            case "original":
            default:
                return sanitizeFileName(cleanFilename);
        }
    }
    
    /**
     * 创建安全的存储路径
     */
    private Path createSecureStoragePath(String fileName) {
        Path uploadPath = Paths.get(storageConfig.getUploadDir()).toAbsolutePath().normalize();
        
        if (storageConfig.getDateFolder()) {
            String dateFolder = LocalDate.now().format(DateTimeFormatter.ofPattern("yyyy/MM/dd"));
            uploadPath = uploadPath.resolve(dateFolder);
        }
        
        Path targetPath = uploadPath.resolve(fileName).normalize();
        
        // 确保目标路径在上传目录内
        if (!targetPath.startsWith(uploadPath)) {
            throw new RuntimeException("检测到路径遍历攻击");
        }
        
        return targetPath;
    }
    
    /**
     * 获取相对路径
     */
    private String getRelativePath(Path absolutePath) {
        Path uploadPath = Paths.get(storageConfig.getUploadDir());
        return uploadPath.relativize(absolutePath).toString().replace("\\", "/");
    }
    
    /**
     * 获取文件扩展名
     */
    public String getFileExtension(String filename) {
        if (filename == null || filename.isEmpty()) {
            return "";
        }
        int lastDotIndex = filename.lastIndexOf('.');
        return lastDotIndex > 0 ? filename.substring(lastDotIndex + 1).toLowerCase() : "";
    }
    
    /**
     * 格式化文件大小
     */
    public String formatFileSize(long size) {
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
    
    /**
     * 验证文件内容类型
     */
    private void validateFileContent(MultipartFile file) throws IOException {
        String declaredContentType = file.getContentType();
        String fileExtension = getFileExtension(file.getOriginalFilename());
        
        // 检查文件头部字节来验证真实文件类型
        try (InputStream inputStream = file.getInputStream()) {
            byte[] header = new byte[8];
            int bytesRead = inputStream.read(header);
            
            if (bytesRead > 0) {
                String detectedType = detectFileTypeFromHeader(header);
                if (detectedType != null && !isValidFileTypeMatch(detectedType, fileExtension)) {
                    throw new RuntimeException("文件类型与扩展名不匹配，可能存在安全风险");
                }
            }
        }
    }
    
    /**
     * 从文件头部检测文件类型
     */
    private String detectFileTypeFromHeader(byte[] header) {
        if (header.length < 4) return null;
        
        // PDF文件
        if (header[0] == 0x25 && header[1] == 0x50 && header[2] == 0x44 && header[3] == 0x46) {
            return "pdf";
        }
        // PNG文件
        if (header[0] == (byte)0x89 && header[1] == 0x50 && header[2] == 0x4E && header[3] == 0x47) {
            return "png";
        }
        // JPEG文件
        if (header[0] == (byte)0xFF && header[1] == (byte)0xD8 && header[2] == (byte)0xFF) {
            return "jpg";
        }
        // ZIP文件（包括docx, xlsx等）
        if (header[0] == 0x50 && header[1] == 0x4B && (header[2] == 0x03 || header[2] == 0x05)) {
            return "zip";
        }
        
        return null;
    }
    
    /**
     * 验证检测到的文件类型与扩展名是否匹配
     */
    private boolean isValidFileTypeMatch(String detectedType, String fileExtension) {
        switch (detectedType.toLowerCase()) {
            case "pdf":
                return "pdf".equals(fileExtension);
            case "png":
                return "png".equals(fileExtension);
            case "jpg":
                return Arrays.asList("jpg", "jpeg").contains(fileExtension);
            case "zip":
                return Arrays.asList("zip", "docx", "xlsx", "pptx").contains(fileExtension);
            default:
                return true; // 未知类型暂时允许
        }
    }
    
    /**
     * 验证文件名安全性
     */
    private boolean isFileNameSecure(String fileName) {
        if (fileName == null || fileName.trim().isEmpty()) {
            return false;
        }
        
        // 检查危险字符
        Pattern dangerousPattern = Pattern.compile("[<>:\"/\\\\|?*\\x00-\\x1f]");
        if (dangerousPattern.matcher(fileName).find()) {
            return false;
        }
        
        // 检查路径遍历
        if (fileName.contains("..") || fileName.startsWith("/") || fileName.startsWith("\\")) {
            return false;
        }
        
        // 检查Windows保留名称
        String nameWithoutExt = fileName.contains(".") ? 
            fileName.substring(0, fileName.lastIndexOf(".")) : fileName;
        List<String> reservedNames = Arrays.asList(
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", 
            "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", 
            "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        );
        
        return !reservedNames.contains(nameWithoutExt.toUpperCase());
    }
    
    /**
     * 清理文件名，移除危险字符
     */
    private String sanitizeFileName(String fileName) {
        if (fileName == null) return "unnamed";
        
        // 替换危险字符为下划线
        String sanitized = fileName.replaceAll("[<>:\"/\\\\|?*\\x00-\\x1f]", "_");
        
        // 移除路径遍历相关字符
        sanitized = sanitized.replace("..", "_");
        
        // 限制文件名长度
        if (sanitized.length() > 255) {
            String extension = "";
            int lastDot = sanitized.lastIndexOf('.');
            if (lastDot > 0) {
                extension = sanitized.substring(lastDot);
                sanitized = sanitized.substring(0, Math.min(255 - extension.length(), lastDot)) + extension;
            } else {
                sanitized = sanitized.substring(0, 255);
            }
        }
        
        return sanitized.trim();
    }
    
    /**
     * 验证路径安全性
     */
    private boolean isPathSecure(Path path) {
        try {
            Path uploadPath = Paths.get(storageConfig.getUploadDir()).toAbsolutePath().normalize();
            Path normalizedPath = path.toAbsolutePath().normalize();
            return normalizedPath.startsWith(uploadPath);
        } catch (Exception e) {
            log.error("路径验证失败: {}", e.getMessage());
            return false;
        }
    }
}
