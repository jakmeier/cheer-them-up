//! The Jack in the box towe
//! Lowers health from all nearby enenmies whenever it shoots, there are no projectiles produced by this tower

use constants::*;
use defence::collision::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};

pub struct AoeTower {
	attributes: TowerAttributes,
}

impl AoeTower {
	pub fn new(x:f64, y:f64) -> AoeTower {
		AoeTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: TOWER_BASE_HEALTH_LIST[AOE_TID],
					health: TOWER_BASE_HEALTH_LIST[AOE_TID],
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[AOE_TID],
					cooldown: 0.0,
					attack_power: TOWER_BASE_ATTACK_LIST[AOE_TID],
				}
		}
	}
}

impl Tower for AoeTower {
	fn get_tower_type_id(&self) -> usize { AOE_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>) -> Option<Projectile> {
		None
	}
}