use db::data_access::accounts::get_account_by_id_pw;
use db::models::Account;

pub fn authenticate() -> Option<Account> {
    get_account_by_id_pw("tony", "123")
}
