use crate::commands::{Cmds, PlayerCmds, SystemCmds};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::{error::Error, sync::mpsc::Sender, thread};

pub fn listen(game_tx: Sender<Cmds>) {
    let game_tx = game_tx.clone();
    thread::spawn(move || user_actions(game_tx).unwrap());
}

pub fn user_actions(game_tx: Sender<Cmds>) -> Result<(), Box<dyn Error>> {
    'userAction: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                let cmd = match key_event.code {
                    KeyCode::Up => Cmds::Player(PlayerCmds::MoveUp),
                    KeyCode::Down => Cmds::Player(PlayerCmds::MoveDown),
                    KeyCode::Left => Cmds::Player(PlayerCmds::MoveLeft),
                    KeyCode::Right => Cmds::Player(PlayerCmds::MoveRight),
                    KeyCode::Char(' ') | KeyCode::Enter => Cmds::Player(PlayerCmds::Attack),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'userAction;
                    }
                    _ => Cmds::System(SystemCmds::None),
                };
                game_tx.send(cmd).unwrap();
            }
        }
    }

    game_tx.send(Cmds::System(SystemCmds::Quit)).unwrap();
    Ok(())
}
