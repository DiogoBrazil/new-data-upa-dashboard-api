use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::{
    application::user_service::UserService,
    domain::models::user::{CreateUserDto, UpdatePasswordDto, UpdateUserDto},
    AppError,
};

pub async fn get_users(service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    service.get_users().await
}

pub async fn create_user(
    service: web::Data<UserService>,
    user: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    service.create_user(user.into_inner()).await
}

pub async fn get_user_by_id(
    service: web::Data<UserService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    service.get_user_by_id(id.into_inner()).await
}

pub async fn update_user(
    service: web::Data<UserService>,
    id: web::Path<Uuid>,
    user: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, AppError> {
    service.update_user(id.into_inner(), user.into_inner()).await
}

pub async fn delete_user(
    service: web::Data<UserService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    service.delete_user(id.into_inner()).await
}

pub async fn update_password(
    service: web::Data<UserService>,
    id: web::Path<Uuid>,
    passwords: web::Json<UpdatePasswordDto>,
) -> Result<HttpResponse, AppError> {
    service.update_password(id.into_inner(), passwords.into_inner()).await
}