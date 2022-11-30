use db::data_access;
pub fn main() {
    // run "cargo run --bin db_main" to execute this function
    println!("Testing db...");

    // data_access::accounts::create_account("tony", "123");
    data_access::accounts::show_accounts();
    if let Some(account) = data_access::accounts::get_account_by_id_pw("tony", "123") {
        println!("getting account {}", account.username);
        // data_access::accounts::delete_account_by_id(&account.id);
        // println!("deleteing account {}", account.id);

        players(account.id);
    }
}

fn players(user_id: i32) {
    println!("Start Players");
    // let new_player = data_access::players::create_player(user_id, "TOny");

    if let Some(player) = data_access::players::get_player(user_id) {
        data_access::players::update_player_exp(player.id, 100).expect("Not able to update exp");
    }
}
