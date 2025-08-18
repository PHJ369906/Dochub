package com.example.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

/**
 * Web MVC 配置
 */
@Configuration
public class WebMvcConfig implements WebMvcConfigurer {
    
    // CORS配置移到了专门的CorsConfig类
    // Jackson配置移到了专门的JacksonConfig类
    // 使用Spring Boot默认的内容协商配置
}
