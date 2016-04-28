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


struct TowerAttributes {
	x: f64, y: f64,
	max_health: f64, health: f64,
}

pub trait Tower{
	fn get_tower_type_id(&self) -> usize;
	fn get(&self) -> &TowerAttributes;
	fn get_mut(&mut self) -> &mut TowerAttributes;
	fn get_coordinates(&self) -> (f64, f64) {
		(self.get().x, self.get().y)
	}
	fn set_coordinates(&mut self, x:f64, y:f64){
		self.get_mut().x = x;
		self.get_mut().y = y;
	}	
	//fn apply_tower_upgrades(&mut self, TowerUpgrades tu);
		
	fn get_tower_size(&self) -> (f64, f64) {
		(DEFAULT_TOWER_W, DEFAULT_TOWER_H)
	}
	fn attack_tower(&mut self, power: f64) {
		let hp = self.get().health - power;
		if hp < 0.0 {  self.get_mut().health = 0.0;}
		else { self.get_mut().health= hp; }
	}
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, mouse:[f64;2], dx: f64, dy: f64, sprite_array: &[Texture<Resources>]) {
		let (w, h) = self.get_tower_size();
		let (sprite_w, sprite_h) = sprite_array[self.get_tower_type_id()].get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_tower_type_id()]), view.trans(x*dx,y*dy).scale(x_scale, y_scale), g);
		//Display health if the mouse hovers over the tower
		if mouse[0]/dx > x && mouse[0]/dx < x+w && mouse[1]/dy > y && mouse[1]/dy < y+h {
			let hp_ratio = self.get().health / self.get().max_health;
			rectangle([0.0, 0.8, 0.0, 1.0], [0.0, -HEALTH_BAR_HEIGHT, w*dx*hp_ratio, HEALTH_BAR_HEIGHT], view.trans(x*dx,y*dy), g );
		}
	}
	fn update(&mut self, dt: f64 ) {
		//TODO
	}
	fn is_dead(&self) -> bool {
		self.get().health <= 0.0
	}
}
