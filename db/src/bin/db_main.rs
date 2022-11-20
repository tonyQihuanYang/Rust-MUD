use db::data_access;
pub fn main() {
  // run "cargo run --bin db_main" to execute this function
  println!("Testing db...");
  data_access::accounts::show_accounts();
}
