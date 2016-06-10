//! An enemy that starts with the highest speed level and has a faster attack ratio than other enemies.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct FastEnemy {
	attributes: EnemyAttributes,
}

impl FastEnemy {
	pub fn new(x: f64, y: f64, level: f64) -> FastEnemy {
		let scale = level * level.sqrt();
		let hp = ENEMY_HEALTH[FAST_EID] + ENEMY_HEALTH_SCALE[FAST_EID] * scale;
		let ap = ENEMY_ATTACK[FAST_EID] + ENEMY_ATTACK_SCALE[FAST_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: 0.0,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 2,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[FAST_EID], attack_reload: 0.0,
			destination: (x,y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		FastEnemy {
			attributes: attr,
		}
	}
}

impl Enemy for FastEnemy {
		fn get_enemy_type_id(&self) -> usize {
			FAST_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
}