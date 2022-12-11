use naia_shared::Protocolize;

mod auth;
mod color;
mod current_user;
mod enemy;
mod entity_assignment;
mod key_command;
mod player;
mod position;
mod spell;
mod spell_key_command;

pub use auth::Auth;
pub use color::{Color, ColorValue};
pub use current_user::CurrentUser;
pub use enemy::Enemy;
pub use entity_assignment::EntityAssignment;
pub use key_command::KeyCommand;
pub use player::Player;
pub use position::Position;
pub use spell::Spell;
pub use spell_key_command::SpellKeyCommand;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    EntityAssignment(EntityAssignment),
    KeyCommand(KeyCommand),
    SpellKeyCommand(SpellKeyCommand),
    Position(Position),
    Color(Color),
    Enemy(Enemy),
    Player(Player),
    Spell(Spell),
    CurrentUser(CurrentUser),
}
