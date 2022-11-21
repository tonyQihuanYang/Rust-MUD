use super::auth_service;
use crate::commands::{Cmds, PlayerCmds, SystemCmds};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::{error::Error, sync::mpsc::Sender, thread};

pub fn listen(game_tx: Sender<Cmds>) {
    let game_tx = game_tx.clone();
    // Simulate Websocket connection or ...
    thread::spawn(move || user_actions(game_tx).unwrap());
}

pub fn user_actions(game_tx: Sender<Cmds>) -> Result<(), Box<dyn Error>> {
    if let Some(logged_user) = auth_service::authenticate() {
        let user_id = logged_user.id;
        game_tx
            .send(Cmds::Player(PlayerCmds::Join(user_id)))
            .unwrap();

        'userAction: loop {
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    let cmd = match key_event.code {
                        KeyCode::Up => Cmds::Player(PlayerCmds::MoveUp(user_id)),
                        KeyCode::Down => Cmds::Player(PlayerCmds::MoveDown(user_id)),
                        KeyCode::Left => Cmds::Player(PlayerCmds::MoveLeft(user_id)),
                        KeyCode::Right => Cmds::Player(PlayerCmds::MoveRight(user_id)),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            Cmds::Player(PlayerCmds::InputAttack(user_id))
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            break 'userAction;
                        }
                        _ => Cmds::System(SystemCmds::None),
                    };
                    game_tx.send(cmd).unwrap();
                }
            }
        }
    }

    game_tx.send(Cmds::System(SystemCmds::Quit)).unwrap();
    Ok(())
}
