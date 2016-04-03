//! The most basic attacker that can appear.

use constants::*;
use super::Enemy;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub struct BasicEnemy {
	x: f64, y: f64,
	life: f64,
	speed: u8,
}

impl BasicEnemy {
	pub fn new() -> BasicEnemy {
		BasicEnemy {
			x: 0.0, y:0.0,
			life: 100.0,
			speed: 2,
		}
	}
}

impl Enemy for BasicEnemy {

		fn get_coordinates(&self) -> (f64, f64) {
			(self.x, self.y)
		}
		
		fn get_enemy_type_id(&self) -> usize {
			BASIC_EID
		}
		
		///Overwrites default behaviour
		fn update(&mut self, dt: f64 ){
			self.y += dt * 5.0 * self.speed as f64;
		}
		
}