//! The most basic attacker that can appear.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct Boss {
	attributes: EnemyAttributes,
}

impl Boss {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> Boss {
		let scale = level * level.sqrt();
		let hp = ENEMY_HEALTH[BOSS_EID] + ENEMY_HEALTH_SCALE[BOSS_EID] * scale;
		let ap = ENEMY_ATTACK[BOSS_EID] + ENEMY_ATTACK_SCALE[BOSS_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: 0.0,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[BOSS_EID], attack_reload: 0.0,
			destination: (x,y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		Boss {
			attributes: attr,
		}
	}
}

impl Enemy for Boss {
		fn get_enemy_type_id(&self) -> usize {
			BOSS_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		fn score_value(&self) -> u32 { 20 }
}