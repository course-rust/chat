use std::mem;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;

use crate::{error::AppError, User};
impl User {
    /// Find user by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as("select id,fullname,email,created_at from users where email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;

        Ok(user)
    }
    /// Create a new user
    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        let passoword_hash = hash_password(password)?;
        let new_user = sqlx::query_as(
            r#"
            insert into users (fullname, email, password_hash)
            values ($1, $2, $3)
            returning id, fullname, email, created_at
            "#,
        )
        .bind(fullname)
        .bind(email)
        .bind(passoword_hash)
        .fetch_one(pool)
        .await?;

        Ok(new_user)
    }
    /// Verifies password and email
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        // 先查询 user, 取出 password_hash 并进行解密
        let user: Option<User> = sqlx::query_as(
            "select id,fullname,email, password_hash,created_at from users where email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(mut u) => {
                // 取走 password_hash 数据， 原位置 Option::default
                let password_hash = mem::take(&mut u.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(u))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

/// 密码生成加密字符串, 使用 Argon2 crate
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    let password = password.as_bytes();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(password_hash)?;
    let ret = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(ret)
}
