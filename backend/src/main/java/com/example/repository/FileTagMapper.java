package com.example.repository;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.example.entity.FileTag;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;

import java.util.List;
import java.util.Set;

/**
 * 文件标签数据访问层
 */
@Mapper
public interface FileTagMapper extends BaseMapper<FileTag> {
    
    /**
     * 根据标签名称查找
     */
    @Select("SELECT * FROM file_tag WHERE name = #{name}")
    FileTag findByName(@Param("name") String name);
    
    /**
     * 检查标签名称是否存在
     */
    @Select("SELECT COUNT(*) > 0 FROM file_tag WHERE name = #{name}")
    boolean existsByName(@Param("name") String name);
    
    /**
     * 根据标签名称列表查找标签
     */
    @Select("<script>" +
            "SELECT * FROM file_tag WHERE name IN " +
            "<foreach item='name' collection='names' open='(' separator=',' close=')'>" +
            "#{name}" +
            "</foreach>" +
            "</script>")
    List<FileTag> findByNameIn(@Param("names") Set<String> names);
    
    /**
     * 查找热门标签（按使用次数排序）
     * 注意：这里需要根据实际的文件标签关联表来实现
     */
    @Select("SELECT t.*, COUNT(pft.file_id) as usage_count " +
            "FROM file_tag t " +
            "LEFT JOIN policy_file_tag pft ON t.id = pft.tag_id " +
            "GROUP BY t.id " +
            "ORDER BY usage_count DESC")
    List<FileTag> findPopularTags();
    
    /**
     * 根据关键词搜索标签
     */
    @Select("SELECT * FROM file_tag WHERE name LIKE CONCAT('%', #{keyword}, '%')")
    List<FileTag> findByNameContainingIgnoreCase(@Param("keyword") String keyword);
}
