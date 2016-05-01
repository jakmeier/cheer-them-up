//! The most basic tower available.

use constants::*;
use defence::collision::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};

pub struct BasicTower {
	attributes: TowerAttributes,
	range: f64,
}

impl BasicTower {
	pub fn new(x:f64, y:f64) -> BasicTower {
		BasicTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: TOWER_BASE_HEALTH_LIST[BASIC_TID],
					health: TOWER_BASE_HEALTH_LIST[BASIC_TID],
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[BASIC_TID],
					cooldown: 0.0, 
					attack_power: TOWER_BASE_ATTACK_LIST[BASIC_TID],
				},
				range: TOWER_BASE_RANGE_LIST[BASIC_TID],
		}
	}
}

impl Tower for BasicTower {
	fn get_tower_type_id(&self) -> usize { BASIC_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>) -> Option<Projectile> {
		let (tower_w, tower_h) = self.get_tower_size();
		let start_x = self.attributes.x + tower_w /2.0;
		let start_y = self.attributes.y + tower_h /2.0;
		if let Some(closest_index) = find_closest_enemy(start_x, start_y, &enemies) {
			let (x,y) = enemies[closest_index].get_coordinates();
			let (w,h) = enemies[closest_index].get_size();
			let x = x + w/2.0;
			let y = y + h/2.0;
			let distance = ((start_x-x)*(start_x-x) + (start_y-y)*(start_y-y)).sqrt();
			if distance <= self.range {		
				Some( Projectile::new( start_x, start_y, x, y, self.attributes.attack_power, self.range) )
			} else {None}
		} else {None}
	}
}