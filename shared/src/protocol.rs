use naia_shared::Protocolize;

mod auth;
mod color;
mod enemy;
mod entity_assignment;
mod key_command;
mod player;
mod position;
mod spell;

pub use auth::Auth;
pub use color::{Color, ColorValue};
pub use enemy::Enemy;
pub use entity_assignment::EntityAssignment;
pub use key_command::KeyCommand;
pub use player::Player;
pub use position::Position;
pub use spell::Spell;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    EntityAssignment(EntityAssignment),
    KeyCommand(KeyCommand),
    Position(Position),
    Color(Color),
    Enemy(Enemy),
    Player(Player),
    Spell(Spell),
}
