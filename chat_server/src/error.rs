use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("SqlxError error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Passwords hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}
