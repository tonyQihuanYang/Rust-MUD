use crate::{
    establish_connection,
    models::player::{NewPlayer, Player},
};
use diesel::prelude::*;

pub fn create_player(user_id: i32, name: &str) -> Player {
    let connection = &mut establish_connection();
    let new_player = NewPlayer {
        user_id,
        exp: 0,
        name,
    };

    use crate::schema::players;
    diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(connection)
        .expect("Error creating new account")
}

pub fn get_player(_user_id: i32) -> Option<Player> {
    let connection = &mut establish_connection();
    use crate::schema::players::dsl::*;

    let result = players
        .filter(user_id.eq(_user_id))
        .first::<Player>(connection)
        .optional()
        .expect("Unexpected error gett account by Id and password");

    result
}

pub fn incr_exp(_id: i32, _exp: i32) -> QueryResult<usize> {
    let connection = &mut establish_connection();
    use crate::schema::players::dsl::*;
    diesel::update(players)
        .filter(id.eq(_id))
        .set(exp.eq(exp + _exp))
        .execute(connection)
}
