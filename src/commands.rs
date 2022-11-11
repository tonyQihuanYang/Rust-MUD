#[derive(Clone)]
pub enum Cmds {
    System(SystemCmds),
    Player(PlayerCmds),
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
        _ => "Non-matched Cmd",
    }
}
