/*
<<< tower >>
*/

//! Super-module of the defensive towers.

pub mod basic_tower;
pub mod aoe_tower;
pub mod wall;

use constants::*;
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
	
	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, mouse:[f64;2], dx: f64, dy: f64, sprite_array: &[Texture<Resources>], upgrades: &GameState) {
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
}
