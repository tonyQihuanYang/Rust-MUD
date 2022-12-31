use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

// #[derive(Component, Replicate)]
// #[protocol_path = "crate::protocol::Protocol"]
// pub struct SpellTimer {
//     pub id: Property<i32>,
//     pub cur_tick: Property<i32>,
//     pub cur_cooldown: Property<i32>,
// }
// impl SpellTimer {
//     pub fn new(id: i32, cur_tick: i32, cur_cooldown: i32) -> Self {
//         SpellTimer::new_complete(id, cur_tick, cur_cooldown)
//     }
// }

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Spell {
    pub id: Property<i32>,
    pub speed: Property<i32>,    // x ticks
    pub cooldown: Property<i32>, // x ticks
    pub range: Property<i32>,
    pub cur_tick: Property<i32>,
    pub cur_cooldown: Property<i32>,
}

impl Spell {
    pub fn new(id: i32, speed: i32, cooldown: i32, range: i32) -> Self {
        Spell::new_complete(id, speed, cooldown, range, 0, 0)
    }
}
