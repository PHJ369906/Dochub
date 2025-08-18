package com.example.exception;

import cn.dev33.satoken.exception.NotLoginException;
import cn.dev33.satoken.exception.NotPermissionException;
import cn.dev33.satoken.exception.NotRoleException;
import com.example.common.Result;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.dao.DataAccessException;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.validation.BindException;
import org.springframework.validation.FieldError;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.MissingServletRequestParameterException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.multipart.MaxUploadSizeExceededException;
import org.springframework.web.multipart.MultipartException;

import jakarta.servlet.http.HttpServletRequest;
import java.io.IOException;
import java.sql.SQLException;
import java.util.List;

/**
 * 全局异常处理器
 */
@RestControllerAdvice
public class GlobalExceptionHandler {
    
    private static final Logger log = LoggerFactory.getLogger(GlobalExceptionHandler.class);
    
    /**
     * 处理未登录异常
     */
    @ExceptionHandler(NotLoginException.class)
    public Result<Void> handleNotLoginException(NotLoginException e) {
        log.warn("用户未登录: {}", e.getMessage());
        return Result.error(401, "请先登录");
    }
    
    /**
     * 处理权限不足异常
     */
    @ExceptionHandler(NotPermissionException.class)
    public Result<Void> handleNotPermissionException(NotPermissionException e) {
        log.warn("权限不足: {}", e.getMessage());
        return Result.error(403, "权限不足");
    }
    
    /**
     * 处理角色不足异常
     */
    @ExceptionHandler(NotRoleException.class)
    public Result<Void> handleNotRoleException(NotRoleException e) {
        log.warn("角色权限不足: {}", e.getMessage());
        return Result.error(403, "角色权限不足");
    }
    
    /**
     * 处理参数校验异常
     */
    @ExceptionHandler(MethodArgumentNotValidException.class)
    public Result<Void> handleValidationException(MethodArgumentNotValidException e) {
        List<FieldError> fieldErrors = e.getBindingResult().getFieldErrors();
        StringBuilder sb = new StringBuilder();
        for (FieldError error : fieldErrors) {
            sb.append(error.getDefaultMessage()).append("; ");
        }
        log.warn("参数校验失败: {}", sb.toString());
        return Result.error(400, sb.toString());
    }
    
    /**
     * 处理绑定异常
     */
    @ExceptionHandler(BindException.class)
    public Result<Void> handleBindException(BindException e, HttpServletRequest request) {
        List<FieldError> fieldErrors = e.getBindingResult().getFieldErrors();
        StringBuilder sb = new StringBuilder();
        for (FieldError error : fieldErrors) {
            sb.append(error.getDefaultMessage()).append("; ");
        }
        log.warn("参数绑定失败 - URL: {}, 错误: {}", request.getRequestURI(), sb.toString());
        return Result.error(400, sb.toString());
    }
    
    /**
     * 处理文件上传大小超限异常
     */
    @ExceptionHandler(MaxUploadSizeExceededException.class)
    public Result<Void> handleMaxUploadSizeExceededException(MaxUploadSizeExceededException e, HttpServletRequest request) {
        log.warn("文件上传大小超限 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage());
        return Result.error(413, "文件大小超出限制");
    }
    
    /**
     * 处理文件上传异常
     */
    @ExceptionHandler(MultipartException.class)
    public Result<Void> handleMultipartException(MultipartException e, HttpServletRequest request) {
        log.warn("文件上传异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage());
        return Result.error(400, "文件上传失败: " + e.getMessage());
    }
    
    /**
     * 处理IO异常
     */
    @ExceptionHandler(IOException.class)
    public Result<Void> handleIOException(IOException e, HttpServletRequest request) {
        log.error("IO异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage(), e);
        return Result.error(500, "文件操作失败");
    }
    
    /**
     * 处理数据库异常
     */
    @ExceptionHandler(SQLException.class)
    public Result<Void> handleSQLException(SQLException e, HttpServletRequest request) {
        log.error("数据库异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage(), e);
        return Result.error(500, "数据库操作失败");
    }
    
    /**
     * 处理数据访问异常
     */
    @ExceptionHandler(DataAccessException.class)
    public Result<Void> handleDataAccessException(DataAccessException e, HttpServletRequest request) {
        log.error("数据访问异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage(), e);
        return Result.error(500, "数据访问失败");
    }
    
    /**
     * 处理缺少请求参数异常
     */
    @ExceptionHandler(MissingServletRequestParameterException.class)
    public Result<Void> handleMissingServletRequestParameterException(MissingServletRequestParameterException e, HttpServletRequest request) {
        log.warn("缺少请求参数 - URL: {}, 参数: {}", request.getRequestURI(), e.getParameterName());
        return Result.error(400, "缺少必要参数: " + e.getParameterName());
    }
    
    /**
     * 处理HTTP消息不可读异常
     */
    @ExceptionHandler(HttpMessageNotReadableException.class)
    public Result<Void> handleHttpMessageNotReadableException(HttpMessageNotReadableException e, HttpServletRequest request) {
        log.warn("请求数据格式错误 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage());
        return Result.error(400, "请求数据格式错误");
    }
    
    /**
     * 处理业务异常
     */
    @ExceptionHandler(RuntimeException.class)
    public Result<Void> handleRuntimeException(RuntimeException e, HttpServletRequest request) {
        log.error("业务异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage(), e);
        return Result.error(500, e.getMessage());
    }
    
    /**
     * 处理其他未知异常
     */
    @ExceptionHandler(Exception.class)
    public Result<Void> handleException(Exception e, HttpServletRequest request) {
        log.error("系统异常 - URL: {}, 错误: {}", request.getRequestURI(), e.getMessage(), e);
        return Result.error(500, "系统内部错误");
    }
}
