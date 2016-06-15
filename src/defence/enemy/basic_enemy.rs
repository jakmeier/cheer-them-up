//! The most basic attacker that can appear.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct BasicEnemy {
	attributes: EnemyAttributes,
}

impl BasicEnemy {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> BasicEnemy {
		let scale = level * level.sqrt();
		let hp = ENEMY_HEALTH[BASIC_EID] + ENEMY_HEALTH_SCALE[BASIC_EID] * scale;
		let ap = ENEMY_ATTACK[BASIC_EID] + ENEMY_ATTACK_SCALE[BASIC_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[BASIC_EID], attack_reload: 0.0,
			destination: (x-(STD_ENEMY_W/2.0),y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		BasicEnemy {
			attributes: attr,
		}
	}
}

impl Enemy for BasicEnemy {
		fn get_enemy_type_id(&self) -> usize {
			BASIC_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
}