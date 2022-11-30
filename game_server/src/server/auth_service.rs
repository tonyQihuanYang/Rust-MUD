use db::data_access::{accounts::get_account_by_id_pw, players::get_player};
use db::models::{account::Account, player::Player};

pub fn authenticate() -> Option<Account> {
    get_account_by_id_pw("tony", "123")
}

pub fn get_player_info(user_id: i32) -> Option<Player> {
    get_player(user_id)
}
