use crate::position::{self, Position};

#[derive(Clone)]
pub enum Cmds {
    System(SystemCmds),
    Player(PlayerCmds),
    Monster(MonsterCmds),
}

#[derive(Clone)]
pub enum MonsterCmds {
    Move(Position),
    Dead,
    Respwan,
    Damaged,
}

#[derive(Clone)]
pub enum PlayerCmds {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Attack,
    Move(Position),
}

#[derive(Clone)]
pub enum SystemCmds {
    Quit,
    None,
}

pub fn format_cmd(cmd: &Cmds) -> Option<String> {
    match cmd {
        Cmds::System(SystemCmds::Quit) => Some(String::from("Quit Game")),
        Cmds::Player(player_cmd) => match player_cmd {
            // PlayerCmds::MoveUp => "Player Moved up",
            // PlayerCmds::MoveLeft => "Player Moved Left",
            // PlayerCmds::MoveDown => "Player Moved Down",
            // PlayerCmds::MoveRight => "Player Moved Right",
            // PlayerCmds::Attack => "Player Attack",
            // PlayerCmds::Move(position) => {
            //     Some(format!("Player Moved x:{} y:{}", position.x, position.y))
            // }
            _ => None,
        },
        Cmds::Monster(monster_cmd) => match monster_cmd {
            MonsterCmds::Move(position) => {
                Some(format!("Monster Moved x:{} y:{}", position.x, position.y))
            }
            MonsterCmds::Dead => Some(String::from("Monster Dead")),
            MonsterCmds::Damaged => Some(String::from("Monster Damaged")),
            MonsterCmds::Respwan => Some(String::from("Monster Respwan")),
            _ => None,
        },
        _ => None,
    }
}

pub trait SendCmds {
    fn send(&self);
}
