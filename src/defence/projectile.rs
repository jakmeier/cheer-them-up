/*
<<< projectile >>
*/

//! 

use constants::*;
use super::collision::*;
use super::enemy::Enemy;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;
use std::f64;

/// represents a singular projectile on the map
pub struct Projectile {
	x: f64, y: f64, 
	vx: f64, vy: f64, 
	impact: ProjectileImpact,
	power: f64,
	life_time: f64,
}

enum ProjectileImpact {
	SingleTarget,
	AoE{radius: f64},
	Slow{charges: usize},
}

impl Projectile {

//	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS

	// The most basic projectile, hitting one target only with the standard power
	pub fn new(x: f64, y: f64, dest_x: f64, dest_y: f64, power: f64, range: f64) -> Projectile {
		let distance = ((y-dest_y)*(y-dest_y) + (x-dest_x)*(x-dest_x)).sqrt();
		let vx = PROJECTILE_VELOCITY/ distance * (dest_x-x);
		let vy = PROJECTILE_VELOCITY/ distance * (dest_y-y);
		Projectile {
			x:x, y:y,
			vx:vx, vy:vy, 
			impact: ProjectileImpact::SingleTarget,
			power: power, 
			life_time: range / PROJECTILE_VELOCITY,
		}
	}
	pub fn new_aoe(x: f64, y: f64, dest_x: f64, dest_y: f64, power: f64, range: f64, r: f64) -> Projectile {
		let distance = ((y-dest_y)*(y-dest_y) + (x-dest_x)*(x-dest_x)).sqrt();
		let vx = PROJECTILE_VELOCITY/ distance * (dest_x-x);
		let vy = PROJECTILE_VELOCITY/ distance * (dest_y-y);
		Projectile {
			x:x, y:y,
			vx:vx, vy:vy, 
			impact: ProjectileImpact::AoE{radius: r},
			power: power, 
			life_time: range / PROJECTILE_VELOCITY,
		}
	}
	pub fn new_slow(x: f64, y: f64, dest_x: f64, dest_y: f64, charges: usize, range: f64) -> Projectile {
		let distance = ((y-dest_y)*(y-dest_y) + (x-dest_x)*(x-dest_x)).sqrt();
		let vx = SLOW_PROJECTILE_VELOCITY/ distance * (dest_x-x);
		let vy = SLOW_PROJECTILE_VELOCITY/ distance * (dest_y-y);
		Projectile {
			x:x, y:y,
			vx:vx, vy:vy, 
			impact: ProjectileImpact::Slow{charges: charges},
			power: 0.0, 
			life_time: range / SLOW_PROJECTILE_VELOCITY,
		}
	}
//	METHODS		METHODS		METHODS		METHODS		METHODS		METHODS		METHODS	

	pub fn update (&mut self, dt: f64, enemies: &mut Vec<Box<Enemy>>) -> Option<(f64,f64,f64)> {
		let mut result = None;
		self.life_time -= dt;
		let dx = self.vx * dt;
		let dy = self.vy * dt;
		
		match self.impact {
			ProjectileImpact::Slow{..} => {
				let mut victims = find_all_enemies_in_rectangle(&enemies, self.x - ROCKET_PROJECTILE_SIZE.0 /2.0, self.y - ROCKET_PROJECTILE_SIZE.1 /2.0, ROCKET_PROJECTILE_SIZE.0, ROCKET_PROJECTILE_SIZE.1);
				while let Some(i) = victims.pop() {
					self.collide(&mut enemies[i]);
				}
			}
			_ => { if let Some(i) = enemies_with_segment(&enemies, self.x, self.y, dx, dy){
					if let Some(explosion_radius) = self.collide(&mut enemies[i]) {
						let (enemy_x,enemy_y) = enemies[i].get_coordinates(); 
						let (w,h) = enemies[i].get_size(); 
						let x = enemy_x + w /2.0;
						let y = enemy_y + h /2.0;
						result = Some((x, y, explosion_radius));
						let mut victims = find_all_enemies_in_circle(&enemies, x, y, explosion_radius);
						while let Some(i) = victims.pop() {
							enemies[i].attack_enemy(self.power);
						}
					}
				}
			}
		}
		
		self.x += dx;
		self.y += dy;
		result
	}
	
	// returns maybe the radius of an explosion
	fn collide (&mut self, enemy: &mut Box<Enemy>) -> Option<f64> {
		match self.impact {
			ProjectileImpact::SingleTarget => {
				enemy.attack_enemy(self.power);
				self.life_time = -1.0;
				None
			},
			ProjectileImpact::Slow{ref mut charges} => {
				if *charges > 0 {
					*charges -= enemy.slow_down(*charges);
				}
				else {
					self.life_time = -1.0;
				}
				None
			},
			ProjectileImpact::AoE{radius} => {
				self.life_time = -1.0;
				Some(radius)
			},
		}
	}
	
	pub fn is_dead(&self) -> bool { self.life_time < 0.0 }
	
	pub fn draw(	&self, 
					g: &mut GfxGraphics<Resources, CommandBuffer>, 
					view: math::Matrix2d,  
					dx: f64, dy: f64, 
					sprite_array: &[Texture<Resources>]) 
	{
		let sprite;
		let w; let h;
		match self.impact {
			ProjectileImpact::SingleTarget => {
				sprite = &sprite_array[0];
				w = BASIC_PROJECTILE_SIZE.0;
				h = BASIC_PROJECTILE_SIZE.1;
			}
			ProjectileImpact::AoE{..} => {
				sprite = &sprite_array[2];
				w = ROCKET_PROJECTILE_SIZE.0;
				h = ROCKET_PROJECTILE_SIZE.1;
			}
			ProjectileImpact::Slow{..} => {
				sprite = &sprite_array[1];
				w = SLOW_PROJECTILE_SIZE.0;
				h = SLOW_PROJECTILE_SIZE.1;
			}
		}
		let (sprite_w, sprite_h) = sprite.get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let alpha = (self.vy/self.vx).atan() * 180.0 / f64::consts::PI;
		image(sprite, view.trans(self.x*dx,self.y*dy).scale(x_scale, y_scale).rot_deg(alpha), g);
	}
}
