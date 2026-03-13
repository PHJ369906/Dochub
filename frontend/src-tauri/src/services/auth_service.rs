use rusqlite::params;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::{User, LoginResponse, PageResult, UserListParams};

pub struct Session {
    pub user_id: i64,
    pub token: String,
    pub created_at: String,
}

pub struct AuthService {
    sessions: Mutex<HashMap<String, Session>>,
}

impl AuthService {
    pub fn new() -> Self {
        AuthService {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn login(&self, db: &Database, username: &str, password: &str) -> Result<LoginResponse, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let result = conn.query_row(
            "SELECT id, username, password, email, role, avatar, is_enabled, create_time, update_time
             FROM sys_user WHERE username = ?1",
            params![username],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, bool>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                ))
            },
        );

        let (id, uname, pwd_hash, email, role, avatar, enabled, create_time, update_time) = match result {
            Ok(data) => data,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(AppError::Unauthorized("用户名或密码错误".to_string()));
            }
            Err(e) => return Err(AppError::Database(e.to_string())),
        };

        if !enabled {
            return Err(AppError::Unauthorized("账户已被禁用".to_string()));
        }

        // 验证密码
        let valid = bcrypt::verify(password, &pwd_hash)
            .map_err(|_| AppError::Unauthorized("用户名或密码错误".to_string()))?;

        if !valid {
            return Err(AppError::Unauthorized("用户名或密码错误".to_string()));
        }

        // 生成 token
        let token = uuid::Uuid::new_v4().to_string();

        let user = User {
            id,
            username: uname,
            email,
            role,
            avatar,
            enabled,
            create_time,
            update_time,
        };

        // 保存会话
        let mut sessions = self.sessions.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        sessions.insert(token.clone(), Session {
            user_id: id,
            token: token.clone(),
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });

        Ok(LoginResponse { token, user })
    }

    pub fn register(&self, db: &Database, username: &str, password: &str, email: Option<&str>) -> Result<User, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        // 检查用户名是否已存在
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM sys_user WHERE username = ?1",
            params![username],
            |row| row.get(0),
        )?;

        if exists {
            return Err(AppError::BadRequest("用户名已存在".to_string()));
        }

        let pwd_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        conn.execute(
            "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, 'user')",
            params![username, pwd_hash, email],
        )?;

        let id = conn.last_insert_rowid();
        let user = self.get_user_by_id_inner(&conn, id)?;
        Ok(user)
    }

    pub fn logout(&self, token: &str) -> Result<(), AppError> {
        let mut sessions = self.sessions.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        sessions.remove(token);
        Ok(())
    }

    pub fn get_current_user(&self, db: &Database, token: &str) -> Result<User, AppError> {
        let user_id = self.validate_token(token)?;
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        self.get_user_by_id_inner(&conn, user_id)
    }

    pub fn validate_token(&self, token: &str) -> Result<i64, AppError> {
        let sessions = self.sessions.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        match sessions.get(token) {
            Some(session) => Ok(session.user_id),
            None => Err(AppError::Unauthorized("无效的登录凭证".to_string())),
        }
    }

    pub fn check_login(&self, token: &str) -> bool {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        sessions.contains_key(token)
    }

    pub fn get_user_list(&self, db: &Database, params: &UserListParams) -> Result<PageResult<User>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let page = params.current.unwrap_or(1).max(1);
        let size = params.size.unwrap_or(10).max(1);
        let offset = (page - 1) * size;

        let mut conditions = vec!["1=1".to_string()];
        let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref username) = params.username {
            if !username.is_empty() {
                conditions.push(format!("username LIKE ?{}", bind_values.len() + 1));
                bind_values.push(Box::new(format!("%{}%", username)));
            }
        }

        if let Some(ref role) = params.role {
            if !role.is_empty() {
                conditions.push(format!("role = ?{}", bind_values.len() + 1));
                bind_values.push(Box::new(role.clone()));
            }
        }

        let where_clause = conditions.join(" AND ");

        // 查询总数
        let count_sql = format!("SELECT COUNT(*) FROM sys_user WHERE {}", where_clause);
        let total: i64 = {
            let mut stmt = conn.prepare(&count_sql)?;
            let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
            stmt.query_row(refs.as_slice(), |row| row.get(0))?
        };

        // 查询列表
        let query_sql = format!(
            "SELECT id, username, email, role, avatar, is_enabled, create_time, update_time
             FROM sys_user WHERE {} ORDER BY id ASC LIMIT ?{} OFFSET ?{}",
            where_clause,
            bind_values.len() + 1,
            bind_values.len() + 2
        );

        bind_values.push(Box::new(size));
        bind_values.push(Box::new(offset));

        let mut stmt = conn.prepare(&query_sql)?;
        let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
        let users = stmt.query_map(refs.as_slice(), |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                role: row.get(3)?,
                avatar: row.get(4)?,
                enabled: row.get(5)?,
                create_time: row.get(6)?,
                update_time: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(PageResult {
            records: users,
            total,
            page,
            size,
        })
    }

    pub fn get_user_detail(&self, db: &Database, user_id: i64) -> Result<User, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        self.get_user_by_id_inner(&conn, user_id)
    }

    pub fn toggle_user_status(&self, db: &Database, user_id: i64) -> Result<String, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let current_enabled: bool = conn.query_row(
            "SELECT is_enabled FROM sys_user WHERE id = ?1",
            params![user_id],
            |row| row.get(0),
        ).map_err(|_| AppError::NotFound("用户不存在".to_string()))?;

        let new_enabled = !current_enabled;
        conn.execute(
            "UPDATE sys_user SET is_enabled = ?1 WHERE id = ?2",
            params![new_enabled, user_id],
        )?;

        let msg = if new_enabled { "用户已启用" } else { "用户已禁用" };
        Ok(msg.to_string())
    }

    fn get_user_by_id_inner(&self, conn: &Connection, user_id: i64) -> Result<User, AppError> {
        conn.query_row(
            "SELECT id, username, email, role, avatar, is_enabled, create_time, update_time
             FROM sys_user WHERE id = ?1",
            params![user_id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    role: row.get(3)?,
                    avatar: row.get(4)?,
                    enabled: row.get(5)?,
                    create_time: row.get(6)?,
                    update_time: row.get(7)?,
                })
            },
        ).map_err(|_| AppError::NotFound("用户不存在".to_string()))
    }
}

use rusqlite::Connection;
