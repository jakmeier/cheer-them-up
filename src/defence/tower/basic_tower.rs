//! The most basic tower available.

use constants::*;
use super::Tower;

pub struct BasicTower {
	x: f64, y: f64,
	max_health: f64, health: f64,
}

impl BasicTower {
	pub fn new(x:f64, y:f64) -> BasicTower {
		BasicTower {
			x: x, y: y,
			max_health: 100.0,
			health: 100.0,
		}
	}
	/*pub fn place_copy(source: &BasicTower, x: f64, y: f64) -> BasicTower{
		BasicTower {
			x: x, y: y,
			health: source.health,
		}
	}*/
}

impl Tower for BasicTower {
	fn get_coordinates(&self) -> (f64, f64) {
		(self.x, self.y)
	}
	fn set_coordinates(&mut self, x:f64, y:f64){
		self.x = x;
		self.y = y;
	}
		
	fn get_tower_type_id(&self) -> usize {
		BASIC_TID
	}
}