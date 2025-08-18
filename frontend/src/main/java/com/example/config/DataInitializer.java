package com.example.config;

import com.example.entity.User;
import com.example.repository.UserMapper;
import org.springframework.beans.factory.annotation.Autowired;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

/**
 * 数据初始化器
 * 注意：这里使用明文密码仅用于演示，生产环境请使用加密密码
 */
@Component
public class DataInitializer implements CommandLineRunner {

    private static final Logger log = LoggerFactory.getLogger(DataInitializer.class);

    @Autowired
    private UserMapper userMapper;

    @Override
    public void run(String... args) throws Exception {
        initAdminUser();
    }

    /**
     * 初始化管理员用户
     * 注意：这里使用明文密码仅用于演示，生产环境请使用BCrypt加密
     */
    private void initAdminUser() {
        if (!userMapper.existsByUsername("admin")) {
            User admin = new User();
            admin.setUsername("admin");
            // 临时使用明文密码，建议后续集成Spring Security进行加密
            admin.setPassword("admin123");
            admin.setEmail("admin@example.com");
            admin.setRole("admin");
            admin.setEnabled(true);

            userMapper.insert(admin);
            log.info("管理员账户初始化完成: admin/admin123");
        }

        if (!userMapper.existsByUsername("user")) {
            User user = new User();
            user.setUsername("user");
            // 临时使用明文密码，建议后续集成Spring Security进行加密
            user.setPassword("user123");
            user.setEmail("user@example.com");
            user.setRole("user");
            user.setEnabled(true);

            userMapper.insert(user);
            log.info("普通用户账户初始化完成: user/user123");
        }
    }
}
