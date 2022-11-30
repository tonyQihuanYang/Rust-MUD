use crate::schema::players;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Player {
    pub user_id: i32,
    pub id: i32,
    pub name: String,
    pub exp: i32,
}

#[derive(Insertable)]
#[diesel(table_name = players)]
pub struct NewPlayer<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub exp: i32,
}
