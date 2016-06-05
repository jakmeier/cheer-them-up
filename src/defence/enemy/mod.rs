/*
<<< enemy >>
*/

//! The super-module of the different enemies.


use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

use std;

use super::jkm_shortest_path_map::JkmShortestPathMap;
use super::tower::Tower;
use constants::*;

pub mod basic_enemy;

pub struct EnemyAttributes {
	x: f64, y: f64,
	w: f64, h: f64,
	life: f64,
	speed: usize,
	health: f64, max_health: f64,
	attack: f64, attack_ratio: f64, attack_reload: f64,
	destination: (f64, f64),
	destination_reached: bool,
	base_reached: bool,
	berserker_mode: bool, attack_target: Option<usize>,
	animation_offset: (f64, f64),
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
	fn is_dead(&self) -> bool { self.get().health <= 0.0 }
	fn attack_enemy(&mut self, power: f64) { self.get_mut().health -= power; }
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, mouse: [f64;2], dx: f64, dy: f64, sprite_array: &[Texture<Resources>]) {
		let (sprite_w, sprite_h) = sprite_array[self.get_enemy_type_id()].get_size();
		let (w,h) = self.get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		let offset = self.get().animation_offset;
		let x = x + offset.0;
		let y = y + offset.1;
		image(&(sprite_array[self.get_enemy_type_id()]), view.trans(x*dx,y*dy).scale(x_scale, y_scale), g);
		//Display health if the mouse hovers over the tower
		if mouse[0]/dx > x && mouse[0]/dx < x+w && mouse[1]/dy > y && mouse[1]/dy < y+h {
			let hp_ratio = self.get().health / self.get().max_health;
			rectangle([0.0, 0.8, 0.0, 1.0], [0.0, -HEALTH_BAR_HEIGHT, w*dx*hp_ratio, HEALTH_BAR_HEIGHT], view.trans(x*dx,y*dy), g );
		}
	}
	
	fn update(&mut self, dt: f64, spm: &JkmShortestPathMap, towers: &mut Vec<Box<Tower>> ) {
		
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
			debug_assert!(self.get().base_reached);
			let (x, _) = self.get_coordinates();
			self.set_coordinates(x, 0.0);
			self.get_mut().base_reached = false;
			self.get_mut().berserker_mode = false;
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
		
		
	}
	
	fn walk_a_step(&mut self, dest_x: f64, dest_y: f64, dt: f64) {
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
	
	// Walk in a straight line, ignoring all obstacles, into the range of the tower and attack it
	// This version: Standard melee attack
	fn attack_tower(&mut self, target: &mut Box<Tower>, dt: f64) {
		
		let (x,y) = target.get_coordinates();
		let (w,h) = target.get_tower_size();
		let enemy_w = self.get().w;
		let enemy_h = self.get().h;
		let enemy_x = self.get().x;
		let enemy_y = self.get().y;
		
		//animation parameters
		let period_third = self.get().attack_ratio / 3.0;
		let amplitude = enemy_w / 2.0;
		let c = amplitude/period_third;
		let a = 2.0* amplitude/(period_third* period_third);
			
			// animation
		if  enemy_x <= x + w && enemy_x + enemy_w >= x && y == enemy_y + enemy_h {
			// enemy on top
			//attack
			self.get_mut().attack_reload += dt;
			if self.get().attack_reload >= self.get().attack_ratio {
				target.attack_tower( self.get().attack );
				self.get_mut().attack_reload = 0.0;
			}
			
			// animation
			match self.get().attack_reload {
				t if t >= 0.0 && t <= period_third => {
						self.get_mut().animation_offset = (0.0, t*t*(-c)/2.0 );
					},
				total_t if total_t >= period_third && total_t <= (2.0 * period_third) => {
						let t = total_t - period_third;
						self.get_mut().animation_offset = (0.0, (period_third*period_third*(-c)/2.0) + t*((period_third*(-c)) + t*c/2.0 ));
					},
				total_t if total_t >= (2.0 * period_third) && total_t <= self.get().attack_ratio => {
						let t = total_t - 2.0 * period_third;
						self.get_mut().animation_offset = (0.0, -amplitude + (a/2.0)* t*t );
					},
				_ => self.get_mut().animation_offset = (0.0,0.0),
			}
			
			
		}
		else if x == enemy_x + enemy_w && enemy_y <= y + h && enemy_y + enemy_h >= y {
			// enemey on the left side
			//attack
			self.get_mut().attack_reload += dt;
			if self.get().attack_reload >= self.get().attack_ratio {
				target.attack_tower( self.get().attack );
				self.get_mut().attack_reload = 0.0;
			}
			
			// animation
			match self.get().attack_reload {
				t if t >= 0.0 && t <= period_third => {
						self.get_mut().animation_offset = ( t*t*(-c)/2.0, 0.0 );
					},
				total_t if total_t >= period_third && total_t <= (2.0 * period_third) => {
						let t = total_t - period_third;
						self.get_mut().animation_offset = ((period_third*period_third*(-c)/2.0) + t*((period_third*(-c)) + t*c/2.0 ), 0.0);
					},
				total_t if total_t >= (2.0 * period_third) && total_t <= self.get().attack_ratio => {
						let t = total_t - 2.0 * period_third;
						self.get_mut().animation_offset = (-amplitude + (a/2.0)* t*t, 0.0 );
					},
				_ => self.get_mut().animation_offset = (0.0,0.0),
			}
		}
		else if x + w == enemy_x && enemy_y <= y + h && enemy_y + enemy_h >= y {
			// enemy on the right side
			//attack
			self.get_mut().attack_reload += dt;
			if self.get().attack_reload >= self.get().attack_ratio {
				target.attack_tower( self.get().attack );
				self.get_mut().attack_reload = 0.0;
			}
			
			// animation
			match self.get().attack_reload {
				t if t >= 0.0 && t <= period_third => {
						self.get_mut().animation_offset = (-t*t*(-c)/2.0, 0.0);
					},
				total_t if total_t >= period_third && total_t <= (2.0 * period_third) => {
						let t = total_t - period_third;
						self.get_mut().animation_offset = (-(period_third*period_third*(-c)/2.0) + t*((period_third*(-c)) + t*c/2.0 ), 0.0);
					},
				total_t if total_t >= (2.0 * period_third) && total_t <= self.get().attack_ratio => {
						let t = total_t - 2.0 * period_third;
						self.get_mut().animation_offset = (amplitude + (a/2.0)* t*t, 0.0 );
					},
				_ => self.get_mut().animation_offset = (0.0,0.0),
			}
		}
		else {
			if enemy_y + enemy_h < y {self.walk_a_step(enemy_x, y - enemy_h, dt); }
			else if enemy_x + enemy_w < x {self.walk_a_step(x - enemy_w, enemy_y, dt); }
			else {self.walk_a_step(x + w, enemy_y, dt); }
		}
		
	}
	// Adjust attribute attack_traget
	// This version: Find target for standard attack
	fn find_target(&mut self, towers: &Vec<Box<Tower>>, destination_x: f64, destination_y: f64) {
		
		let enemy_x = self.get().x;
		let enemy_y = self.get().y;
		let enemy_w = self.get().w;
		let enemy_h = self.get().h;
		
		if (enemy_x - destination_x).abs() < EPS && (enemy_y - destination_y).abs() < EPS {
			// base_reached
			self.get_mut().base_reached = true;
			self.get_mut().berserker_mode = false;
			return;
		} 
		
		let mut new_target: (f64, usize) = (std::f64::INFINITY, 0);
		
		if enemy_y < destination_y {
			// Walk down
			for (i,t) in towers.iter().enumerate() {
				let (x,y) = t.get_coordinates();
				let (w,h) = t.get_tower_size();
				if enemy_x +enemy_w >= x && enemy_x <= x + w 
					&& y >= enemy_y //+ enemy_h
					&& (y-enemy_y-enemy_h).abs() < new_target.0
					{ new_target = ((y-enemy_y-enemy_h).abs(), i); }
			}			
			if new_target.0 == std::f64::INFINITY {
				self.get_mut().destination = (enemy_x, destination_y);
				self.get_mut().berserker_mode = false;
			}
		}
		else {
			// Walk horizontally
			if enemy_x < destination_x {
				// Walk right
				for (i,t) in towers.iter().enumerate() {
					let (x,y) = t.get_coordinates();
					let (w,h) = t.get_tower_size();
					if x <= destination_x && x + w >= enemy_x //horizontally between enemy and destination
						&& y <= destination_y + enemy_h  && y + h >= destination_y //vertically blocking destination
						&& (x-enemy_x-enemy_w).abs() < new_target.0
						{ new_target = ((x-enemy_x-enemy_w).abs(), i); }
				}
			}
			else {
				// Walk left
				for (i,t) in towers.iter().enumerate() {
					let (x,y) = t.get_coordinates();
					let (w,h) = t.get_tower_size();
					if x + w >= destination_x && x <= enemy_x + enemy_w //horizontally between enemy and destination
						&& y <= destination_y + enemy_h  && y + h >= destination_y //vertically blocking destination
						&& (enemy_x - x -w).abs() < new_target.0
						{ new_target = ((enemy_x-x-w).abs(), i); }
				}
			}

		}
		debug_assert!(new_target.0 < std::f64::INFINITY );
		self.get_mut().attack_target = Some(new_target.1);
	}
	
	// recompute the shortest path and attack target for the enemy 
	fn refresh_destination(&mut self, spm: &JkmShortestPathMap) {
	if self.get().berserker_mode {
		self.get_mut().berserker_mode = false;
		self.get_mut().attack_target = None;
	}
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
				 else {self.get_mut().berserker_mode = true;}
			}
			else
			{
				if let Some(d) = spm.nearest_checkpoint(x, y) { 
					self.get_mut().destination = d;
				}
				else {self.get_mut().berserker_mode = true;}
			}
		}
	}
	
}