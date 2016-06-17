/*
<<< defence >>
*/

//! This is the Tower Defence Module. The remaining health points of the player are also stored in here and therefore this module can send a message that the player lost. This is the only way the game comes to an end other than exiting it manually.

extern crate jkm_shortest_path_map;
use self::jkm_shortest_path_map::JkmShortestPathMap;

use std::rc::Rc;

use constants::*;
use definitions::{Drawable, DrawRequest, DefenceUserInteraction, GameState, Statistics, Settings};

mod enemy;
mod tower;
mod projectile;
mod shop;
mod collision;
mod controller;

use self::enemy::*;
use self::tower::*;
use self::projectile::Projectile;

use super::piston_window::*;
use super::gfx_device_gl::Resources;
use super::gfx_device_gl::command::CommandBuffer;
use super::gfx_graphics::GfxGraphics;
use super::find_folder;

const HEALTH_BAR_HEIGHT: f64 = 8.0;
const NOTIFICATION_TIME: f64 = 1.0;

/// width and height are not related to the actual drawn size, they only define the size of the battle field, i.e. how many objects can fit on it.
/// dx and dy store the amount of pixels that are drawn per width / height for the entire module and are updated whenever draw() is called
pub struct Defence {
	controller: controller::Ctrl,
	hp: u32,
	width: f64, height: f64, dx: f64, dy: f64,
	shop: shop::Shop,
	general_sprites: Vec<Texture<Resources>>,
	shortest_path_map: JkmShortestPathMap,
	towers: Vec<Box<Tower>>,
	tower_templates: [Box<Tower>;NUMBER_OF_TOWERS],
	tower_sprites: Vec<Texture<Resources>>,
	enemies: Vec<Box<Enemy>>,
	enemy_sprites: Vec<Texture<Resources>>,
	kill_notifications: Vec<(f64,f64,f64)>,
	projectiles: Vec<Projectile>,
	projectile_sprites: Vec<Texture<Resources>>,
	explosions: Vec<((f64, f64, f64), f64)>,
	font: Glyphs,
	config: Rc<Settings>
}

impl Defence {
	/// width and height are not related to the actual drawn size, they only define the size of the battle field, i.e. how many objects can fit on it.
	pub fn new (w: &PistonWindow, hp: u32, width: f64, height: f64, state: &GameState, config: &Rc<Settings>) -> Defence {
		let controller = controller::Ctrl::new(width/2.0, 0.0);
		
		let bf_height = height * BF_SHOP_SPLIT_RATIO;
		let mut spm = JkmShortestPathMap::new( ((width-STD_ENEMY_W) / 2.0, 0.0), ((width-STD_ENEMY_W) / 2.0 , bf_height - STD_ENEMY_H ),(0.0, 0.0, width - STD_ENEMY_W, bf_height- STD_ENEMY_H) );
		spm.add_map_border();
		
		let enemies: Vec<Box<Enemy>> = Vec::new();
		
		let pros = Vec::new();
		
		let towers = Vec::new();
		
		// load generalsprites
		let mut general_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = GENERAL_BATTLEFIELD_SPRITE_LIST;
		let folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("defence").unwrap();
		for s in sprite_names.iter() {
			let f = folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			general_sprites.push(sprite);
		}
		
		//load enemy sprites
		let mut enemy_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = ENEMY_SPRITE_LIST;
		let enemy_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("enemies").unwrap();
		for s in sprite_names.iter() {
			let f = enemy_folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			enemy_sprites.push(sprite);
		}
		
		// load tower sprites
		let mut tower_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = TOWER_SPRITE_LIST;
		let tower_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("towers").unwrap();
		for s in sprite_names.iter() {
			let f = tower_folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			tower_sprites.push(sprite);
		}
		
		//load projectile sprites
		let mut projectile_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = PROJECTILE_SPRITE_LIST;
		let projectile_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("projectiles").unwrap();
		for s in sprite_names.iter() {
			let f = projectile_folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			projectile_sprites.push(sprite);
		}
		
		//create tower templates
		let t_temp :[Box<Tower>;NUMBER_OF_TOWERS] = [
			Box::new(basic_tower::BasicTower::new(0.0, 0.0, &state)),
			Box::new(aoe_tower::AoeTower::new(0.0, 0.0, &state)),
			Box::new(wall::Wall::new(0.0, 0.0, &state)),
			Box::new(slow_tower::SlowTower::new(0.0, 0.0, &state)),
			Box::new(rocket_tower::RocketTower::new(0.0, 0.0, &state)),
		];
		
		let shop = shop::Shop::new(&config);
		
		
		let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
		let ref font = assets.join("FiraSans-Regular.ttf");
		let factory = w.factory.borrow().clone();
		let glyphs = Glyphs::new(font, factory).unwrap();
		
		Defence {
			controller: controller,
			hp: hp,
			width: width, height: height, dx: 1.0, dy: 1.0,
			shop: shop,
			general_sprites: general_sprites,
			shortest_path_map: spm,
			towers: towers,
			tower_templates: t_temp,
			tower_sprites: tower_sprites,
			enemies: enemies,
			enemy_sprites: enemy_sprites,
			kill_notifications: Vec::new(),
			projectiles: pros, 
			projectile_sprites: projectile_sprites,
			explosions: Vec::new(),
			font: glyphs, 
			config: config.clone(),
		}
	}
	pub fn alive(&self) -> bool { self.hp > 0 }
	pub fn on_update(&mut self, upd: UpdateArgs, state: &GameState, stats: &mut Statistics) {
		
		// animations
			// explosions
		let mut to_remove = Vec::new();
		for (i, expl) in self.explosions.iter_mut().enumerate() {
			expl.1 -= upd.dt;
			if expl.1 <= 0.0 {
				to_remove.push(i);
			}
		}
		while let Some(i) = to_remove.pop() {
			self.explosions.swap_remove(i);
		}
			// notifications
		let mut to_remove = Vec::new();
		for (i, notification) in self.kill_notifications.iter_mut().enumerate() {
			notification.2 -= upd.dt;
			if notification.2 <= 0.0 {
				to_remove.push(i);
			}
		}
		while let Some(i) = to_remove.pop() {
			self.kill_notifications.swap_remove(i);
		}
		
		// enemy creation
		self.controller.update(upd.dt, &mut self.enemies);
		
		// towers
		let mut to_remove = Vec::new();
		for (i,t) in self.towers.iter().enumerate() {
			if t.is_dead() { 
				to_remove.push(i); 
				let (x,y) = t.get_coordinates();
				let (w,h) = t.get_tower_size();
				self.shortest_path_map.remove_obstacle(x-STD_ENEMY_W, y-STD_ENEMY_H, w+STD_ENEMY_W, h+STD_ENEMY_H);
			}
		}
		let map_changed = to_remove.len() > 0;
		while let Some(i) = to_remove.pop() {
			self.towers.swap_remove(i);
		}
		for t in self.towers.iter_mut() {
			if let Some(p) = t.update(upd.dt, &mut self.enemies, &state){
				self.projectiles.push(p);
			}
		}
		
		// projectiles 
		let mut to_remove = Vec::new();
		for (i, p) in self.projectiles.iter_mut().enumerate() {
			if let Some(explosion) = p.update(upd.dt, &mut self.enemies) {
				self.explosions.push((explosion, EXPLOSION_VISIBILITY_TIME));
			}
			if p.is_dead() { to_remove.push(i); }
		}
		while let Some(i) = to_remove.pop() {
			self.projectiles.swap_remove(i);
		}
		
		// enemies
		if map_changed { 
			for e in self.enemies.iter_mut() {
				e.refresh_destination(&self.shortest_path_map);
			}
		}
		let mut to_remove = Vec::new();
		for (i, e) in self.enemies.iter_mut().enumerate() {
			if e.update(upd.dt, &self.shortest_path_map, &mut self.towers) {
				// base reached
				self.hp -= 1;
				to_remove.push(i);
			}
			else if e.is_dead() { 
				to_remove.push(i); 
				stats.add_unit_kill( e.score_value() );
				let (x,y) = e.get_coordinates();
				self.kill_notifications.push((x, y, NOTIFICATION_TIME));
			}
		}
		while let Some(i) = to_remove.pop() {
			self.enemies.swap_remove(i);
		}
	}
	pub fn on_click(&mut self, x: f64, y: f64, state: &GameState) -> Option<DefenceUserInteraction> {
		if let Some(DefenceUserInteraction::BuyTower{x: w, y: h, tower_id}) = self.shop.on_click(x, y - BF_SHOP_SPLIT_RATIO * self.height * self.dy, state) {
			let x = x/self.dx;
			let y = y/self.dy;
			if self.valid_tower_place(x,y,w,h) {
				return Some(
					DefenceUserInteraction::BuyTower
						{
							x: x, 
							y: y,
							tower_id: tower_id,
						}
				);
			}
		
		}
		for t in self.towers.iter_mut() {
			t.on_click([x,y], [self.dx,self.dy]);
		}
		None
	}
	fn on_battlefield(&self, x: f64, y:f64, w: f64, h: f64) -> bool {x > 0.0 && x < (self.width - w) && y > 0.0 && y < (self.height - h)* BF_SHOP_SPLIT_RATIO}
	fn valid_tower_place(&self, x: f64, y:f64, w: f64, h: f64) -> bool {
		// Battlefield boundry
		if  self.on_battlefield(x,y,w,h) {	
			// Collision with base
			if x + w < BASE_X || x > (BASE_X + BASE_W) || y + h < BASE_Y || y > (BASE_Y + BASE_H) {
				// Collision with towers
				if !collision::towers_with_rectangle(&self.towers, x, y, w, h) {
					// Collission with enemies
					if !collision::enemies_with_rectangle(&self.enemies, x, y, w, h) {
						return true;
					}
				}
			}
		}
		false
	}
}

impl Defence {
	#[allow(unused_variables)]
	pub fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, mouse: [f64;2], state: &GameState) -> Option<DrawRequest> {
		
		let dx = w / self.width;
		let dy = h / self.height;
		
		self.dx = dx;
		self.dy = dy;
		
		let battlefield_h = h * BF_SHOP_SPLIT_RATIO;
		
		// background <=> battlefield
		let background = &self.general_sprites[0];
		let (sprite_w, sprite_h) = background.get_size();
		let x_scale = w/(sprite_w as f64);
		let y_scale = battlefield_h/(sprite_h as f64);
		image(background, view.scale(x_scale, y_scale), g);	
		
		// towers
		for t in self.towers.iter() {
			t.draw(g, view, mouse, dx, dy, &self.tower_sprites, &self.general_sprites, &state);
		}
		
		
		// enemies
		//let enemy_view = view.trans(w/2.0, 0.0);
		for e in self.enemies.iter() {
			e.draw(g, view, mouse, dx, dy, &self.enemy_sprites);
		}
		
		// projectiles
		for p in self.projectiles.iter() {
			p.draw(g, view, dx, dy, &self.projectile_sprites);
		}
		
		// explosions
		for &((x,y,r),_) in self.explosions.iter() {
			let explosion = &self.general_sprites[3];
			let (sprite_w, sprite_h) = explosion.get_size();
			let x_scale = 2.0 * r * dx /(sprite_w as f64);
			let y_scale = 2.0 * r * dy /(sprite_h as f64);
			image(explosion, view.trans((x-r) * dx,(y-r) * dy).scale(x_scale, y_scale), g);
		}
		
		// Kill notifications
		for &(x,y,t) in self.kill_notifications.iter() {
			let notification = &self.general_sprites[4];
			let (sprite_w, sprite_h) = notification.get_size();
			let x_scale = STD_ENEMY_W * dx /(sprite_w as f64);
			let y_scale = STD_ENEMY_H * dy /(sprite_h as f64);
			let q = 1.0 - t/NOTIFICATION_TIME;
			image(notification, view.trans((x + STD_ENEMY_W/2.0 * q) * dx,(y - q * STD_ENEMY_H * dy) * dy).scale(x_scale, y_scale).zoom(t), g);
		}
		
		// Life display
		text::Text::new_color([0.05,0.75,0.05,1.0], self.config.get_std_font_size()).draw( &(self.hp.to_string()), &mut self.font, &draw_state, view.trans(3.0, battlefield_h ), g);
		
		// shop
		let draw_req = self.shop.draw(g, view.trans(0.0, battlefield_h), w, h - battlefield_h, [mouse[0], mouse[1] - battlefield_h], &self.tower_sprites, dx, dy, state);
		match draw_req{
			Some(DrawRequest::DrawTower{tower_id}) => {
				self.tower_templates[tower_id].draw(g, view.trans(mouse[0],mouse[1]), [mouse[0] - 10.0, mouse[1]-10.0], dx, dy, &self.tower_sprites, &self.general_sprites, &state);
				let (w,h) = self.tower_templates[tower_id].get_tower_size();
				let x = mouse[0] / self.dx;
				let y = mouse[1] / self.dy;
				if !self.valid_tower_place(x, y, w, h) {
					let red_cross = &self.general_sprites[1];
					let (sprite_w, sprite_h) = red_cross.get_size();
					let x_scale = 40.0 * self.config.get_battlefield_scaling_factor()/(sprite_w as f64);
					let y_scale = 40.0 * self.config.get_battlefield_scaling_factor()/(sprite_h as f64);
					let x = mouse[0] + w*self.dx - 40.0 * self.config.get_battlefield_scaling_factor();
					let y = mouse[1];
					image(red_cross, view.trans(x,y).scale(x_scale, y_scale), g);
				}
			}
			_ => {}
		}
		draw_req
	}
}

// 
impl Defence {
	pub fn build_tower(&mut self, x: f64, y: f64, tower_id: usize, state: &GameState) {
		let new_tower : Box<Tower> =
		match tower_id {
			BASIC_TID => Box::new(tower::basic_tower::BasicTower::new(x, y, &state)),
			AOE_TID => Box::new(tower::aoe_tower::AoeTower::new(x, y, &state)),
			WALL_TID => Box::new(tower::wall::Wall::new(x, y, &state)),
			SLOW_TID => Box::new(tower::slow_tower::SlowTower::new(x, y, &state)),
			ROCKET_TID => Box::new(tower::rocket_tower::RocketTower::new(x, y, &state)),
			_ => panic!("Unexpected tower ID: {}", tower_id),
		};
		let (w,h) = new_tower.get_tower_size();
		// TODO (if different sized enemies exist): replace constants with some correct value!
		self.shortest_path_map.insert_obstacle(x-STD_ENEMY_W, y-STD_ENEMY_H, w+STD_ENEMY_W, h+STD_ENEMY_H );
		self.towers.push(new_tower);
		
		for e in self.enemies.iter_mut() {
			e.refresh_destination(&self.shortest_path_map);
		}
		
	}
	pub fn cascade_health_upgrade(&mut self, state: &GameState) {
		for tower in self.towers.iter_mut() {
			tower.apply_health_upgrade(state);
		}
	}
}