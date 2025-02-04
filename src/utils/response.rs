use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct ApiResponse<T: Serialize> {
    message: String,
    status_code: StatusCode,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(message: String, status_code: StatusCode, data: Option<T>) -> Self {
        Self {
            message,
            status_code,
            data,
        }
    }

    pub fn success(data: T) -> Self {
        Self {
            message: "Operation successful".to_string(),
            status_code: StatusCode::OK,
            data: Some(data),
        }
    }

    pub fn user_not_found() -> Self {
        Self {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            data: None,
        }
    }

    pub fn users_not_found() -> Self {
        Self {
            message: "Users not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            data: None,
        }
    }

    pub fn feedbacks_not_found() -> Self {
        Self {
            message: "Feedbacks not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            data: None,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            message: "Resource created successfully".to_string(),
            status_code: StatusCode::CREATED,
            data: Some(data),
        }
    }

    pub fn updated(data: T) -> Self {
        Self {
            message: "Resource updated successfully".to_string(),
            status_code: StatusCode::OK,
            data: Some(data),
        }
    }

    pub fn updated_password() -> Self {
        Self {
            message: "Resource updated successfully".to_string(),
            status_code: StatusCode::OK,
            data: None,
        }
    }

    pub fn updated_enabled() -> Self {
        Self {
            message: "Resource updated successfully".to_string(),
            status_code: StatusCode::OK,
            data: None,
        }
    }

    pub fn deleted() -> Self {
        Self {
            message: "Resource deleted successfully".to_string(),
            status_code: StatusCode::OK,
            data: None,
        }
    }

    pub fn resent_code_success() -> Self {
        Self {
            message: "Verification code resent successfully".to_string(),
            status_code: StatusCode::OK,
            data: None,
        }
    }

    pub fn application_not_found() -> Self {
        Self {
            message: "Application not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            data: None,
        }
    }

    pub fn code_not_found() -> Self {
        Self {
            message: "code not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            data: None,
        }
    }

    pub fn into_response(self) -> HttpResponse {
        HttpResponse::build(self.status_code)
            .json(json!({
                "message": self.message,
                "status": self.status_code.as_u16(),
                "data": self.data
            }))
    }

    
}