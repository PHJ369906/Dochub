package com.example.repository;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.example.entity.FileCategory;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;

import java.util.List;

/**
 * 文件分类数据访问层
 */
@Mapper
public interface FileCategoryMapper extends BaseMapper<FileCategory> {
    
    /**
     * 查找顶级分类（父分类为空）
     */
    @Select("SELECT * FROM file_category WHERE parent_id IS NULL AND is_enabled = 1 ORDER BY sort_order ASC")
    List<FileCategory> findByParentIdIsNullAndEnabledTrueOrderBySortOrderAsc();
    
    /**
     * 根据父分类ID查找子分类
     */
    @Select("SELECT * FROM file_category WHERE parent_id = #{parentId} AND is_enabled = 1 ORDER BY sort_order ASC")
    List<FileCategory> findByParentIdAndEnabledTrueOrderBySortOrderAsc(@Param("parentId") Long parentId);
    
    /**
     * 检查分类名称是否存在
     */
    @Select("SELECT COUNT(*) > 0 FROM file_category WHERE name = #{name} AND parent_id = #{parentId}")
    boolean existsByNameAndParentId(@Param("name") String name, @Param("parentId") Long parentId);
    
    /**
     * 查找所有启用的分类
     */
    @Select("SELECT * FROM file_category WHERE is_enabled = 1 ORDER BY sort_order ASC")
    List<FileCategory> findByEnabledTrueOrderBySortOrderAsc();
    
    /**
     * 查询分类树结构
     */
    @Select("SELECT * FROM file_category WHERE is_enabled = 1 ORDER BY ISNULL(parent_id), parent_id ASC, sort_order ASC")
    List<FileCategory> findCategoryTree();

    /**
     * 查询分类树结构（带递归文件统计）
     */
    @Select("SELECT c.*, " +
            "(" +
                "WITH RECURSIVE category_hierarchy AS (" +
                    "SELECT id FROM file_category WHERE id = c.id AND is_enabled = 1 " +
                    "UNION ALL " +
                    "SELECT fc.id FROM file_category fc " +
                    "INNER JOIN category_hierarchy ch ON fc.parent_id = ch.id " +
                    "WHERE fc.is_enabled = 1" +
                ") " +
                "SELECT COUNT(*) FROM policy_file p " +
                "WHERE p.category_id IN (SELECT id FROM category_hierarchy) AND p.is_enabled = 1" +
            ") as file_count " +
            "FROM file_category c WHERE c.is_enabled = 1 " +
            "ORDER BY ISNULL(c.parent_id), c.parent_id ASC, c.sort_order ASC")
    List<FileCategory> findCategoryTreeWithFileCount();

    /**
     * 统计分类下的文件数量
     */
    @Select("SELECT COUNT(*) FROM policy_file WHERE category_id = #{categoryId} AND is_enabled = 1")
    long countByCategoryIdAndEnabledTrue(@Param("categoryId") Long categoryId);

    /**
     * 获取分类及其所有子分类ID（递归）
     */
    @Select("WITH RECURSIVE category_hierarchy AS (" +
                "SELECT id FROM file_category WHERE id = #{categoryId} AND is_enabled = 1 " +
                "UNION ALL " +
                "SELECT fc.id FROM file_category fc " +
                "INNER JOIN category_hierarchy ch ON fc.parent_id = ch.id " +
                "WHERE fc.is_enabled = 1" +
            ") " +
            "SELECT id FROM category_hierarchy")
    List<Long> findCategoryAndChildrenIds(@Param("categoryId") Long categoryId);
}
