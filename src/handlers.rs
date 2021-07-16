use super::actions;
use super::Pool;
use crate::auth::{create_jwt, create_jwt_role, Claims, Role};
use crate::models::User;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

type HttpResult = Result<HttpResponse, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserWithToken {
    pub user: User,
    pub token: String,
}

/// handler for `GET /users`
pub async fn get_users(db: web::Data<Pool>, claims: Claims) -> HttpResult {
    if claims.role >= Role::ReadAll {
        Ok(web::block(move || actions::get_all_users(&db))
            .await
            .map(|users| HttpResponse::Ok().json(users))
            .map_err(|_| HttpResponse::InternalServerError())?)
    } else {
        Err(ErrorUnauthorized("Cannot read other users"))
    }
}

/// handler for `GET /users/{id}`
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
    claims: Claims,
) -> HttpResult {
    if claims.uid == *user_id || claims.role >= Role::ReadAll {
        Ok(web::block(move || actions::get_user_by_id(&db, *user_id))
            .await
            .map(|user| user.into())
            .map_err(|_| HttpResponse::InternalServerError())?)
    } else {
        Err(ErrorUnauthorized("Cannot read other users"))
    }
}

/// handler for `POST /users`
pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> HttpResult {
    Ok(
        web::block(move || actions::add_user(&db, item.into_inner()))
            .await
            .map_err(|_| HttpResponse::InternalServerError())
            .map(|user| {
                HttpResponse::Ok().json(UserWithToken {
                    token: create_jwt(&user).expect("Could not create JWT"),
                    user,
                })
            })?,
    )
}

/// handler for `DELETE /users/{id}`
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
    claims: Claims,
) -> HttpResult {
    if claims.uid == *user_id || claims.role >= Role::WriteAll {
        Ok(web::block(move || actions::delete_user(&db, *user_id))
            .await
            .map(|count| HttpResponse::Ok().body(format!("Deleted {} user.", count)))
            .map_err(|_| HttpResponse::InternalServerError())?)
    } else {
        Err(ErrorUnauthorized("Cannot delete other user"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LoginData {
    name: String,
    password: String,
}

pub async fn admin_login(credentials: Json<LoginData>) -> HttpResult {
    if *credentials.password == *"hugo" && *credentials.name == *"hugo" {
        Ok(HttpResponse::Ok().body(create_jwt_role(
            &User {
                id: 0,
                first_name: "".to_string(),
                last_name: "".to_string(),
                email: "".to_string(),
                created_at: chrono::Local::now().naive_local(),
            },
            Role::Admin,
        )?))
    } else {
        Err(ErrorUnauthorized("Incorrect credentials"))
    }
}
