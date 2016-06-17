//! An enemy that starts with the highest speed level and has a faster attack ratio than other enemies.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct Alien {
	attributes: EnemyAttributes,
}

impl Alien {
	pub fn new(x: f64, y: f64, level: f64) -> Alien {
		let scale = level * level / 2.0;
		let hp = ENEMY_HEALTH[ALIEN_EID] + ENEMY_HEALTH_SCALE[ALIEN_EID] * scale;
		let ap = ENEMY_ATTACK[ALIEN_EID] + ENEMY_ATTACK_SCALE[ALIEN_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 2,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[ALIEN_EID], attack_reload: 0.0,
			destination: (x-(STD_ENEMY_W/2.0),y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		Alien {
			attributes: attr,
		}
	}
}

impl Enemy for Alien {
		fn get_enemy_type_id(&self) -> usize {
			ALIEN_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
}