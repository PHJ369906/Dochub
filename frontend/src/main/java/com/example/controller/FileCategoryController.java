package com.example.controller;

import cn.dev33.satoken.annotation.SaCheckRole;
import com.example.common.Result;
import com.example.dto.CategoryCreateRequest;
import com.example.dto.CategoryMoveRequest;
import com.example.dto.CategoryUpdateRequest;
import com.example.entity.FileCategory;
import com.example.service.FileCategoryService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * 文件分类控制器
 */
@Tag(name = "文件分类管理", description = "文件分类管理相关接口")
@RestController
@RequestMapping("/api/categories")
public class FileCategoryController {
    
    @Autowired
    private FileCategoryService categoryService;
    
    /**
     * 获取所有分类（树形结构）
     */
    @Operation(summary = "获取所有分类")
    @GetMapping(produces = "application/json; charset=utf-8")
    public Result<List<FileCategory>> getAllCategories() {
        try {
            List<FileCategory> categories = categoryService.getAllCategories();
            return Result.success(categories);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取顶级分类
     */
    @Operation(summary = "获取顶级分类")
    @GetMapping("/root")
    public Result<List<FileCategory>> getRootCategories() {
        try {
            List<FileCategory> categories = categoryService.getRootCategories();
            return Result.success(categories);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取子分类
     */
    @Operation(summary = "获取子分类")
    @GetMapping("/{parentId}/children")
    public Result<List<FileCategory>> getChildCategories(@PathVariable Long parentId) {
        try {
            List<FileCategory> categories = categoryService.getChildCategories(parentId);
            return Result.success(categories);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 根据ID获取分类
     */
    @Operation(summary = "获取分类详情")
    @GetMapping("/{id}")
    public Result<FileCategory> getCategoryById(@PathVariable Long id) {
        try {
            FileCategory category = categoryService.getCategoryById(id);
            return Result.success(category);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 创建分类
     */
    @Operation(summary = "创建分类")
    @PostMapping
    @SaCheckRole("admin")
    public Result<FileCategory> createCategory(@Valid @RequestBody CategoryCreateRequest request) {
        try {
            FileCategory category = categoryService.createCategory(
                request.getName(), 
                request.getDescription(), 
                request.getParentId(), 
                request.getSortOrder()
            );
            return Result.success("分类创建成功", category);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 更新分类
     */
    @Operation(summary = "更新分类")
    @PutMapping("/{id}")
    @SaCheckRole("admin")
    public Result<FileCategory> updateCategory(
            @PathVariable Long id,
            @Valid @RequestBody CategoryUpdateRequest request) {
        try {
            FileCategory category = categoryService.updateCategory(
                id, 
                request.getName(), 
                request.getDescription(), 
                request.getSortOrder(), 
                request.getEnabled()
            );
            return Result.success("分类更新成功", category);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 删除分类
     */
    @Operation(summary = "删除分类")
    @DeleteMapping("/{id}")
    @SaCheckRole("admin")
    public Result<String> deleteCategory(@PathVariable Long id) {
        try {
            categoryService.deleteCategory(id);
            return Result.success("分类删除成功");
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 移动分类
     */
    @Operation(summary = "移动分类")
    @PutMapping("/{id}/move")
    @SaCheckRole("admin")
    public Result<FileCategory> moveCategory(
            @PathVariable Long id,
            @Valid @RequestBody CategoryMoveRequest request) {
        try {
            FileCategory category = categoryService.moveCategory(id, request.getNewParentId());
            return Result.success("分类移动成功", category);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取分类文件统计
     */
    @Operation(summary = "获取分类文件统计")
    @GetMapping("/{id}/stats")
    public Result<Long> getCategoryStats(@PathVariable Long id) {
        try {
            long fileCount = categoryService.getCategoryFileCount(id);
            return Result.success(fileCount);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
}
