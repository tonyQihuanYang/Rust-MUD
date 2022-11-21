use crate::{
    establish_connection,
    models::{Account, NewAccount},
};
use diesel::prelude::*;

pub fn show_accounts() {
    use crate::schema::accounts::dsl::*;

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

pub fn get_account_by_id_pw(_username: &str, _password: &str) -> Option<Account> {
    use crate::schema::accounts::dsl::*;

    let connection = &mut establish_connection();
    let result = accounts
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .first::<Account>(connection)
        .optional()
        .expect("Unexpected error gett account by Id and password");

    result
}

pub fn create_account(username: &str, password: &str) -> Account {
    let connection = &mut establish_connection();
    let new_account = NewAccount { username, password };

    use crate::schema::accounts;
    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(connection)
        .expect("Error creating new account")
}

pub fn delete_account_by_id(_id: &i32) -> usize {
    use crate::schema::accounts::dsl::*;
    let connection = &mut establish_connection();
    diesel::delete(accounts.filter(id.eq(_id)))
        .execute(connection)
        .expect("Error deleting account")
}
