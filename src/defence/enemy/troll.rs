//! The troll recovers health over time and regains speed.

use constants::*;
use super::{Enemy, EnemyAttributes};
use super::super::jkm_shortest_path_map::JkmShortestPathMap;
use super::super::tower::Tower;

const HEALTH_REGENERATION_BASE: f64 = 20.0;
const HEALTH_REGENERATION_SCALE: f64 = 10.0;
const SPEED_REGENERATION_COOLDOWN: f64 = 2.0;

pub struct Troll {
	attributes: EnemyAttributes,
	speed_reload_timer: f64, 
	health_regeneration: f64,
}

impl Troll {
    /// The level scales some attributes, it can be used to produce stronger units of the same type.
	pub fn new(x: f64, y: f64, level: f64) -> Troll {
		let scale = level * level / 2.0;
		let hp = ENEMY_HEALTH[TROLL_EID] + ENEMY_HEALTH_SCALE[TROLL_EID] * scale;
		let ap = ENEMY_ATTACK[TROLL_EID] + ENEMY_ATTACK_SCALE[TROLL_EID] * scale;
		let attr = EnemyAttributes {
			x: x-(STD_ENEMY_W/2.0), y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			speed: 1,
			health: hp, max_health: hp,
			attack: ap, attack_ratio: ENEMY_ATTACK_RATIO[TROLL_EID], attack_reload: 0.0,
			destination: (x-(STD_ENEMY_W/2.0),y),
			destination_reached: false,
			base_reached: false,
			berserker_mode: false, attack_target: None,
			animation_offset: (0.0, 0.0),
		};
		Troll {
			attributes: attr,
			speed_reload_timer: SPEED_REGENERATION_COOLDOWN, 
			health_regeneration: HEALTH_REGENERATION_BASE + scale * HEALTH_REGENERATION_SCALE,
		}
	}
}

impl Enemy for Troll {
		fn get_enemy_type_id(&self) -> usize {
			TROLL_EID
		}	
		fn get(&self) -> &EnemyAttributes { &self.attributes }
		fn get_mut(&mut self) -> &mut EnemyAttributes { &mut self.attributes }
		
		// OVERWRITE 	OVERWRITE 	OVERWRITE	OVERWRITE
		
		fn score_value(&self) -> u32 { 40 }
		
		/// Returns true when the enemy reached the base, false otherwise
		#[allow(unused_variables)]
		fn update(&mut self, dt: f64, spm: &JkmShortestPathMap, towers: &mut Vec<Box<Tower>> ) -> bool {
		
			// HEALTH AND SPEED REGENERATION
			self.get_mut().health += self.health_regeneration * dt;
			if self.get().speed < 1 {
				if self.speed_reload_timer <= 0.0 {
					self.speed_reload_timer = SPEED_REGENERATION_COOLDOWN;
					self.get_mut().speed = 1;
				}
				else {
					self.speed_reload_timer -= dt;
				}
			}
		
			//WALKING ( Open path to destination )
			if !self.get().base_reached && !self.get().berserker_mode {
				if self.get().destination_reached {
					self.refresh_destination(&spm);
				}
				
				let (dest_x, dest_y) = self.get().destination;
				self.walk_a_step(dest_x, dest_y, dt);
			}
			
			// BASE REACHED
			else if self.get().base_reached {
				return true;
			}
			
			// BERSERKER MODE
			else {
				if let Some(target) = self.get().attack_target {
					self.attack_tower( &mut towers[target], dt );
				}
				else {
					let (x,y) = spm.get_destination_coordinates();
					self.find_target(&towers, x, y);
				}	
			}
			false
		}
}