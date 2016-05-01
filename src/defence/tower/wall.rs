//! A simple wall, doing nothing but blocking certain paths for enemies

use constants::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};

pub struct Wall {
	attributes: TowerAttributes,
}

impl Wall {
	pub fn new(x:f64, y:f64) -> Wall {
		Wall {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: TOWER_BASE_HEALTH_LIST[WALL_TID],
					health: TOWER_BASE_HEALTH_LIST[WALL_TID],
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[WALL_TID],
					cooldown: 0.0,
					attack_power: TOWER_BASE_ATTACK_LIST[WALL_TID],
				},
		}
	}
}

impl Tower for Wall {
	fn get_tower_type_id(&self) -> usize { WALL_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>) -> Option<Projectile> {
		None
	}
}