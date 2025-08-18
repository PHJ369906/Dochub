package com.example.config;

import cn.dev33.satoken.stp.StpInterface;
import com.example.entity.User;
import com.example.repository.UserMapper;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.util.ArrayList;
import java.util.List;

/**
 * 自定义权限验证接口扩展
 */
@Component
public class StpInterfaceImpl implements StpInterface {

    @Autowired
    private UserMapper userMapper;

    /**
     * 返回一个账号所拥有的权限码集合 
     */
    @Override
    public List<String> getPermissionList(Object loginId, String loginType) {
        // 本项目暂时不使用权限功能，返回空集合
        return new ArrayList<>();
    }

    /**
     * 返回一个账号所拥有的角色标识集合 (权限与角色可分开校验)
     */
    @Override
    public List<String> getRoleList(Object loginId, String loginType) {
        List<String> roles = new ArrayList<>();
        
        try {
            // 根据用户ID查询用户信息
            Long userId = Long.valueOf(loginId.toString());
            User user = userMapper.selectById(userId);
            
            if (user != null && user.getRole() != null) {
                // 添加用户角色
                roles.add(user.getRole());
            }
        } catch (Exception e) {
            System.err.println("获取用户角色失败: " + e.getMessage());
        }
        
        return roles;
    }
}
