//! The most basic attacker that can appear.

use constants::*;
use super::Enemy;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub struct BasicEnemy {
	x: f64, y: f64,
	w: f64, h: f64,
	life: f64,
	speed: usize,
	destination: (f64, f64),
	destination_reached: bool,
}

impl BasicEnemy {
	pub fn new() -> BasicEnemy {
	let x = -(STD_ENEMY_W/2.0);
	let y = 0.0;
		BasicEnemy {
			x: x, y: y,
			w:STD_ENEMY_W, h:STD_ENEMY_H ,
			life: 100.0,
			speed: 2,
			destination: (x,y),
			destination_reached: true,
		}
	}
}

impl Enemy for BasicEnemy {

		fn get_coordinates(&self) -> (f64, f64) {
			(self.x, self.y)
		}
		
		fn set_coordinates(&mut self, x: f64, y: f64) {
			self.x = x;
			self.y = y;
		}
		
		fn get_enemy_type_id(&self) -> usize {
			BASIC_EID
		}
		
		fn get_size(&self) -> (f64,f64) {
			(self.w, self.h)
		}
		
		fn get_speed(&self) -> usize {
			self.speed
		}
		
		
		fn get_destination(&self) -> (f64, f64) { self.destination }
		fn set_destination(&mut self, d: (f64, f64)) { self.destination = d; }
		fn get_destination_reached(&self) -> bool {self.destination_reached}
		fn set_destination_reached(&mut self, dr: bool) {self.destination_reached = dr;}
		fn destination_reached(&mut self) {self.destination_reached = true;}
		
}