package com.example.service;

import org.apache.pdfbox.Loader;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.rendering.ImageType;
import org.apache.pdfbox.rendering.PDFRenderer;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.ss.usermodel.WorkbookFactory;
import org.apache.poi.xwpf.usermodel.XWPFDocument;
import org.apache.poi.xslf.usermodel.XMLSlideShow;
import org.apache.poi.hslf.usermodel.HSLFSlideShow;
import org.apache.tika.Tika;
import org.apache.tika.exception.TikaException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;
import net.coobird.thumbnailator.Thumbnails;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

/**
 * 文件转换服务 - 简化版本
 */
@Service
public class FileConvertService {
    
    private static final Logger log = LoggerFactory.getLogger(FileConvertService.class);
    
    @Value("${file.storage.upload-dir:uploads}")
    private String uploadPath;
    
    private final Tika tika = new Tika();
    
    /**
     * 将PDF转换为图片页面
     */
    public List<String> convertPdfToImages(String filePath) throws IOException {
        List<String> imagePages = new ArrayList<>();
        
        try (PDDocument document = Loader.loadPDF(new File(filePath))) {
            PDFRenderer pdfRenderer = new PDFRenderer(document);
            
            for (int page = 0; page < document.getNumberOfPages(); page++) {
                BufferedImage bim = pdfRenderer.renderImageWithDPI(page, 150, ImageType.RGB);
                
                String imageName = "pdf_page_" + page + ".png";
                String imagePath = getPreviewPath(filePath) + "/" + imageName;
                
                // 确保目录存在
                Files.createDirectories(Paths.get(imagePath).getParent());
                
                ImageIO.write(bim, "png", new File(imagePath));
                imagePages.add("/api/preview/image/" + getRelativePath(imagePath));
            }
        }
        
        return imagePages;
    }
    
    /**
     * 将Word文档转换为HTML - 简化版本
     */
    public String convertWordToHtml(String filePath) throws IOException {
        String htmlPath = getPreviewPath(filePath) + ".html";
        
        try {
            if (filePath.toLowerCase().endsWith(".docx")) {
                // 处理 .docx 文件 - 简化版本，只提取文本
                try (FileInputStream fis = new FileInputStream(filePath);
                     XWPFDocument document = new XWPFDocument(fis)) {
                    
                    StringBuilder content = new StringBuilder();
                    document.getParagraphs().forEach(paragraph -> {
                        content.append(paragraph.getText()).append("<br/>");
                    });
                    
                    String html = createHtmlWrapper("Word文档预览", content.toString());
                    try (FileWriter writer = new FileWriter(htmlPath, StandardCharsets.UTF_8)) {
                        writer.write(html);
                    }
                }
            } else {
                // .doc 文件直接用Tika解析
                String textContent = tika.parseToString(new File(filePath));
                String html = createHtmlWrapper("Word文档预览", textContent.replace("\n", "<br/>"));
                try (FileWriter writer = new FileWriter(htmlPath, StandardCharsets.UTF_8)) {
                    writer.write(html);
                }
            }
        } catch (Exception e) {
            log.error("Word转HTML失败: {}", e.getMessage(), e);
            throw new IOException("Word转HTML失败", e);
        }
        
        return "/api/preview/html/" + getRelativePath(htmlPath);
    }
    
    /**
     * 将Excel转换为HTML
     */
    public String convertExcelToHtml(String filePath) throws IOException {
        String htmlPath = getPreviewPath(filePath) + ".html";
        
        try (FileInputStream fis = new FileInputStream(filePath);
             Workbook workbook = WorkbookFactory.create(fis)) {
            
            StringBuilder content = new StringBuilder();
            
            for (int sheetIndex = 0; sheetIndex < workbook.getNumberOfSheets(); sheetIndex++) {
                var sheet = workbook.getSheetAt(sheetIndex);
                content.append("<h3>").append(sheet.getSheetName()).append("</h3>");
                content.append("<table border='1' style='border-collapse:collapse;width:100%;'>");
                
                for (var row : sheet) {
                    content.append("<tr>");
                    for (var cell : row) {
                        content.append("<td style='padding:8px;border:1px solid #ddd;'>");
                        switch (cell.getCellType()) {
                            case STRING -> content.append(cell.getStringCellValue());
                            case NUMERIC -> content.append(cell.getNumericCellValue());
                            case BOOLEAN -> content.append(cell.getBooleanCellValue());
                            case FORMULA -> content.append(cell.getCellFormula());
                            default -> content.append("");
                        }
                        content.append("</td>");
                    }
                    content.append("</tr>");
                }
                content.append("</table><br>");
            }
            
            String html = createHtmlWrapper("Excel文档预览", content.toString());
            try (FileWriter writer = new FileWriter(htmlPath, StandardCharsets.UTF_8)) {
                writer.write(html);
            }
            
        } catch (Exception e) {
            log.error("Excel转HTML失败: {}", e.getMessage(), e);
            throw new IOException("Excel转HTML失败", e);
        }
        
        return "/api/preview/html/" + getRelativePath(htmlPath);
    }
    
    /**
     * 将PPT转换为图片
     */
    public List<String> convertPptToImages(String filePath) throws IOException {
        List<String> imagePages = new ArrayList<>();
        
        try {
            if (filePath.toLowerCase().endsWith(".pptx")) {
                try (FileInputStream fis = new FileInputStream(filePath);
                     XMLSlideShow ppt = new XMLSlideShow(fis)) {
                    
                    Dimension pgsize = ppt.getPageSize();
                    int slideIndex = 0;
                    
                    for (var slide : ppt.getSlides()) {
                        BufferedImage img = new BufferedImage(pgsize.width, pgsize.height, BufferedImage.TYPE_INT_RGB);
                        Graphics2D graphics = img.createGraphics();
                        graphics.setPaint(Color.WHITE);
                        graphics.fill(new Rectangle(0, 0, pgsize.width, pgsize.height));
                        
                        slide.draw(graphics);
                        
                        String imageName = "ppt_slide_" + slideIndex + ".png";
                        String imagePath = getPreviewPath(filePath) + "/" + imageName;
                        
                        Files.createDirectories(Paths.get(imagePath).getParent());
                        ImageIO.write(img, "png", new File(imagePath));
                        imagePages.add("/api/preview/image/" + getRelativePath(imagePath));
                        
                        graphics.dispose();
                        slideIndex++;
                    }
                }
            } else {
                try (FileInputStream fis = new FileInputStream(filePath);
                     HSLFSlideShow ppt = new HSLFSlideShow(fis)) {
                    
                    Dimension pgsize = ppt.getPageSize();
                    int slideIndex = 0;
                    
                    for (var slide : ppt.getSlides()) {
                        BufferedImage img = new BufferedImage(pgsize.width, pgsize.height, BufferedImage.TYPE_INT_RGB);
                        Graphics2D graphics = img.createGraphics();
                        graphics.setPaint(Color.WHITE);
                        graphics.fill(new Rectangle(0, 0, pgsize.width, pgsize.height));
                        
                        slide.draw(graphics);
                        
                        String imageName = "ppt_slide_" + slideIndex + ".png";
                        String imagePath = getPreviewPath(filePath) + "/" + imageName;
                        
                        Files.createDirectories(Paths.get(imagePath).getParent());
                        ImageIO.write(img, "png", new File(imagePath));
                        imagePages.add("/api/preview/image/" + getRelativePath(imagePath));
                        
                        graphics.dispose();
                        slideIndex++;
                    }
                }
            }
        } catch (Exception e) {
            log.error("PPT转图片失败: {}", e.getMessage(), e);
            throw new IOException("PPT转图片失败", e);
        }
        
        return imagePages;
    }
    
    /**
     * 生成图片缩略图
     */
    public String generateImageThumbnail(String filePath, int width, int height) throws IOException {
        String thumbnailPath = getPreviewPath(filePath) + "_thumb_" + width + "x" + height + ".jpg";
        
        Files.createDirectories(Paths.get(thumbnailPath).getParent());
        
        Thumbnails.of(filePath)
                .size(width, height)
                .outputFormat("jpg")
                .toFile(thumbnailPath);
        
        return "/api/preview/image/" + getRelativePath(thumbnailPath);
    }
    
    /**
     * 读取文本文件内容
     */
    public String readTextFile(String filePath) throws IOException {
        try {
            String content = tika.parseToString(new File(filePath));
            if (content.length() > 50000) { // 限制文本长度
                content = content.substring(0, 50000) + "\n\n... (内容过长，已截断)";
            }
            return content;
        } catch (TikaException e) {
            log.error("读取文本文件失败: {}", e.getMessage(), e);
            // 回退到基本文件读取
            return Files.readString(Paths.get(filePath), StandardCharsets.UTF_8);
        }
    }
    
    /**
     * 创建HTML包装器
     */
    private String createHtmlWrapper(String title, String content) {
        return "<!DOCTYPE html>" +
                "<html><head>" +
                "<meta charset='UTF-8'>" +
                "<title>" + title + "</title>" +
                "<style>body{font-family:Arial,sans-serif;padding:20px;line-height:1.6;}" +
                "table{border-collapse:collapse;width:100%;}" +
                "td,th{border:1px solid #ddd;padding:8px;text-align:left;}" +
                "th{background-color:#f2f2f2;}</style>" +
                "</head><body>" +
                content +
                "</body></html>";
    }
    
    /**
     * 获取预览文件保存路径
     */
    private String getPreviewPath(String originalPath) {
        Path path = Paths.get(originalPath);
        String fileName = path.getFileName().toString();
        String nameWithoutExt = fileName.substring(0, fileName.lastIndexOf('.'));
        return path.getParent().toString() + "/preview/" + nameWithoutExt;
    }
    
    /**
     * 获取相对路径
     */
    private String getRelativePath(String fullPath) {
        String basePath = System.getProperty("user.dir") + "/" + uploadPath;
        if (fullPath.startsWith(basePath)) {
            return fullPath.substring(basePath.length() + 1).replace("\\", "/");
        }
        // 如果路径不以basePath开头，尝试从uploads开始
        if (fullPath.contains("/" + uploadPath + "/")) {
            int index = fullPath.indexOf("/" + uploadPath + "/");
            return fullPath.substring(index + uploadPath.length() + 2).replace("\\", "/");
        }
        return fullPath.replace("\\", "/");
    }
}