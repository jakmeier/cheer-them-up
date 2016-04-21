/*
<<< enemy >>
*/

//! The super-module of the different enemies.


use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;
use super::jkm_shortest_path_map::JkmShortestPathMap;
use constants::ENEMY_SPEED;

pub mod basic_enemy;

pub trait Enemy {
	fn get_coordinates(&self) -> (f64, f64);
	fn set_coordinates(&mut self, f64, f64);
	fn get_enemy_type_id(&self) -> usize;
	fn get_size(&self) -> (f64,f64);
	fn get_speed(&self) -> usize;
	fn get_destination(&self) -> (f64, f64);
	fn set_destination(&mut self, (f64, f64));
	fn destination_reached(&mut self);
	fn get_destination_reached(&self) -> bool;
	fn set_destination_reached(&mut self, bool);
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64, sprite_array: &[Texture<Resources>]) {
		let (sprite_w, sprite_h) = sprite_array[self.get_enemy_type_id()].get_size();
		let (w,h) = self.get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_enemy_type_id()]), view.trans(x,y).scale(x_scale, y_scale), g);
	}
	fn update(&mut self, dt: f64, spm: &JkmShortestPathMap ) {
		//TODO: Berserker mode / attack base
		if self.get_destination_reached() {
			self.refresh_destination(&spm);
		}
		
		let (dest_x, dest_y) = self.get_destination();
		let (mut x, mut y) = self.get_coordinates();
		if x < dest_x {
			x += ENEMY_SPEED[self.get_speed()] * dt;
			if x > dest_x { x = dest_x; }
		}
		else if x > dest_x {
			x -= ENEMY_SPEED[self.get_speed()] * dt;
			if x < dest_x { x = dest_x; }
		}
		else if y < dest_y {
			y += ENEMY_SPEED[self.get_speed()] * dt;
			if y > dest_y { y = dest_y; }
		}
		else if y > dest_y {
			y -= ENEMY_SPEED[self.get_speed()] * dt;
			if y < dest_y { y = dest_y; }
		}
		else {
			self.destination_reached();
		}
		self.set_coordinates(x,y);
	}
	
	fn refresh_destination(&mut self, spm: &JkmShortestPathMap) {
	let (x, y) = self.get_coordinates();
		if self.get_destination_reached() {
			if let Some(d) = spm.next_checkpoint(x, y) {
				//let (old_x. old_y) = self.destination;
				self.set_destination(d) ;
				//let (x,y) = self.destination;
				//if x == old_x && y == old_y { /*activate attack base mode*/ }
				//else
				self.set_destination_reached(false);
			}
			// else activate berserker mode
		}
		else
		{
			if let Some(d) = spm.nearest_checkpoint(x, y) {
				self.set_destination(d);
			}//else: berserker mode
		}
		
	}
	
}