use super::actions;
use super::models::{NewUser, User};
use super::Pool;
use actix_web::{web, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

type HttpResult = Result<HttpResponse, actix_web::Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_users(db: web::Data<Pool>) -> HttpResult {
    Ok(web::block(move || actions::get_all_users(&db))
        .await
        .map(|user| user.into())
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<i32>) -> HttpResult {
    Ok(web::block(move || actions::get_user_by_id(&db, *user_id))
        .await
        .map(|user| user.into())
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> HttpResult {
    Ok(web::block(move || actions::add_user(&db, *item))
        .await
        .map(|user| user.into())
        .map_err(|_| HttpResponse::InternalServerError())?)
}

/// handler for `DELETE /users/{id}`
pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    Ok(web::block(move || actions::delete_user(&db, *user_id))
        .await
        .map(|count| HttpResponse::Ok().body(format!("Deleted {} user.", count)))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
