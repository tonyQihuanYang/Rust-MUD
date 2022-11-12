use crate::commands::{format_cmd, Cmds, MonsterCmds, PlayerCmds};

use super::{
    frame::{new_frame, Drawable, FrameMsg},
    render::{self},
    section::Section,
    LOG_X_END, LOG_X_START, LOG_Y_END, LOG_Y_START, MAP_X_END, MAP_X_START, MAP_Y_END, MAP_Y_START,
    MONSTERS_X_END, MONSTERS_X_START, MONSTERS_Y_END, MONSTERS_Y_START,
};
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread, time,
    time::{Duration, Instant},
};

pub fn ui_loop() -> Sender<Cmds> {
    // let audio = init_audio();
    let game_cmds_tx = init_terminal();
    game_cmds_tx
}

fn init_audio() -> Audio {
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");
    audio.add("win", "./sounds/win.wav");
    audio
}

fn init_terminal() -> Sender<Cmds> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    // Render loop
    let (game_cmds_tx, game_cmds_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        let map_section = Arc::new(RwLock::new(Section::new(
            MAP_X_START,
            MAP_X_END,
            MAP_Y_START,
            MAP_Y_END,
        )));
        let log_section = Arc::new(RwLock::new(Section::new(
            LOG_X_START,
            LOG_X_END,
            LOG_Y_START,
            LOG_Y_END,
        )));
        let monsters_section = Section::new(
            MONSTERS_X_START,
            MONSTERS_X_END,
            MONSTERS_Y_START,
            MONSTERS_Y_END,
        );

        {
            let log_lock = Arc::clone(&log_section);
            let log = log_lock.read().unwrap();
            (*log).draw_outline(&mut last_frame);
        }
        monsters_section.draw_outline(&mut last_frame);
        render::render(&mut stdout, &last_frame, &last_frame, true);

        let log_lock = Arc::clone(&log_section);
        let map_lock = Arc::clone(&map_section);
        thread::spawn(move || {
            let mut last_frame = new_frame();
            loop {
                let mut curr_frame = new_frame();
                {
                    let log_section = log_lock.read().unwrap();
                    (*log_section).draw_outline(&mut curr_frame);
                    monsters_section.draw_outline(&mut curr_frame);
                    let map = map_lock.read().unwrap();
                    let drawables: Vec<&dyn Drawable> =
                        vec![&(*log_section), &monsters_section, &(*map)];

                    for drawable in drawables {
                        drawable.draw(&mut curr_frame);
                    }
                }
                render::render(&mut stdout, &last_frame, &curr_frame, false);
                last_frame = curr_frame;
                thread::sleep(Duration::from_millis(16));
            }
        });

        loop {
            let log_lock = Arc::clone(&log_section);
            let map_lock = Arc::clone(&map_section);
            match game_cmds_rx.recv() {
                Ok(cmd) => {
                    {
                        if let Some(formated_msg) = format_cmd(&cmd) {
                            let mut log_section = log_lock.write().unwrap();
                            (*log_section).add_message(formated_msg, None);
                        }
                    }
                    match cmd {
                        Cmds::Player(player_cmd) => match player_cmd {
                            PlayerCmds::Move(position) => {
                                let mut map = map_lock.write().unwrap();
                                (*map).add_message(String::from("A"), Some(position));
                            }
                            _ => (),
                        },
                        Cmds::Monster(monster_cmd) => match monster_cmd {
                            MonsterCmds::Move(position) => {
                                let mut map = map_lock.write().unwrap();
                                (*map).add_message(String::from("M"), Some(position));
                            }
                            _ => {}
                        },

                        _ => (),
                    };
                }
                Err(_) => break,
            };
        }
    });
    // ui_thread.join().unwrap();

    // stdout.execute(Show).unwrap();
    // stdout.execute(LeaveAlternateScreen).unwrap();
    println!("Exit UI...");
    game_cmds_tx
    // Ok(())
}
