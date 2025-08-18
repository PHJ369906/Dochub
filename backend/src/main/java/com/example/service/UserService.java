package com.example.service;

import cn.dev33.satoken.stp.StpUtil;
import com.example.dto.LoginRequest;
import com.example.dto.RegisterRequest;
import com.example.dto.UserResponse;
import com.example.entity.User;
import com.example.repository.UserMapper;
import org.springframework.beans.factory.annotation.Autowired;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.HashMap;
import java.util.Map;

/**
 * 用户服务类
 */
@Service
public class UserService {
    
    @Autowired
    private UserMapper userMapper;
    
    /**
     * 用户登录
     */
    public Map<String, Object> login(LoginRequest request) {
        User user = userMapper.findByUsername(request.getUsername());
        
        if (user == null) {
            throw new RuntimeException("用户名或密码错误");
        }
        
        if (!user.getEnabled()) {
            throw new RuntimeException("账户已被禁用");
        }
        
        // 临时使用明文密码比较，生产环境建议使用加密密码
        if (!request.getPassword().equals(user.getPassword())) {
            throw new RuntimeException("用户名或密码错误");
        }
        
        // 登录成功，生成token
        StpUtil.login(user.getId());
        String token = StpUtil.getTokenValue();
        
        Map<String, Object> result = new HashMap<>();
        result.put("token", token);
        result.put("user", UserResponse.fromUser(user));
        
        return result;
    }
    
    /**
     * 用户注册
     */
    @Transactional
    public UserResponse register(RegisterRequest request) {
        // 验证确认密码
        if (!request.getPassword().equals(request.getConfirmPassword())) {
            throw new RuntimeException("两次输入的密码不一致");
        }
        
        // 检查用户名是否已存在
        if (userMapper.existsByUsername(request.getUsername())) {
            throw new RuntimeException("用户名已存在");
        }
        
        // 检查邮箱是否已存在
        if (request.getEmail() != null && !request.getEmail().isEmpty() 
            && userMapper.existsByEmail(request.getEmail())) {
            throw new RuntimeException("邮箱已存在");
        }
        
        // 创建新用户
        User user = new User();
        user.setUsername(request.getUsername());
        // 临时使用明文密码，生产环境建议使用加密密码
        user.setPassword(request.getPassword());
        user.setEmail(request.getEmail());
        user.setRole("user");
        user.setEnabled(true);
        
        userMapper.insert(user);
        return UserResponse.fromUser(user);
    }
    
    /**
     * 获取当前用户信息
     */
    public UserResponse getCurrentUser() {
        Long userId = StpUtil.getLoginIdAsLong();
        User user = userMapper.selectById(userId);
        
        if (user == null) {
            throw new RuntimeException("用户不存在");
        }
        
        return UserResponse.fromUser(user);
    }
    
    /**
     * 获取用户列表（分页）
     */
    public IPage<UserResponse> getUserList(Page<User> page, String username, String role) {
        com.baomidou.mybatisplus.core.conditions.query.QueryWrapper<User> queryWrapper = 
            new com.baomidou.mybatisplus.core.conditions.query.QueryWrapper<>();
        
        if (username != null && !username.trim().isEmpty()) {
            queryWrapper.like("username", username.trim());
        }
        
        if (role != null && !role.trim().isEmpty()) {
            queryWrapper.eq("role", role.trim());
        }
        
        IPage<User> userPage = userMapper.selectPage(page, queryWrapper);
        return userPage.convert(UserResponse::fromUser);
    }
    
    /**
     * 根据ID获取用户
     */
    public UserResponse getUserById(Long id) {
        User user = userMapper.selectById(id);
        if (user == null) {
            throw new RuntimeException("用户不存在");
        }
        return UserResponse.fromUser(user);
    }
    
    /**
     * 切换用户状态
     */
    @Transactional
    public String toggleUserStatus(Long id) {
        User user = userMapper.selectById(id);
        if (user == null) {
            throw new RuntimeException("用户不存在");
        }
        
        boolean newStatus = !user.getEnabled();
        user.setEnabled(newStatus);
        userMapper.updateById(user);
        
        return newStatus ? "用户已启用" : "用户已禁用";
    }
}
