/*
<<< tower >>
*/

//! Super-module of the defensive towers.

pub mod basic_tower;
pub mod aoe_tower;

use constants::*;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;



pub trait Tower{
	fn get_coordinates(&self) -> (f64, f64);
	fn set_coordinates(&mut self, x:f64, y:f64);
	fn get_tower_type_id(&self) -> usize;
	
	//fn apply_tower_upgrades(&mut self, TowerUpgrades tu);
		
	fn get_tower_size(&self) -> (f64, f64) {
		(DEFAULT_TOWER_W, DEFAULT_TOWER_H)
	}
	
	fn attack_tower(&mut self, power: f64) {
		let hp = self.get_health() - power;
		if hp < 0.0 {  self.set_health(0.0);}
		else { self.set_health(hp); }
	}
	
	fn get_health(&self) -> f64;
	fn set_health(&mut self, f64);
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64, sprite_array: &[Texture<Resources>]) {
		let (w, h) = self.get_tower_size();
		let (sprite_w, sprite_h) = sprite_array[self.get_tower_type_id()].get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_tower_type_id()]), view.trans(x*dx,y*dy).scale(x_scale, y_scale), g);
	}
	fn update(&mut self, dt: f64 ) {
		//TODO
	}
	fn is_dead(&self) -> bool {
		self.get_health() <= 0.0
	}
}
