package com.example.repository;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.example.entity.PolicyFile;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;
import org.apache.ibatis.annotations.Update;
import org.apache.ibatis.annotations.Insert;

import java.time.LocalDateTime;
import java.util.List;

/**
 * 政策文件数据访问层
 */
@Mapper
public interface PolicyFileMapper extends BaseMapper<PolicyFile> {
    
    /**
     * 根据分类ID查找文件
     */
    @Select("SELECT * FROM policy_file WHERE category_id = #{categoryId} AND is_enabled = 1")
    IPage<PolicyFile> findByCategoryIdAndEnabledTrue(Page<PolicyFile> page, @Param("categoryId") Long categoryId);

    /**
     * 根据分类ID列表查找文件（支持包含子分类）
     */
    @Select("<script>" +
            "SELECT * FROM policy_file WHERE is_enabled = 1" +
            "<if test='categoryIds != null and categoryIds.size() > 0'>" +
            " AND category_id IN " +
            "<foreach collection='categoryIds' item='id' open='(' separator=',' close=')'>" +
            "#{id}" +
            "</foreach>" +
            "</if>" +
            "</script>")
    IPage<PolicyFile> findByCategoryIdsAndEnabledTrue(Page<PolicyFile> page, @Param("categoryIds") List<Long> categoryIds);

    /**
     * 根据标题搜索文件
     */
    @Select("SELECT * FROM policy_file WHERE title LIKE CONCAT('%', #{title}, '%') AND is_enabled = 1")
    IPage<PolicyFile> findByTitleContainingIgnoreCaseAndEnabledTrue(Page<PolicyFile> page, @Param("title") String title);

    /**
     * 根据文件编号查找
     */
    @Select("SELECT * FROM policy_file WHERE document_number = #{documentNumber} AND is_enabled = 1")
    PolicyFile findByDocumentNumberAndEnabledTrue(@Param("documentNumber") String documentNumber);

    /**
     * 根据发布机关查找文件
     */
    @Select("SELECT * FROM policy_file WHERE issuing_authority LIKE CONCAT('%', #{authority}, '%') AND is_enabled = 1")
    IPage<PolicyFile> findByIssuingAuthorityContainingIgnoreCaseAndEnabledTrue(Page<PolicyFile> page, @Param("authority") String authority);

    /**
     * 查找公开的文件
     */
    @Select("SELECT * FROM policy_file WHERE is_public = 1 AND is_enabled = 1")
    IPage<PolicyFile> findByIsPublicTrueAndEnabledTrue(Page<PolicyFile> page);

    /**
     * 根据创建者查找文件
     */
    @Select("SELECT * FROM policy_file WHERE created_by = #{createdBy} AND is_enabled = 1")
    IPage<PolicyFile> findByCreatedByAndEnabledTrue(Page<PolicyFile> page, @Param("createdBy") Long createdBy);
    
    /**
     * 根据时间范围查找文件
     */
    @Select("SELECT * FROM policy_file WHERE create_time BETWEEN #{startTime} AND #{endTime} AND is_enabled = 1")
    IPage<PolicyFile> findByCreateTimeBetweenAndEnabledTrue(Page<PolicyFile> page, @Param("startTime") LocalDateTime startTime, @Param("endTime") LocalDateTime endTime);
    
    /**
     * 更新下载次数
     */
    @Update("UPDATE policy_file SET download_count = download_count + 1 WHERE id = #{id}")
    void incrementDownloadCount(@Param("id") Long id);
    
    /**
     * 更新查看次数
     */
    @Update("UPDATE policy_file SET view_count = view_count + 1 WHERE id = #{id}")
    void incrementViewCount(@Param("id") Long id);
    
    /**
     * 统计文件总数
     */
    @Select("SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1")
    long countByEnabledTrue();
    
    /**
     * 统计分类下的文件数量
     */
    @Select("SELECT COUNT(*) FROM policy_file WHERE category_id = #{categoryId} AND is_enabled = 1")
    long countByCategoryIdAndEnabledTrue(@Param("categoryId") Long categoryId);
    
    /**
     * 查找热门文件（按查看次数排序）
     */
    @Select("SELECT * FROM policy_file WHERE is_enabled = 1 ORDER BY view_count DESC LIMIT 10")
    List<PolicyFile> findTop10ByEnabledTrueOrderByViewCountDesc();
    
    /**
     * 查找最新文件
     */
    @Select("SELECT * FROM policy_file WHERE is_enabled = 1 ORDER BY create_time DESC LIMIT 10")
    List<PolicyFile> findTop10ByEnabledTrueOrderByCreateTimeDesc();
    
    /**
     * 简单文件插入
     */
    @Insert("INSERT INTO policy_file (title, file_path, original_name, file_size, created_by, is_enabled, is_public, download_count, view_count, create_time) VALUES (#{title}, #{filePath}, #{originalName}, #{fileSize}, #{createdBy}, 1, 1, 0, 0, NOW())")
    int insertFile(@Param("title") String title, 
                   @Param("filePath") String filePath,
                   @Param("originalName") String originalName,
                   @Param("fileSize") Long fileSize,
                   @Param("createdBy") Long createdBy);
}
