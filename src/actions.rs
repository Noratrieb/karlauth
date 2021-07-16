use super::models::{NewUser, User};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::handlers::InputUser;
use crate::Pool;
use diesel::{delete, insert_into};

type DbResult<T> = Result<T, diesel::result::Error>;

pub fn get_all_users(db: &Pool) -> DbResult<User> {
    use super::schema::users::dsl::*;
    let conn = db.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn get_user_by_id(db: &Pool, id: i32) -> DbResult<User> {
    use super::schema::users::dsl::*;
    let conn = db.get().unwrap();
    users.find(id).get_result::<User>(&conn)
}

pub fn add_user(db: &Pool, user: InputUser) -> DbResult<User> {
    use super::schema::users::dsl::*;
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users).values(&new_user).get_result(&conn)
}

pub fn delete_user(db: &Pool, id: i32) -> DbResult<usize> {
    use super::schema::users::dsl::*;
    let conn = db.get().unwrap();
    delete(users.find(id)).execute(&conn)
}
