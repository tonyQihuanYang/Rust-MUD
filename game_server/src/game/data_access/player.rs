use db::data_access::players;

pub fn incr_exp(id: i32, exp: i32) {
    players::incr_exp(id, exp).expect("_");
}
