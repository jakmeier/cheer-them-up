//! Ghosts ignore all obstacles in their way and go directly to the base.

use constants::*;
use super::{Enemy, EnemyAttributes};
use super::super::jkm_shortest_path_map::JkmShortestPathMap;
use super::super::tower::Tower;

pub struct Ghost {
	attributes: EnemyAttributes,
}

impl Ghost {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> Ghost {
		let scale = level * level.sqrt();
		let hp = ENEMY_HEALTH[GHOST_EID] + ENEMY_HEALTH_SCALE[GHOST_EID] * scale;
		let ap = ENEMY_ATTACK[GHOST_EID] + ENEMY_ATTACK_SCALE[GHOST_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[GHOST_EID], attack_reload: 0.0,
			destination: (0.0,0.0), //ignored for this
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		Ghost {
			attributes: attr,
		}
	}
}

impl Enemy for Ghost {
	fn get_enemy_type_id(&self) -> usize {
		GHOST_EID
	}	
	fn get(&self) -> &EnemyAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
	
	// OVERWRITE 	OVERWRITE 	OVERWRITE	OVERWRITE
	
	fn score_value(&self) -> u32 { 5 }
	
	/// Returns true when the enemy reached the base, false otherwise
	#[allow(unused_variables)]
	fn update(&mut self, dt: f64, spm: &JkmShortestPathMap, towers: &mut Vec<Box<Tower>> ) -> bool {
		
		if self.get().destination_reached { self.get_mut().base_reached = true; return true; }
		
		let (dest_x, dest_y) = spm.get_destination_coordinates();
		self.walk_a_step(dest_x, dest_y, dt);
		false
	}
	#[allow(unused_variables)]
	fn refresh_destination(&mut self, spm: &JkmShortestPathMap) {}
}