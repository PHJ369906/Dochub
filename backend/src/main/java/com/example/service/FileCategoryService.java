package com.example.service;

import cn.dev33.satoken.stp.StpUtil;
import com.example.entity.FileCategory;
import com.example.repository.FileCategoryMapper;
import org.springframework.beans.factory.annotation.Autowired;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.util.StringUtils;

import java.util.List;

/**
 * 文件分类服务
 */
@Slf4j
@Service
public class FileCategoryService {
    
    @Autowired
    private FileCategoryMapper categoryMapper;
    
    /**
     * 获取所有分类（树形结构）
     */
    public List<FileCategory> getAllCategories() {
        return categoryMapper.findCategoryTree();
    }
    
    /**
     * 获取顶级分类
     */
    public List<FileCategory> getRootCategories() {
        return categoryMapper.findByParentIdIsNullAndEnabledTrueOrderBySortOrderAsc();
    }
    
    /**
     * 获取子分类
     */
    public List<FileCategory> getChildCategories(Long parentId) {
        return categoryMapper.findByParentIdAndEnabledTrueOrderBySortOrderAsc(parentId);
    }
    
    /**
     * 根据ID获取分类
     */
    public FileCategory getCategoryById(Long id) {
        FileCategory category = categoryMapper.selectById(id);
        if (category == null || !category.getEnabled()) {
            throw new RuntimeException("分类不存在");
        }
        return category;
    }
    
    /**
     * 创建分类
     */
    @Transactional
    public FileCategory createCategory(String name, String description, Long parentId, Integer sortOrder) {
        // 检查管理员权限
        if (!StpUtil.hasRole("admin")) {
            throw new RuntimeException("只有管理员可以创建分类");
        }
        
        // 验证参数
        if (!StringUtils.hasText(name)) {
            throw new RuntimeException("分类名称不能为空");
        }
        
        // 检查同级分类名称是否重复
        if (categoryMapper.existsByNameAndParentId(name, parentId)) {
            throw new RuntimeException("同级分类中已存在相同名称");
        }
        
        // 验证父分类是否存在
        if (parentId != null) {
            getCategoryById(parentId);
        }
        
        FileCategory category = new FileCategory();
        category.setName(name);
        category.setDescription(description);
        category.setParentId(parentId);
        category.setSortOrder(sortOrder != null ? sortOrder : 0);
        category.setCreatedBy(StpUtil.getLoginIdAsLong());
        
        categoryMapper.insert(category);
        return category;
    }
    
    /**
     * 更新分类
     */
    @Transactional
    public FileCategory updateCategory(Long id, String name, String description, Integer sortOrder, Boolean enabled) {
        // 检查管理员权限
        if (!StpUtil.hasRole("admin")) {
            throw new RuntimeException("只有管理员可以修改分类");
        }
        
        FileCategory category = getCategoryById(id);
        
        // 更新名称时检查重复
        if (StringUtils.hasText(name) && !name.equals(category.getName())) {
            if (categoryMapper.existsByNameAndParentId(name, category.getParentId())) {
                throw new RuntimeException("同级分类中已存在相同名称");
            }
            category.setName(name);
        }
        
        if (description != null) {
            category.setDescription(description);
        }
        
        if (sortOrder != null) {
            category.setSortOrder(sortOrder);
        }
        
        if (enabled != null) {
            category.setEnabled(enabled);
        }
        
        categoryMapper.updateById(category);
        return category;
    }
    
    /**
     * 删除分类
     */
    @Transactional
    public void deleteCategory(Long id) {
        // 检查管理员权限
        if (!StpUtil.hasRole("admin")) {
            throw new RuntimeException("只有管理员可以删除分类");
        }
        
        FileCategory category = getCategoryById(id);
        
        // 检查是否有子分类
        List<FileCategory> children = getChildCategories(id);
        if (!children.isEmpty()) {
            throw new RuntimeException("该分类下还有子分类，无法删除");
        }
        
        // 检查是否有文件使用该分类
        long fileCount = categoryMapper.countByCategoryIdAndEnabledTrue(id);
        if (fileCount > 0) {
            throw new RuntimeException("该分类下还有文件，无法删除");
        }
        
        // 软删除
        category.setEnabled(false);
        categoryMapper.updateById(category);
    }
    
    /**
     * 移动分类
     */
    @Transactional
    public FileCategory moveCategory(Long id, Long newParentId) {
        // 检查管理员权限
        if (!StpUtil.hasRole("admin")) {
            throw new RuntimeException("只有管理员可以移动分类");
        }
        
        FileCategory category = getCategoryById(id);
        
        // 验证新父分类
        if (newParentId != null) {
            FileCategory newParent = getCategoryById(newParentId);
            
            // 检查是否会形成循环引用
            if (isDescendant(newParentId, id)) {
                throw new RuntimeException("不能将分类移动到其子分类下");
            }
        }
        
        // 检查新位置是否有同名分类
        if (categoryMapper.existsByNameAndParentId(category.getName(), newParentId)) {
            throw new RuntimeException("目标位置已存在同名分类");
        }
        
        category.setParentId(newParentId);
        categoryMapper.updateById(category);
        return category;
    }
    
    /**
     * 检查是否为子孙分类
     */
    private boolean isDescendant(Long ancestorId, Long descendantId) {
        if (ancestorId.equals(descendantId)) {
            return true;
        }
        
        FileCategory category = categoryMapper.selectById(ancestorId);
        if (category != null && category.getParentId() != null) {
            return isDescendant(category.getParentId(), descendantId);
        }
        
        return false;
    }
    
    /**
     * 获取分类统计信息
     */
    public long getCategoryFileCount(Long categoryId) {
        return categoryMapper.countByCategoryIdAndEnabledTrue(categoryId);
    }
}
