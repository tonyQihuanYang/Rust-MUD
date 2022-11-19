use self::models::*;
use db::*;
use diesel::prelude::*;

pub fn main() {
    use self::schema::accounts::dsl::*;

    let connection = &mut establish_connection();
    let results = accounts
        .load::<Account>(connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.username);
        println!("-----------\n");
        println!("{}", account.id);
    }
}
