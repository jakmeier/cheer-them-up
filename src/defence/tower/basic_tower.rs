//! The most basic tower available.

use constants::*;
use super::{Tower, TowerAttributes};

pub struct BasicTower {
	attributes: TowerAttributes,
}

impl BasicTower {
	pub fn new(x:f64, y:f64) -> BasicTower {
		BasicTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: 100.0,
					health: 100.0,
				}
		}
	}
}

impl Tower for BasicTower {
	fn get_tower_type_id(&self) -> usize {
		BASIC_TID
	}
	fn get(&self) -> &TowerAttributes {
		&self.attributes
	}
	fn get_mut(&mut self) -> &mut TowerAttributes {
		&mut self.attributes
	}
}