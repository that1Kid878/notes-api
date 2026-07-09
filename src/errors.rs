use crate::responses::AppError;
use sqlx::Error as SqlError;
use sqlx::error::ErrorKind;

pub async fn sqlx_error_handler(err: SqlError) -> AppError {
    match err {
        SqlError::RowNotFound => AppError::NotFound {
            message: "Item not found".to_string(),
        },
        SqlError::Database(db_err) => match db_err.kind() {
            ErrorKind::NotNullViolation => AppError::BadRequest {
                message: format!("Not null constraint violation: {}", db_err.message()),
            },
            ErrorKind::UniqueViolation => AppError::BadRequest {
                message: format!("Unqiue constraint violation: {}", db_err.message()),
            },

            _ => AppError::InternalServerError {
                message: format!("Database error: {}", db_err.message()),
            },
        },
        _ => AppError::InternalServerError {
            message: format!("Database error: {}", err),
        },
    }
}
