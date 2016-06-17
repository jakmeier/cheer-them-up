/*
<<< tower >>
*/

//! Super-module of the defensive towers.

pub mod basic_tower;
pub mod aoe_tower;
pub mod wall;
pub mod slow_tower;
pub mod rocket_tower;

use constants::*;
use super::HEALTH_BAR_HEIGHT;
use super::projectile::Projectile;
use super::enemy::Enemy;
use definitions::GameState;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;


pub struct TowerAttributes {
	x: f64, y: f64,
	max_health: f64, health: f64,
	reload_time: f64, cooldown: f64, 
	attack_power: f64,
	show_delete_button: bool,
}

pub trait Tower{
	fn get_tower_type_id(&self) -> usize;
	fn get(&self) -> &TowerAttributes;
	fn get_mut(&mut self) -> &mut TowerAttributes;
	fn get_coordinates(&self) -> (f64, f64) {(self.get().x, self.get().y)}
	fn set_coordinates(&mut self, x:f64, y:f64){
		self.get_mut().x = x;
		self.get_mut().y = y;
	}	
	//fn apply_tower_upgrades(&mut self, TowerUpgrades tu);
		
	fn get_tower_size(&self) -> (f64, f64) { TOWER_SIZE_LIST[self.get_tower_type_id()] }
	fn attack_tower(&mut self, power: f64) {
		let hp = self.get().health - power;
		if hp < 0.0 {  self.get_mut().health = 0.0;}
		else { self.get_mut().health= hp; }
	}
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, mouse:[f64;2], dx: f64, dy: f64, sprite_array: &[Texture<Resources>], general_sprite_array: &[Texture<Resources>], upgrades: &GameState) {
		let (w, h) = self.get_tower_size();
		let (sprite_w, sprite_h) = sprite_array[self.get_tower_type_id()].get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_tower_type_id()]), view.trans(x*dx,y*dy).scale(x_scale, y_scale), g);
		self.attack_animation(g, view.trans(x*dx,y*dy), dx, dy, &upgrades);
		//Display health if the mouse hovers over the tower
		if mouse[0]/dx > x && mouse[0]/dx < x+w && mouse[1]/dy > y && mouse[1]/dy < y+h {
			let hp_ratio = self.get().health / self.get().max_health;
			rectangle([0.0, 0.8, 0.0, 1.0], [0.0, -HEALTH_BAR_HEIGHT, w*dx*hp_ratio, HEALTH_BAR_HEIGHT], view.trans(x*dx,y*dy), g );
		}
		//Display delete button
		if self.get().show_delete_button {
			let del_button = &general_sprite_array[2];
			let (sprite_w, sprite_h) = del_button.get_size();
			let s = if w*dx < h*dy { w } else { h };
			let scale = if w*dx < h*dy { w/sprite_w as f64 * dx } else { h / sprite_h as f64 * dy };
			image(del_button, view.trans((x-s)*dx,y*dy).scale(scale, scale), g);
		}
	}
	#[allow(unused_variables)]
	fn attack_animation(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, dx: f64, dy: f64, upgrades: &GameState){}
	
	fn update(&mut self, dt: f64, enemies: &mut Vec<Box<Enemy>>, upgrades: &GameState) -> Option<Projectile> {
		self.get_mut().cooldown += dt;
		if self.get().cooldown > self.get().reload_time {
			self.get_mut().cooldown = 0.0;
			self.perform_attack(enemies, &upgrades)
		}
		else { None }
	}
	fn perform_attack(&self, &mut Vec<Box<Enemy>>, upgrades: &GameState,) -> Option<Projectile> ;
	fn is_dead(&self) -> bool {
		self.get().health <= 0.0
	}
	
	fn calculate_defence_bonus(&self, d: f64, b: u8) -> f64;
	/// Increases the used health upgrade to the current level. 
	/// This must be called on all living towers when a new defence upgrade is bought to change their HP. 
	/// The HP bonus for new towers is applied directly in the constructor and is not affected by this function.
	/// If no upgrade was apllied, this function will do nothing.
	fn apply_health_upgrade(&mut self, upgrades: &GameState) {
		let max_hp_after = self.calculate_defence_bonus(TOWER_BASE_HEALTH_LIST[self.get_tower_type_id()], upgrades.tower_upgrades[self.get_tower_type_id()][1]);
		let bonus_hp = max_hp_after - self.get().max_health;
		self.get_mut().max_health = max_hp_after;
		self.get_mut().health += bonus_hp;
	}
	
	fn on_click(&mut self, mouse: [f64;2], scale: [f64;2]) {
		
		let (x,y) = self.get_coordinates();
		let x = x*scale[0];
		let y = y*scale[1];
		let (w, h) = self.get_tower_size();
		//println!("mouse:({}|{}) tower:({}|{})({}|{})", mouse[0], mouse[1], x, y, w, h);
		let s = if w*scale[0] < h*scale[1] { w*scale[0] } else { h*scale[1] };
		if self.get().show_delete_button && (mouse[0] > x-s && mouse[0] < x && mouse[1] > y && mouse[1] < y+s) {
			self.get_mut().health = 0.0;
		}
		else if mouse[0] > x && mouse[0] < x+w*scale[0] && mouse[1] > y && mouse[1] < y+h*scale[1] {
			self.get_mut().show_delete_button = !self.get().show_delete_button;
		}
		else { self.get_mut().show_delete_button = false; }
	}
}
