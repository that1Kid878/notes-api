use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub status_code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub enum AppResponse<T: Serialize> {
    OK(T),
    Created(T),
}

pub enum AppError {
    NotFound { message: String },
    BadRequest { message: String },
    InternalServerError { message: String },
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::OK(data) => {
                let response = ApiResponse {
                    success: true,
                    status_code: StatusCode::OK.as_u16(),
                    message: None,
                    data: Some(data),
                };
                (StatusCode::OK, Json(response)).into_response()
            }

            Self::Created(data) => {
                let response = ApiResponse {
                    success: true,
                    status_code: StatusCode::CREATED.as_u16(),
                    message: None,
                    data: Some(data),
                };
                (StatusCode::CREATED, Json(response)).into_response()
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound { message } => {
                let response: ApiResponse<()> = ApiResponse {
                    success: false,
                    status_code: StatusCode::NOT_FOUND.as_u16(),
                    message: Some(message.to_string()),
                    data: None,
                };
                (StatusCode::NOT_FOUND, Json(response)).into_response()
            }

            Self::BadRequest { message } => {
                let response: ApiResponse<()> = ApiResponse {
                    success: false,
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    message: Some(message.to_string()),
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }

            Self::InternalServerError { message } => {
                let response: ApiResponse<()> = ApiResponse {
                    success: false,
                    status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: Some(message.to_string()),
                    data: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }
        }
    }
}
