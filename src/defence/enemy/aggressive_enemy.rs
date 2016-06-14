//! The most basic attacker that can appear.

use std;

use constants::*;
use super::{Enemy, EnemyAttributes};
use super::super::tower::Tower;
use super::super::jkm_shortest_path_map::JkmShortestPathMap;

pub struct AggressiveEnemy {
	attributes: EnemyAttributes,
}

impl AggressiveEnemy {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> AggressiveEnemy {
		let scale = level * level.sqrt();
		let hp = ENEMY_HEALTH[AGGRO_EID] + ENEMY_HEALTH_SCALE[AGGRO_EID] * scale;
		let ap = ENEMY_ATTACK[AGGRO_EID] + ENEMY_ATTACK_SCALE[AGGRO_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: 0.0,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[AGGRO_EID], attack_reload: 0.0,
			destination: (x,y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		AggressiveEnemy {
			attributes: attr,
		}
	}
}

impl Enemy for AggressiveEnemy {
	fn get_enemy_type_id(&self) -> usize {
		AGGRO_EID
	}	
	fn get(&self) -> &EnemyAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
	
	// Overwrite
	/// Usually returns true when the enemy reached the base, false otherwise
	/// For this enemy, always return false, it does not even try to get to the base
	#[allow(unused_variables)] //spm
	fn update(&mut self, dt: f64, spm: &JkmShortestPathMap, towers: &mut Vec<Box<Tower>> ) -> bool {
		if let Some(target) = self.get().attack_target {
			self.attack_tower( &mut towers[target], dt );
		}
		else {
			let e_x = self.get().x + self.get().w / 2.0;
			let e_y = self.get().y + self.get().h / 2.0;
			let mut new_target = (std::f64::INFINITY, 0);
			for (i,t) in towers.iter().enumerate() {
				let (x,y) = t.get_coordinates();
				let (w,h) = t.get_tower_size();
				let squared_distance = (e_x - x).powi(2) + (e_y - y).powi(2);
				if squared_distance < new_target.0
					{ new_target = (squared_distance, i); }
			}
			if new_target.0 < std::f64::INFINITY {self.get_mut().attack_target = Some(new_target.1);}
		}	
		false
	}
}