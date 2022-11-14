use crate::{
    monsters::monster::{Monster, MonsterProfile},
    position::Position,
    profile::Profile,
};

#[derive(Clone)]
pub enum Cmds {
    System(SystemCmds),
    Player(PlayerCmds),
    Monster(MonsterCmds),
}

#[derive(Clone)]
pub enum MonsterCmds {
    Move(u32, Position),
    Updated(Monster),
    Dead(MonsterProfile),
    Respwan,
    Damaged,
}

#[derive(Clone)]
pub enum PlayerCmds {
    InputAttack, // TODO: Create INPUT_PLAYERS_CMD
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Attack(Profile),
    Move(u32, Position),
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
            PlayerCmds::Attack(_) => Some(String::from("Player Attack")),
            // PlayerCmds::Move(id, position) => Some(format!(
            //     "Player {} Moved x:{} y:{}",
            //     id, position.x, position.y
            // )),
            _ => None,
        },
        Cmds::Monster(monster_cmd) => match monster_cmd {
            // MonsterCmds::Move(id, position) => Some(format!(
            //     "Monster {} Moved x:{} y:{}",
            //     id, position.x, position.y
            // )),
            MonsterCmds::Updated(m) => Some(format!("{} {}", m.name, m.health)),
            MonsterCmds::Dead(m) => Some(format!("{} is Dead", m.name)),
            MonsterCmds::Damaged => Some(String::from("Monster Damaged")),
            MonsterCmds::Respwan => Some(String::from("Monster Respwan")),
            _ => None,
        },
        _ => None,
    }
}
