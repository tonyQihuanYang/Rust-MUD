use invaders::{
    commands::{format_cmd, Cmds, MonsterCmds, PlayerCmds, SendCmds, SystemCmds},
    monsters::monsters::Monsters,
    player::Player,
    server::fake_server,
    ui::{
        frame::{self, new_frame, Drawable, Frame},
        main::ui_loop,
    },
    LOG_X_END, LOG_X_START, LOG_Y_END, LOG_Y_START, MONSTERS_X_END, MONSTERS_X_START,
    MONSTERS_Y_END, MONSTERS_Y_START,
};
use std::{
    error::Error,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
    time::Instant,
};
use std::{sync::Arc, time::Duration};

fn main() {
    let render_tx = ui_loop();
    game_loop(render_tx).unwrap();
    println!("Exit main...");
}

fn game_loop(render_tx: Sender<Cmds>) -> Result<(), Box<dyn Error>> {
    // Game global-logs loop
    let (game_log_tx, game_log_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    {
        thread::spawn(move || {
            for cmd in game_log_rx {
                render_tx.send(cmd).unwrap();
            }
        });
    }

    //  == Game loop ==
    let running = Arc::new(Mutex::new(true));
    let player = Arc::new(Mutex::new(Player::new(game_log_tx.clone())));
    let mut monsters = Monsters::new(game_log_tx.clone());
    let mut instant = Instant::now();
    let (game_tx, game_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();

    // play inputs
    {
        fake_server::listen(game_tx.clone());
        let player_lock = Arc::clone(&player);
        let running = Arc::clone(&running);
        thread::spawn(move || {
            for msg in game_rx {
                let mut player = player_lock.lock().unwrap();
                match msg.clone() {
                    Cmds::System(SystemCmds::Quit) => {
                        let mut game_running = running.lock().unwrap();
                        *game_running = false;
                    }
                    Cmds::Player(player_cmd) => match player_cmd {
                        PlayerCmds::MoveUp => {
                            (*player).move_up();
                        }
                        PlayerCmds::MoveDown => {
                            (*player).move_down();
                        }
                        PlayerCmds::MoveLeft => {
                            (*player).move_left();
                        }
                        PlayerCmds::MoveRight => {
                            (*player).move_right();
                        }
                        PlayerCmds::Attack => {
                            (*player).shoot();
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        });
    }

    let running = Arc::clone(&running);
    // game logic
    while *running.lock().unwrap() {
        // per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();

        // Updates
        let player_lock = Arc::clone(&player);
        let mut player = player_lock.lock().unwrap();
        player.update(delta);
        if monsters.update(delta) {
            // audio.play("move");
        }

        if player.detect_hits(&mut monsters) {
            // audio.play("explode");
        }

        // Should remove this
        // let drawables: Vec<&dyn SendCmds> = vec![&(*player), &monsters];
        // let drawables: Vec<&dyn SendCmds> = vec![&(*player)];
        // for drawable in drawables {
        //     drawable.send();
        // }

        // thread::sleep(Duration::from_millis(16));
    }

    //Clean up
    // drop(render_tx);
    // drop(game_tx);
    // audio.wait();
    // stdout.execute(Show)?;
    // stdout.execute(LeaveAlternateScreen)?;
    println!("Exit...");
    Ok(())
}
