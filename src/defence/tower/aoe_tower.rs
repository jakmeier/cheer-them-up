//! The Jack in the box towe
//! Lowers health from all nearby enenmies whenever it shoots, there are no projectiles produced by this tower

use constants::*;
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
					max_health: 100.0,
					health: 100.0,
				}
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
	fn get_tower_type_id(&self) -> usize {
		AOE_TID
	}
	fn get(&self) -> &TowerAttributes {
		&self.attributes
	}
	fn get_mut(&mut self) -> &mut TowerAttributes {
		&mut self.attributes
	}
}