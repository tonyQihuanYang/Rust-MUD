use crate::schema::accounts;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_on: SystemTime,
    pub last_login: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
