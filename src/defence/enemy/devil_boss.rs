//! Devils are extremly hard to slow down. Since this is a boss, it has increased stats.

use constants::*;
use super::{Enemy, EnemyAttributes};

pub struct DevilBoss {
	attributes: EnemyAttributes,
	slow_resistance: u32, slowing_hits_taken: u32,
}

impl DevilBoss {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> DevilBoss {
		let scale = level * level / 2.0;
		let hp = ENEMY_HEALTH[DEVIL_BOSS_EID] + ENEMY_HEALTH_SCALE[DEVIL_BOSS_EID] * scale;
		let ap = ENEMY_ATTACK[DEVIL_BOSS_EID] + ENEMY_ATTACK_SCALE[DEVIL_BOSS_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[DEVIL_BOSS_EID], attack_reload: 0.0,
			destination: (x-(STD_ENEMY_W/2.0),y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		DevilBoss {
			attributes: attr,
			slow_resistance: 15 + (scale * 3.0) as u32, slowing_hits_taken: 0,
		}
	}
}

impl Enemy for DevilBoss {
		fn get_enemy_type_id(&self) -> usize { DEVIL_BOSS_EID }	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
		// OVERWRITE 	OVERWRITE 	OVERWRITE	OVERWRITE
		
		fn score_value(&self) -> u32 { 30 }
		
		/// returns the amount of slwoing power that was used up
		fn slow_down(&mut self, slow_power: usize) -> usize {
			let mut p = 0;
			while slow_power - p > 0 && self.get().speed > 0 {
				if self.slow_resistance > self.slowing_hits_taken {
					self.slowing_hits_taken += 1;
					p += 1;
				}
				if self.slow_resistance <= self.slowing_hits_taken {
					self.get_mut().speed -= 1;
					self.slowing_hits_taken = 0;
				}
			}
			p
		}	
}