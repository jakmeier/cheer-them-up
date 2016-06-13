//! A simple wall, doing nothing but blocking certain paths for enemies

use constants::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};
use definitions::GameState;

pub struct Wall {
	attributes: TowerAttributes,
}

fn apply_defence_bonus(d: f64, b: u8) -> f64 {
	d + (b as f64) * 30.0
}

impl Wall {
	pub fn new(x:f64, y:f64, upgrades: &GameState) -> Wall {
		let hp = apply_defence_bonus(TOWER_BASE_HEALTH_LIST[WALL_TID], upgrades.tower_upgrades[WALL_TID][1]);
		Wall {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: hp,
					health: hp,
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[WALL_TID],
					cooldown: 0.0,
					attack_power: TOWER_BASE_ATTACK_LIST[WALL_TID],
					show_delete_button: false,
				},
		}
	}
}

impl Tower for Wall {
	fn get_tower_type_id(&self) -> usize { WALL_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	#[allow(unused_variables)]
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>, upgrades: &GameState) -> Option<Projectile> {
		None
	}
	
	fn calculate_defence_bonus(&self, d: f64, b: u8) -> f64 {
		apply_defence_bonus(d,b)
	}
	
}