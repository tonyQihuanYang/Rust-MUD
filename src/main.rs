use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, clear_monster_list, new_frame, Drawable},
    monsters::{self, monsters::Monsters},
    player::Player,
    profile::Profile,
    render::{self},
    section::Section,
    LOG_X_END, LOG_X_START, LOG_Y_END, LOG_Y_START,
};
use rusty_audio::Audio;
use std::time::Duration;
use std::{
    error::Error,
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Instant,
};

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
    let (game_tx, game_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut log = Section::new(LOG_X_START, LOG_X_END, LOG_Y_START, LOG_Y_END);
    let mut _log = log.clone();
    let game_handle = thread::spawn(move || {
        for msg in game_rx {
            _log.add_message(msg);
        }
    });

    // render_handle.join().unwrap();
    // game_handle.join().unwrap();

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut monsters = Monsters::new(game_tx.clone());
    'gameloop: loop {
        // per-frame init
        let mut curr_frame = new_frame();
        log.draw_outline(&mut curr_frame);
        let delta = instant.elapsed();
        instant = Instant::now();

        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => player.move_up(),
                    KeyCode::Down => player.move_down(),
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if monsters.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut monsters) {
            audio.play("explode");
            clear_monster_list(&mut curr_frame);
        }

        let drawables: Vec<&dyn Drawable> = vec![&player, &monsters, &log];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(16));

        // win or lose ?
        // if monsters.all_killed() {
        //     audio.play("win");
        //     break 'gameloop;
        // }
        // if monsters.reached_bottom() {
        //     audio.play("lose");
        //     break 'gameloop;
        // }
    }

    //Clean up
    drop(render_tx);
    drop(game_tx);
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    println!("Exit...");
    Ok(())
}
