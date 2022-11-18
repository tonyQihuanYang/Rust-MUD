use crate::game::models::{
    gear_profile::{GearId, GearProfile},
    monster_profile::MonsterProfile,
};
use rand::Rng;
use std::collections::HashMap;

pub struct FallOffControl {
    gears_dict: HashMap<GearId, GearProfile>,
}

impl FallOffControl {
    pub fn new(gears_dict: HashMap<GearId, GearProfile>) -> Self {
        Self { gears_dict }
    }

    pub fn get_fall_off(&self, monster: &MonsterProfile) -> Vec<GearProfile> {
        let fall_off_list = monster.fall_off.clone();
        let mut fall_offs = Vec::with_capacity(monster.max_fall_off.clone());
        let mut rng = rand::thread_rng();
        for (id, chance) in fall_off_list.iter() {
            let random_precentage = rng.gen_range(0..100);
            let dummy_chance = chance * 100 as f32;
            if random_precentage <= dummy_chance as i32 {
                if let Some(gear) = self.gears_dict.get(&id) {
                    fall_offs.push((*gear).clone());
                }
            }
        }
        fall_offs
    }
}
