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
     * 统计分类下的文件数量
     */
    @Select("SELECT COUNT(*) FROM policy_file WHERE category_id = #{categoryId} AND is_enabled = 1")
    long countByCategoryIdAndEnabledTrue(@Param("categoryId") Long categoryId);
}
