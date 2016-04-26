//! The most basic attacker that can appear.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct BasicEnemy {
	attributes: EnemyAttributes,
}

impl BasicEnemy {
	pub fn new(x: f64, y: f64) -> BasicEnemy {
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			life: 100.0,
			speed: 2,
			attack: STD_ENEMY_ATTACK, attack_ratio: STD_ENEMY_ATTACK_RATIO, attack_reload: 0.0,
			destination: (x,y),
			destination_reached: true,
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