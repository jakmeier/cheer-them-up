//! Uses projectiles which slow enemies when they hit them. One projectile may slow a number of enemies, depending on the attack power of the tower.

use constants::*;
use defence::collision::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};
use definitions::GameState;

pub struct SlowTower {
	attributes: TowerAttributes,
	range: f64,
}

impl SlowTower {
	pub fn new(x:f64, y:f64, upgrades: &GameState) -> SlowTower {
		let hp = apply_defence_bonus(TOWER_BASE_HEALTH_LIST[SLOW_TID], upgrades.tower_upgrades[SLOW_TID][1]);
		SlowTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: hp,
					health: hp,
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[SLOW_TID],
					cooldown: 0.0, 
					attack_power: TOWER_BASE_ATTACK_LIST[SLOW_TID],
				},
				range: TOWER_BASE_RANGE_LIST[SLOW_TID],
		}
	}
}

fn apply_range_bonus(r: f64, b: u8) -> f64 {
	r + (b as f64) * 10.0
}
fn apply_attack_bonus(a: usize, b: u8) -> usize {
	a + (b as usize) * 3 
}
fn apply_defence_bonus(d: f64, b: u8) -> f64 {
	d + (b as f64) * 20.0
}

impl Tower for SlowTower {
	fn get_tower_type_id(&self) -> usize { SLOW_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>, upgrades: &GameState) -> Option<Projectile> {
		let (tower_w, tower_h) = self.get_tower_size();
		let start_x = self.attributes.x + tower_w /2.0;
		let start_y = self.attributes.y + tower_h /2.0;
		if let Some(closest_index) = find_closest_enemy(start_x, start_y, &enemies) {
			let (x,y) = enemies[closest_index].get_coordinates();
			let (w,h) = enemies[closest_index].get_size();
			let x = x + w/2.0;
			let y = y + h/2.0;
			let distance = ((start_x-x)*(start_x-x) + (start_y-y)*(start_y-y)).sqrt();
			let range = apply_range_bonus(self.range, upgrades.tower_upgrades[self.get_tower_type_id()][2]);
			let charges = apply_attack_bonus(1, upgrades.tower_upgrades[self.get_tower_type_id()][0]);
			if distance <= range {		
				Some( Projectile::new_slow( start_x, start_y, x, y, charges, range) )
			} else {None}
		} else {None}
	}
	
	fn calculate_defence_bonus(&self, d: f64, b: u8) -> f64 {
		apply_defence_bonus(d,b)
	}
}