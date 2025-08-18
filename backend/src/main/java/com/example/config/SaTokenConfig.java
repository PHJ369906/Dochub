package com.example.config;

import cn.dev33.satoken.interceptor.SaInterceptor;
import cn.dev33.satoken.jwt.StpLogicJwtForSimple;
import cn.dev33.satoken.stp.StpLogic;
import cn.dev33.satoken.stp.StpUtil;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

/**
 * Sa-Token 配置类
 */
@Configuration
public class SaTokenConfig implements WebMvcConfigurer {

    /**
     * 注册 Sa-Token 拦截器，打开注解式鉴权功能
     */
    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        // 注册 Sa-Token 拦截器，校验规则为 StpUtil.checkLogin() 登录校验。
        registry.addInterceptor(new SaInterceptor(handle -> StpUtil.checkLogin()))
                .addPathPatterns("/**")
                .excludePathPatterns(
                        "/api/auth/login",
                        "/api/auth/register", 
                        "/api/auth/check",
                        "/api/files/popular",
                        "/api/files/latest",
                        "/api/files",
                        "/api/files/{id}",
                        "/api/categories/**",
                        "/api/preview/**",
                        "/api/test/**",
                        "/doc.html",
                        "/webjars/**",
                        "/swagger-resources/**",
                        "/v3/api-docs/**",
                        "/favicon.ico",
                        "/error",
                        "/actuator/**",
                        // 静态资源
                        "/",
                        "/index.html",
                        "/static/**",
                        "/assets/**",
                        "/*.js",
                        "/*.css",
                        "/*.png", 
                        "/*.jpg",
                        "/*.ico"
                );
    }

    /**
     * Sa-Token 整合 jwt (简单模式)
     * 暂时注释掉JWT模式，使用默认的Redis模式进行调试
     */
    // @Bean
    // public StpLogic getStpLogicJwt() {
    //     return new StpLogicJwtForSimple();
    // }
}
