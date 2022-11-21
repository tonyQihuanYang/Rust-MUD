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
    }
}
