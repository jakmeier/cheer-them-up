//! The Jack in the box towe
//! Lowers health from all nearby enenmies whenever it shoots, there are no projectiles produced by this tower

use constants::*;
use defence::collision::*;
use defence::enemy::Enemy;
use defence::projectile::Projectile;
use super::{Tower, TowerAttributes};
use definitions::GameState;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub struct AoeTower {
	attributes: TowerAttributes,
	range: f64,
}

impl AoeTower {
	pub fn new(x:f64, y:f64, upgrades: &GameState) -> AoeTower {
		let hp = apply_defence_bonus(TOWER_BASE_HEALTH_LIST[AOE_TID], upgrades.tower_upgrades[AOE_TID][1]);
		AoeTower {
			attributes:
				TowerAttributes{
					x: x, y: y,
					max_health: hp,
					health: hp,
					reload_time: TOWER_BASE_ATTACK_RATIO_LIST[AOE_TID],
					cooldown: 0.0,
					attack_power: TOWER_BASE_ATTACK_LIST[AOE_TID],
					show_delete_button: false,
				},
				range: TOWER_BASE_RANGE_LIST[AOE_TID],
		}
	}
}

fn apply_range_bonus(r: f64, b: u8) -> f64 {
	r + (b as f64) * 5.0
}
fn apply_attack_bonus(a: f64, b: u8) -> f64 {
	a + (b as f64) * 5.0
}
fn apply_defence_bonus(d: f64, b: u8) -> f64 {
	d + (b as f64) * 20.0
}

impl Tower for AoeTower {
	fn get_tower_type_id(&self) -> usize { AOE_TID }
	fn get(&self) -> &TowerAttributes { &self.attributes }
	fn get_mut(&mut self) -> &mut TowerAttributes { &mut self.attributes }
	
	fn perform_attack(&self, enemies: &mut Vec<Box<Enemy>>, upgrades: &GameState) -> Option<Projectile> {
		let (w,h) = self.get_tower_size();
		let center_x = self.attributes.x + w / 2.0;
		let center_y = self.attributes.y + h / 2.0;
		let range = apply_range_bonus(self.range, upgrades.tower_upgrades[self.get_tower_type_id()][2]);
		let ap = apply_attack_bonus(self.attributes.attack_power, upgrades.tower_upgrades[self.get_tower_type_id()][0]);
		let to_attack = find_all_enemies_in_circle(enemies, center_x, center_y, range);
		for &i in to_attack.iter() {
			enemies[i].attack_enemy(ap);
		}
		None
	}
	
	fn calculate_defence_bonus(&self, d: f64, b: u8) -> f64 {
		apply_defence_bonus(d,b)
	}
	
	// OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	OVERWRITE	
	
	fn attack_animation(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64, upgrades: &GameState) {
		let state = self.attributes.cooldown / self.attributes.reload_time;
		let range = apply_range_bonus(self.range, upgrades.tower_upgrades[self.get_tower_type_id()][2]);
		match state {
			0.9 ... 1.0 => {
				let (w,h) = self.get_tower_size();
				let r = (state - 0.9) * 10.0* range;
				ellipse([1.0,1.0,1.0,0.3], [w/2.0 -r, h/2.0 -r, 2.0*r , 2.0*r], view.scale(dx,dy), g );
			},
			_ => {},
		}
	}
}