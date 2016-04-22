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

struct EnemyAttributes {
	x: f64, y: f64,
	w: f64, h: f64,
	life: f64,
	speed: usize,
	destination: (f64, f64),
	destination_reached: bool,
	base_reached: bool,
	berserker_mode: bool,
}

pub trait Enemy {
	fn get_mut(&mut self) -> &mut EnemyAttributes;
	fn get(&self) -> &EnemyAttributes;
	fn get_enemy_type_id(&self) -> usize;
	
	fn get_coordinates(&self) -> (f64, f64) {
		(self.get().x, self.get().y )
	}
	fn set_coordinates(&mut self, x: f64, y: f64){
		self.get_mut().x = x;
		self.get_mut().y = y;
	}
	fn get_size(&self) -> (f64,f64) {
		(self.get().w, self.get().h )
	}
	fn destination_reached(&mut self) {
		self.get_mut().destination_reached = true;
	}
	
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64, sprite_array: &[Texture<Resources>]) {
		let (sprite_w, sprite_h) = sprite_array[self.get_enemy_type_id()].get_size();
		let (w,h) = self.get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_enemy_type_id()]), view.trans(x*dx,y*dy).scale(x_scale, y_scale), g);
	}
	fn update(&mut self, dt: f64, spm: &JkmShortestPathMap ) {
		//TODO: Berserker mode / attack base
		
		//WALKING
		if !self.get().base_reached {
			if self.get().destination_reached {
				self.refresh_destination(&spm);
			}
			
			let (dest_x, dest_y) = self.get().destination;
			let (mut x, mut y) = self.get_coordinates();
			let step = ENEMY_SPEED[self.get().speed] * dt;
			
			if x < dest_x {
				x += step;
				if x > dest_x { x = dest_x; }
			}
			else if x > dest_x {
				x -= step;
				if x < dest_x { x = dest_x; }
			}
			else if y < dest_y {
				y += step;
				if y > dest_y { y = dest_y; }
			}
			else if y > dest_y {
				y -= step;
				if y < dest_y { y = dest_y; }
			}
			else {
				debug_assert!(x==dest_x && y == dest_y);
				self.destination_reached();
			}
			self.set_coordinates(x,y);
		}
	}
	
	fn refresh_destination(&mut self, spm: &JkmShortestPathMap) {
	if !self.get().base_reached {
		let (x, y) = self.get_coordinates();
			if self.get().destination_reached {
				if let Some(d) = spm.next_checkpoint(x, y) {
					let (old_x, old_y) = self.get().destination;
					self.get_mut().destination = d ;
					let (new_x,new_y) = self.get().destination;
					if new_x == old_x && new_y == old_y { self.get_mut().base_reached = true; }
					else { self.get_mut().destination_reached = false; }
				}
				 else {println!("No next checkpoint: activate berserker mode");}
			}
			else
			{
				if let Some(d) = spm.nearest_checkpoint(x, y) { 
					self.get_mut().destination = d;
				}
				else {println!("No nearest checkpoint: activate berserker mode");}
			}
		}
	}
	
}