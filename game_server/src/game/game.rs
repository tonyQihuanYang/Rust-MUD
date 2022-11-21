use super::{
    controllers::monsters_controller::MonstersControl,
    models::{
        gear_profile::{GearId, GearProfile},
        monster_profile::{MonsterId, MonsterProfile},
        monster_respawn_location::MonsterRespawnLocation,
        player::Player,
    },
};
use crate::{
    commands::{Cmds, PlayerCmds, SystemCmds},
    game::controllers::players_controller::PlayersControl,
};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread,
    time::Instant,
};

use crate::game::startup::startup;
pub fn start(render_tx: Sender<Cmds>, server_cmds_rx: Receiver<Cmds>) {
    let (gears_dict, monsters_dict, monster_respawn_location) = startup::load().unwrap();
    let (game_log_tx, game_log_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    {
        thread::spawn(move || {
            for cmd in game_log_rx {
                render_tx.send(cmd).unwrap();
            }
        });
    }
    game_loop(
        game_log_tx,
        server_cmds_rx,
        monsters_dict,
        monster_respawn_location,
        gears_dict,
    );
}

fn game_loop(
    game_log_tx: Sender<Cmds>,
    server_cmds_rx: Receiver<Cmds>,
    monsters_lookup: HashMap<MonsterId, MonsterProfile>,
    monsters_respawn_location: Vec<MonsterRespawnLocation>,
    gears_dict: HashMap<GearId, GearProfile>,
) {
    // == Main Game Logic ==
    let running = Arc::new(Mutex::new(true));
    // let player = Arc::new(Mutex::new(Player::new(game_log_tx.clone())));
    let players_controller = Arc::new(PlayersControl::new(game_log_tx.clone()));

    let mut monsters_controller = MonstersControl::new(
        game_log_tx.clone(),
        monsters_lookup,
        monsters_respawn_location,
        gears_dict,
    );
    let mut instant = Instant::now();

    // Player Inputs
    // {
    //     // fake_server::listen(player_input_tx.clone());
    //     // let player_lock = Arc::clone(&player);
    //     let running = Arc::clone(&running);
    //     let players_controller = Arc::clone(&players_controller);
    //     // thread::spawn(move || {
    //     for msg in server_cmds_rx {
    //         match msg.clone() {
    //             Cmds::System(SystemCmds::Quit) => {
    //                 let mut game_running = running.lock().unwrap();
    //                 *game_running = false;
    //             }
    //             Cmds::Player(player_cmd) => {
    //                 (*players_controller).execute_cmds(player_cmd);
    //             }
    //             _ => (),
    //         }
    //     }
    //     // });
    // }

    let running = Arc::clone(&running);
    // game logic
    while *running.lock().unwrap() {
        {
            // fake_server::listen(player_input_tx.clone());
            // let player_lock = Arc::clone(&player);
            let running = Arc::clone(&running);
            let players_controller = Arc::clone(&players_controller);
            // thread::spawn(move || {
            if let Ok(msg) = server_cmds_rx.recv() {
                match msg.clone() {
                    Cmds::System(SystemCmds::Quit) => {
                        let mut game_running = running.lock().unwrap();
                        *game_running = false;
                    }
                    Cmds::Player(player_cmd) => {
                        (*players_controller).execute_cmds(player_cmd);
                    }
                    _ => (),
                }
            }
            // });
        }
        //
        //
        //
        // per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();

        // Updates
        // let player_lock = Arc::clone(&player);
        // let mut player = player_lock.lock().unwrap();
        // player.update(delta);
        monsters_controller.update(delta);
        // player.detect_hits(&mut monsters_controller);
        // thread::sleep(Duration::from_millis(16));
    }
    println!("Exit...");
}
