use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    commands::{format_cmd, Cmds, PlayerCmds, SystemCmds},
    frame::{self, clear_monster_list, new_frame, Drawable},
    monsters::{self, monsters::Monsters},
    player::Player,
    profile::Profile,
    render::{self},
    section::Section,
    server::fake_server,
    LOG_X_END, LOG_X_START, LOG_Y_END, LOG_Y_START,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
    time::Instant,
};
use std::{sync::Arc, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("pew", "./sounds/pew.wav");
    audio.add("startup", "./sounds/startup.wav");
    audio.add("win", "./sounds/win.wav");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide).unwrap();

    // audio.play("startup");

    // Render loop
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game message loop
    let (game_log_tx, game_log_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut log = Section::new(LOG_X_START, LOG_X_END, LOG_Y_START, LOG_Y_END);
    {
        let mut log = log.clone();
        thread::spawn(move || {
            for msg in game_log_rx {
                log.add_message(msg);
            }
        });
    }

    // Game loop
    let running = Arc::new(Mutex::new(true));
    let player = Arc::new(Mutex::new(Player::new()));
    let mut instant = Instant::now();
    let mut monsters = Monsters::new(game_log_tx.clone());
    let (game_tx, game_rx): (Sender<Cmds>, Receiver<Cmds>) = mpsc::channel();
    {
        fake_server::listen(game_tx.clone());
        let mut log = log.clone();
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
                    },
                    _ => {}
                }
                log.add_message(format_cmd(msg).to_string());
            }
        });
    }

    let running = Arc::clone(&running);
    while *running.lock().unwrap() {
        // per-frame init
        let mut curr_frame = new_frame();
        log.draw_outline(&mut curr_frame);
        let delta = instant.elapsed();
        instant = Instant::now();

        //input
        // while event::poll(Duration::default())? {
        //     if let Event::Key(key_event) = event::read()? {
        //         match key_event.code {
        //             KeyCode::Up => player.move_up(),
        //             KeyCode::Down => player.move_down(),
        //             KeyCode::Left => player.move_left(),
        //             KeyCode::Right => player.move_right(),
        //             KeyCode::Char(' ') | KeyCode::Enter => {
        //                 if player.shoot() {
        //                     audio.play("pew");
        //                 }
        //             }
        //             KeyCode::Esc | KeyCode::Char('q') => {
        //                 // audio.play("lose");
        //                 break 'gameloop;
        //             }
        //             _ => {}
        //         }
        //     }
        // }

        // Updates

        let player_lock = Arc::clone(&player);
        let mut player = player_lock.lock().unwrap();
        player.update(delta);
        if monsters.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut monsters) {
            audio.play("explode");
            clear_monster_list(&mut curr_frame);
        }

        let drawables: Vec<&dyn Drawable> = vec![&(*player), &monsters, &log];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(16));
    }

    //Clean up
    drop(render_tx);
    // drop(game_tx);
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    println!("Exit...");
    Ok(())
}
