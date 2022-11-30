use super::auth_service;
use crate::commands::{Cmds, PlayerCmds, SystemCmds};
use crossterm::event::{self, Event, KeyCode};
use db::models::player::Player;
use std::time::Duration;
use std::{error::Error, sync::mpsc::Sender, thread};

pub fn listen(game_tx: Sender<Cmds>) {
    let game_tx = game_tx.clone();
    // Simulate Websocket connection or ...
    thread::spawn(move || login(game_tx).unwrap());
}

pub fn login(game_tx: Sender<Cmds>) -> Result<(), Box<dyn Error>> {
    match auth_service::authenticate() {
        Some(logged_user) => {
            let user_id = logged_user.id;
            match auth_service::get_player_info(user_id) {
                Some(player) => listen_player(player, game_tx),
                None => Ok(()),
            }
        }
        None => Ok(()),
    }
}

fn listen_player(player: Player, game_tx: Sender<Cmds>) -> Result<(), Box<dyn Error>> {
    game_tx
        .send(Cmds::Player(PlayerCmds::Join(player.clone())))
        .unwrap();
    'userAction: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                let id = player.id.clone();
                let cmd = match key_event.code {
                    KeyCode::Up => Cmds::Player(PlayerCmds::MoveUp(id)),
                    KeyCode::Down => Cmds::Player(PlayerCmds::MoveDown(id)),
                    KeyCode::Left => Cmds::Player(PlayerCmds::MoveLeft(id)),
                    KeyCode::Right => Cmds::Player(PlayerCmds::MoveRight(id)),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        Cmds::Player(PlayerCmds::InputAttack(id))
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

    game_tx.send(Cmds::System(SystemCmds::Quit)).unwrap();
    Ok(())
}
