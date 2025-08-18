package com.example.controller;

import cn.dev33.satoken.annotation.SaCheckRole;
import cn.dev33.satoken.stp.StpUtil;
import com.example.common.Result;
import com.example.dto.LoginRequest;
import com.example.dto.RegisterRequest;
import com.example.dto.UserResponse;
import com.example.service.UserService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import org.springframework.beans.factory.annotation.Autowired;
import com.baomidou.mybatisplus.core.metadata.IPage;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.Map;

/**
 * 认证控制器
 */
@Tag(name = "认证管理", description = "用户认证相关接口")
@RestController
@RequestMapping("/api/auth")
public class AuthController {
    
    @Autowired
    private UserService userService;
    
    /**
     * 用户登录
     */
    @Operation(summary = "用户登录")
    @PostMapping(value = "/login", produces = "application/json; charset=utf-8")
    public ResponseEntity<String> login(@Valid @RequestBody LoginRequest request) {
        try {
            Map<String, Object> result = userService.login(request);
            UserResponse user = (UserResponse) result.get("user");
            String json = String.format(
                "{\"code\":200,\"message\":\"登录成功\",\"data\":{\"token\":\"%s\",\"user\":{\"id\":%d,\"username\":\"%s\",\"email\":\"%s\",\"role\":\"%s\"}}}",
                result.get("token"),
                user.getId(),
                user.getUsername(),
                user.getEmail() != null ? user.getEmail() : "", 
                user.getRole()
            );
            return ResponseEntity.ok()
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(json);
        } catch (Exception e) {
            String errorJson = String.format("{\"code\":500,\"message\":\"%s\"}", e.getMessage().replace("\"", "\\\""));
            return ResponseEntity.ok()
                    .header("Content-Type", "application/json; charset=utf-8")
                    .body(errorJson);
        }
    }
    
    /**
     * 用户注册
     */
    @Operation(summary = "用户注册")
    @PostMapping("/register")
    public Result<UserResponse> register(@Valid @RequestBody RegisterRequest request) {
        try {
            UserResponse user = userService.register(request);
            return Result.success("注册成功", user);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 用户登出
     */
    @Operation(summary = "用户登出")
    @PostMapping("/logout")
    public Result<String> logout() {
        StpUtil.logout();
        return Result.success("登出成功");
    }
    
    /**
     * 获取当前用户信息
     */
    @Operation(summary = "获取当前用户信息")
    @GetMapping("/userinfo")
    public Result<UserResponse> getUserInfo() {
        try {
            UserResponse user = userService.getCurrentUser();
            return Result.success(user);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 获取用户列表（管理员权限）
     */
    @Operation(summary = "获取用户列表")
    @GetMapping("/users")
    @SaCheckRole("admin")
    public Result<IPage<UserResponse>> getUserList(
            @RequestParam(defaultValue = "1") int current,
            @RequestParam(defaultValue = "10") int size) {
        try {
            Page<com.example.entity.User> page = new Page<>(current, size);
            IPage<UserResponse> users = userService.getUserList(page);
            return Result.success(users);
        } catch (Exception e) {
            return Result.error(e.getMessage());
        }
    }
    
    /**
     * 检查登录状态
     */
    @Operation(summary = "检查登录状态")
    @GetMapping("/check")
    public Result<Boolean> checkLogin() {
        boolean isLogin = StpUtil.isLogin();
        return Result.success(isLogin);
    }
}
