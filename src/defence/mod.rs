/*
<<< defence >>
*/

//! This is the Tower Defence Module. The remaining health points of the player are also stored in here and therefore this module can send a message that the player lost. This is the only way the game comes to an end other than exiting it manually.

extern crate jkm_shortest_path_map;
use self::jkm_shortest_path_map::JkmShortestPathMap;

use constants::*;
use super::Drawable;
use DrawRequest;

mod enemy;
mod tower;
mod shop;
mod collision;

use self::enemy::*;
use self::tower::*;
use self::collision::*;

use super::piston_window::*;
use super::gfx_device_gl::Resources;
use super::gfx_device_gl::command::CommandBuffer;
use super::gfx_graphics::GfxGraphics;
use super::find_folder;




/// Used to communicate with the root of the project. Mostly to request constructions and upgrades that are only allowed if there are enough resources.
pub enum DefenceUserInteraction{
	BuyTower{x: f64, y:f64, tower_id: usize},
}

/// width and height are not related to the actual drawn size, they only define the size of the battle field, i.e. how many objects can fit on it.
/// dx and dy store the amount of pixels that are drawn per width / height and are updated whenever draw() is called
pub struct Defence {
	hp: u32,
	width: f64, height: f64, dx: f64, dy: f64,
	shop: shop::Shop,
	background: Texture<Resources>,
	shortest_path_map: JkmShortestPathMap,
	towers: Vec<Box<Tower>>,
	tower_templates: [Box<Tower>;NUMBER_OF_TOWERS],
	tower_sprites: Vec<Texture<Resources>>,
	enemies: Vec<Box<Enemy>>,
	enemy_sprites: Vec<Texture<Resources>>,
	//projectile datastructures and sprites
}

impl Defence {
	/// width and height are not related to the actual drawn size, they only define the size of the battle field, i.e. how many objects can fit on it.
	pub fn new (w: &PistonWindow, hp: u32, width: f64, height: f64) -> Defence {
		
		let mut spm = JkmShortestPathMap::new( (0.0, 0.0), (0.0, height - STD_ENEMY_H ),(-(width / 2.0), 0.0, width, height) );
		spm.add_map_border();
		
		let img = find_folder::Search::ParentsThenKids(3, 3).for_folder("img").unwrap();
		let folder = img.join("rainbow_road_from_hell.png");
		let background = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let mut enemies: Vec<Box<Enemy>> = Vec::new();
		enemies.push( Box::new(enemy::basic_enemy::BasicEnemy::new()) );
		
		let towers = Vec::new();
		
		// load tower sprites
		let mut tower_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = TOWER_SPRITE_LIST;
		let tower_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("towers").unwrap();
		for s in sprite_names.iter() {
			let f = tower_folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			tower_sprites.push(sprite);
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
		
		//create tower templates
		let t_temp :[Box<Tower>;NUMBER_OF_TOWERS] = [
			Box::new(basic_tower::BasicTower::new(0.0, 0.0)),
			Box::new(aoe_tower::AoeTower::new(0.0, 0.0)),
		];
		
		Defence {
			hp: hp,
			width: width, height: height, dx: 1.0, dy: 1.0,
			shop: shop::Shop::new(),
			background: background,
			shortest_path_map: spm,
			towers: towers,
			tower_templates: t_temp,
			tower_sprites: tower_sprites,
			enemies: enemies,
			enemy_sprites: enemy_sprites,
		}
	}
	pub fn on_update(&mut self, upd: UpdateArgs) {
		// enemies
		for e in self.enemies.iter_mut() {
			e.update(upd.dt, &self.shortest_path_map);
		}
	}
	pub fn on_click(&mut self, x: f64, y: f64) -> Option<DefenceUserInteraction> {
		if let Some(DefenceUserInteraction::BuyTower{x: w, y: h, tower_id}) = self.shop.on_click(x, y - BF_SHOP_SPLIT_RATIO * self.height * self.dy) {
			if x > 0.0 && x < (self.width  - w)* self.dx && y > 0.0 && y < (self.height - h)* self.dy * BF_SHOP_SPLIT_RATIO {	
				if !collision::towers_with_rectangle(&self.towers, x/self.dx, y/self.dy, w, h) {
					return Some(
						DefenceUserInteraction::BuyTower
							{
								x: x/self.dx, 
								y: y/self.dy,
								tower_id: tower_id,
							}
					);
				}
			}
		}
		None
	}
	
}

impl Drawable for Defence {
	#[allow(unused_variables)]
	fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, mouse: [f64;2]) -> Option<DrawRequest> {
		
		let dx = w / self.width;
		let dy = h / self.height;
		
		self.dx = dx;
		self.dy = dy;
		
		let battlefield_h = h * BF_SHOP_SPLIT_RATIO;
		
		// background <=> battlefield
		let (sprite_w, sprite_h) = self.background.get_size();
		let x_scale = w/(sprite_w as f64);
		let y_scale = battlefield_h/(sprite_h as f64);
		image(&self.background, view.scale(x_scale, y_scale), g);	
		
		// towers
		for t in self.towers.iter() {
			t.draw(g, view, dx, dy, &self.tower_sprites);
		}
		
		
		// enemies
		let enemy_view = view.trans(w/2.0, 0.0);
		for e in self.enemies.iter() {
			e.draw(g, enemy_view, dx, dy, &self.enemy_sprites);
		}
		
		// projectiles
		
		// shop
		let draw_req = self.shop.draw(g, view.trans(0.0, battlefield_h), w, h - battlefield_h, [mouse[0], mouse[1] - battlefield_h], &self.tower_sprites, dx, dy);
		match draw_req{
			Some(DrawRequest::DrawTower{tower_id}) => {
				self.tower_templates[tower_id].draw(g, view.trans(mouse[0],mouse[1]), dx, dy, &self.tower_sprites);
			}
			_ => {}
		}
		draw_req
		
	}
}

// 
impl Defence {
	pub fn build_tower(&mut self, x: f64, y: f64, tower_id: usize) {
		let new_tower : Box<Tower> =
		match tower_id {
			BASIC_TID => Box::new(tower::basic_tower::BasicTower::new(x, y)),
			AOE_TID => Box::new(tower::aoe_tower::AoeTower::new(x, y)),
			_ => panic!("Unexpected tower ID: {}", tower_id),
		};
		let (w,h) = new_tower.get_tower_size();
		self.shortest_path_map.insert_obstacle(x-STD_ENEMY_W, y-STD_ENEMY_H, w, h );
		self.towers.push(new_tower);
		
		for e in self.enemies.iter_mut() {
			e.refresh_destination(&self.shortest_path_map);
		}
		
	}
}