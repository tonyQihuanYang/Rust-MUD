use crate::position::Position;

#[derive(Clone)]
pub enum Cmds {
    System(SystemCmds),
    Player(PlayerCmds),
    Monster(MonsterCmds),
}

#[derive(Clone)]
pub enum MonsterCmds {
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
}

#[derive(Clone)]
pub enum SystemCmds {
    Quit,
    None,
}

pub fn format_cmd(cmd: Cmds) -> &'static str {
    match cmd {
        Cmds::System(SystemCmds::Quit) => "Quit Game",
        Cmds::Player(player_cmd) => match player_cmd {
            PlayerCmds::MoveUp => "Player Moved up",
            PlayerCmds::MoveLeft => "Player Moved Left",
            PlayerCmds::MoveDown => "Player Moved Down",
            PlayerCmds::MoveRight => "Player Moved Right",
            PlayerCmds::Attack => "Player Attack",
            // _ => "Non-matched Player CMD",
        },
        Cmds::Monster(monster_cmd) => match monster_cmd {
            MonsterCmds::Dead => "Monster Dead",
            MonsterCmds::Damaged => "Monster Damaged",
            MonsterCmds::Respwan => "Monster Respwan",
        },
        _ => "Non-matched Cmd",
    }
}
