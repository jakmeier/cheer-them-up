//! The Jack in the box towe
//! Lowers health from all nearby enenmies whenever it shoots, there are no projectiles produced by this tower

use constants::*;
use super::Tower;

pub struct AoeTower {
	x: f64, y: f64,
	max_health: f64, health: f64,
}

impl AoeTower {
	pub fn new(x:f64, y:f64) -> AoeTower {
		AoeTower {
			x: x, y:y,
			max_health: 100.0, health: 100.0,
		}
	}
	/*pub fn place_copy(source: &AoeTower, x: f64, y: f64) -> AoeTower{
		AoeTower {
			x: x, y: y,
			health: source.health,
		}
	}*/
}

impl Tower for AoeTower {
	fn get_coordinates(&self) -> (f64, f64) {
		(self.x, self.y)
	}
	fn set_coordinates(&mut self, x:f64, y:f64){
		self.x = x;
		self.y = y;
	}
	fn get_health(&self) -> f64 { self.health }
	fn set_health(&mut self, h: f64) { self.health = h; }	
	fn get_tower_type_id(&self) -> usize {
		AOE_TID
	}
}