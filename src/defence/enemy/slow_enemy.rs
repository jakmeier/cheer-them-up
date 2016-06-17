//! An enemy that enters with the slowest speed level. It has a high amount of life.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct SlowEnemy {
	attributes: EnemyAttributes,
}

impl SlowEnemy {
	pub fn new(x: f64, y: f64, level: f64) -> SlowEnemy {
		let scale = level * level / 2.0;
		let hp = ENEMY_HEALTH[SLOW_EID] + ENEMY_HEALTH_SCALE[SLOW_EID] * scale;
		let ap = ENEMY_ATTACK[SLOW_EID] + ENEMY_ATTACK_SCALE[SLOW_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 0,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[SLOW_EID], attack_reload: 0.0,
			destination: (x-(STD_ENEMY_W/2.0),y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		SlowEnemy {
			attributes: attr,
		}
	}
}

impl Enemy for SlowEnemy {
		fn get_enemy_type_id(&self) -> usize {
			SLOW_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
}