use game_server::{
    commands::{Cmds, PlayerCmds, SystemCmds},
    game::game,
    server::fake_server,
    ui::ui,
};
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
};

use db::data_access::accounts;

fn main() {
    let (ui_cmds_tx, ui_cmds_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    let (server_cmds_tx, server_cmds_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    accounts::show_accounts();
    ui::start(ui_cmds_rx);
    fake_server::listen(server_cmds_tx.clone());
    game::start(ui_cmds_tx, server_cmds_rx);
    println!("Exit main...");
}
