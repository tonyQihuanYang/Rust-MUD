use crate::commands::{format_cmd, Cmds, MonsterCmds, PlayerCmds, SystemCmds};
use crate::position::{Bound, Position};
use crate::ui::sections::{
    messages_box::MessagesBox,
    mini_map::{MiniMap, MiniMapItem},
    ui_box::UiBoxMsg,
};

use super::{
    frame::{new_frame, Drawable, FrameMsg},
    render::{self},
    skills::{skill::Skill, skills_control::SkillsControl},
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
    io,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

pub fn start(ui_cmds_rx: Receiver<Cmds>) {
    // let audio = init_audio();
    init_terminal(ui_cmds_rx);
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

fn init_terminal(ui_cmds_rx: Receiver<Cmds>) {
    // Render loop
    thread::spawn(move || {
        let is_running = Arc::new(RwLock::new(true));
        let skills_control: Arc<RwLock<SkillsControl>> =
            Arc::new(RwLock::new(SkillsControl::new()));
        let map_section = Arc::new(RwLock::new(MiniMap::new()));
        let log_section = Arc::new(RwLock::new(MessagesBox::new(
            LOG_X_START,
            LOG_X_END,
            LOG_Y_START,
            LOG_Y_END,
        )));
        let monsters_section = MessagesBox::new(
            MONSTERS_X_START,
            MONSTERS_X_END,
            MONSTERS_Y_START,
            MONSTERS_Y_END,
        );

        let attacks_control_lock = Arc::clone(&skills_control);
        let running_lock = Arc::clone(&is_running);
        let log_lock = Arc::clone(&log_section);
        let map_lock = Arc::clone(&map_section);
        thread::spawn(move || {
            let mut stdout = io::stdout();
            terminal::enable_raw_mode().unwrap();
            stdout.execute(EnterAlternateScreen).unwrap();
            stdout.execute(Hide).unwrap();
            render::render(&mut stdout, &new_frame(), &new_frame(), true);
            let mut last_frame = new_frame();
            let is_running = running_lock.read().unwrap();

            while *is_running {
                let mut curr_frame = new_frame();
                {
                    let log_section = log_lock.read().unwrap();
                    let map = map_lock.read().unwrap();
                    {
                        let mut skills_control = attacks_control_lock.write().unwrap();
                        (*skills_control).refresh()
                    }
                    let skills_control = attacks_control_lock.read().unwrap();
                    let drawables: Vec<&dyn Drawable> = vec![
                        &(*log_section),
                        &monsters_section,
                        &(*map),
                        &*skills_control,
                    ];

                    for drawable in drawables {
                        drawable.draw(&mut curr_frame);
                    }
                }
                render::render(&mut stdout, &last_frame, &curr_frame, false);
                last_frame = curr_frame;
                thread::sleep(Duration::from_millis(16));
            }
            stdout.execute(Show).unwrap();
            stdout.execute(LeaveAlternateScreen).unwrap();
        });

        loop {
            let log_lock = Arc::clone(&log_section);
            let map_lock = Arc::clone(&map_section);
            let attacks_control_lock = Arc::clone(&skills_control);

            match ui_cmds_rx.recv() {
                Ok(cmd) => {
                    {
                        if let Some(formated_msg) = format_cmd(&cmd) {
                            let log_section = log_lock.write().unwrap();
                            (*log_section).show(&UiBoxMsg {
                                value: FrameMsg::String(formated_msg),
                                position: None,
                            });
                        }
                    }
                    match cmd {
                        Cmds::System(system_cmd) => match system_cmd {
                            SystemCmds::Quit => {
                                break;
                            }
                            _ => (),
                        },
                        Cmds::Player(player_cmd) => match player_cmd {
                            PlayerCmds::Move(id, position) => {
                                let map = map_lock.write().unwrap();
                                (*map).show(&MiniMapItem {
                                    id,
                                    icon: String::from("A"),
                                    position,
                                });
                            }
                            PlayerCmds::Attack(player) => {
                                let attack = Skill::new(
                                    Position::new(
                                        player.position.x,
                                        player.position.y,
                                        Some(Bound::new(
                                            MAP_X_START,
                                            MAP_X_END,
                                            MAP_Y_START,
                                            MAP_Y_END,
                                        )),
                                    ),
                                    3,
                                    3,
                                    None,
                                );
                                attack.render();
                                let mut attacks_control = attacks_control_lock.write().unwrap();
                                attacks_control.push(attack);
                            }
                            _ => (),
                        },
                        Cmds::Monster(monster_cmd) => match monster_cmd {
                            MonsterCmds::Move(id, position) => {
                                let map = map_lock.write().unwrap();
                                (*map).show(&MiniMapItem {
                                    id,
                                    icon: String::from("M"),
                                    position,
                                });
                            }
                            MonsterCmds::Dead(monster) => {
                                let map = map_lock.write().unwrap();
                                (*map).remove(&monster.id);
                            }
                            _ => {}
                        },
                        _ => (),
                    };
                }
                Err(_) => break,
            };
        }

        let running_lock = Arc::clone(&is_running);
        let mut is_running = running_lock.write().unwrap();
        *is_running = false;
    });
    println!("Exit UI...");
}
