//! The Jack in the box towe
//! Lowers health from all nearby enenmies whenever it shoots, there are no projectiles produced by this tower

use constants::*;
use defence::collision::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub struct AoeTower {
	attributes: TowerAttributes,
	range: f64,
}

impl AoeTower {
	pub fn new(x:f64, y:f64) -> AoeTower {
		AoeTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: TOWER_BASE_HEALTH_LIST[AOE_TID],
					health: TOWER_BASE_HEALTH_LIST[AOE_TID],
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[AOE_TID],
					cooldown: 0.0,
					attack_power: TOWER_BASE_ATTACK_LIST[AOE_TID],
				},
				range: TOWER_BASE_RANGE_LIST[AOE_TID],
		}
	}
}

impl Tower for AoeTower {
	fn get_tower_type_id(&self) -> usize { AOE_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>) -> Option<Projectile> {
		let (w,h) = self.get_tower_size();
		let center_x = self.attributes.x + w / 2.0;
		let center_y = self.attributes.y + h / 2.0;
		let to_attack = find_enemies_in_circle(enemies, center_x, center_y, self.range);
		for &i in to_attack.iter() {
			enemies[i].attack_enemy(self.attributes.attack_power);
		}
		None
	}
	
	// OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	
	
	fn attack_animation(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64) {
		let state = self.attributes.cooldown / self.attributes.reload_time;
		match state {
			0.9 ... 1.0 => {
				let (w,h) = self.get_tower_size();
				let r = (state - 0.9) * 10.0* self.range;
				ellipse([1.0,1.0,1.0,0.3], [w/2.0 -r, h/2.0 -r, 2.0*r , 2.0*r], view.scale(dx,dy), g );
			},
			_ => {},
		}
	}
}