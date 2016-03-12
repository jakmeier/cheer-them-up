/*
<<< cash >>
	- keep track of available resources (cash) and show it on the screen
*/
//! Keeps track of the resources the player currently owns. Use take_ and add_ functions to interact with the module.
//! Can also display the resources as a header bar. 
//! Additional functionalities: So far only drawing a price on a location on the screen

use super::Drawable;
use super::piston_window::*;
use super::gfx_device_gl::Resources;
use super::gfx_device_gl::command::CommandBuffer;
use super::gfx_graphics::GfxGraphics;
use super::find_folder;

use DrawRequest;

pub struct CashHeader{
	coins: u32, wood: u32, iron: u32, crystals: u32,
	coins_sprite: Texture<Resources>, wood_sprite: Texture<Resources>, iron_sprite: Texture<Resources>, crystal_sprite: Texture<Resources>,
	font: Glyphs,
}

impl CashHeader{
	pub fn new(w: &PistonWindow) -> CashHeader{	
		let img_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("img").unwrap();
		let img = img_folder.join("coins.png");
		let coin_img = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&img,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let img = img_folder.join("wood.png");
		let wood_img = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&img,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let img = img_folder.join("iron.png");
		let iron_img = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&img,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let img = img_folder.join("crystal.png");
		let crystal_img = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&img,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		
		let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
		let ref font = assets.join("FiraSans-Regular.ttf");
		let factory = w.factory.borrow().clone();
		let glyphs = Glyphs::new(font, factory).unwrap();
		
		CashHeader{
			coins: 10, wood: 10, iron: 10, crystals: 10,
			coins_sprite: coin_img, wood_sprite: wood_img, iron_sprite: iron_img, crystal_sprite: crystal_img,
			font: glyphs
		}
	}

	///pays the given amount of resources if available, otherwise does nothing and returns false
	pub fn test_and_pay(&mut self, res: [u32;4]) ->  bool {
		if self.coins >= res[0] && self.wood >= res[1] && self.iron >= res[2] && self.crystals >= res[3] {
			self.coins -= res[0];
			self.wood -= res[1];
			self.iron -= res[2];
			self.crystals -= res[3];
			true
		}
		else { false }
	}
	
	pub fn add_resources (&mut self, res: [u32;4]){
		self.coins += res[0];
		self.wood += res[1];
		self.iron += res[2];
		self.crystals += res[3];
	}
	
	pub fn add_coins(&mut self, a: u32){
		self.coins += a;
	}
	pub fn take_coins(&mut self, a: u32) -> bool {
		if self.coins >= a {
			self.coins -= a;
			true
		}
		else {false}
	}
	
	pub fn add_wood(&mut self, a: u32){
		self.wood += a;
	}
	pub fn take_wood(&mut self, a: u32) -> bool {
		if self.wood >= a {
			self.wood -= a;
			true
		}
		else {false}
	}
	
	pub fn add_iron(&mut self, a: u32){
		self.iron += a;
	}
	pub fn take_iron(&mut self, a: u32) -> bool {
		if self.iron >= a {
			self.iron -= a;
			true
		}
		else {false}
	}
	
	pub fn add_crystals(&mut self, a: u32){
		self.crystals += a;
	}
	pub fn take_crystals(&mut self, a: u32) -> bool {
		if self.crystals >= a {
			self.crystals -= a;
			true
		}
		else {false}
	}

	/// Draws the given price to the given location.
	/// Font turns red when there are not enough resources
	pub fn draw_resource_price (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, res: [u32;4],font_size: u32) {
		let mut x  = 0.0;
		let mut color :[f32;4];
		let sprites = [&self.coins_sprite, &self.wood_sprite, &self.iron_sprite, &self.crystal_sprite ];
		let enough = [self.coins >= res[0], self.wood >= res[1], self.iron >= res[2], self.crystals >= res[3]];
		
		for i in 0..4 {
			if res[i] > 0 {
				if enough[i] { color =  [1.0, 1.0, 1.0, 1.0];}
				else { color = [1.0, 0.0, 0.0, 1.0]; }
				text::Text::new_color(color, font_size as u32).draw( &(res[i].to_string()), &mut self.font, &draw_state, view.trans(x, 0.0 ), g);
				x += font_size as f64;
				
				let (sprite_w, sprite_h) = (*sprites[i]).get_size();
				let scale = (font_size as f64) / (sprite_h as f64);
				image(sprites[i], view.trans(x, -(sprite_h as f64* scale * 0.9)).scale(scale,scale), g);
				x += sprite_w as f64 * scale * 1.2;
			}
		}
	}
}

impl Drawable for CashHeader {
	#[allow(unused_variables)] //mouse
	fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, mouse: [f64;2]) -> Option<DrawRequest> { 
		//Background, black
		let color = [0.0, 0.0, 0.0, 1.0];
		rectangle(color, [0.0, 0.0, w, h], view, g);
		
		//resources
		
		//prepare sprite names
		let sprites = [&self.coins_sprite, &self.wood_sprite, &self.iron_sprite, &self.crystal_sprite ];
		let resources = [self.coins, self.wood, self.iron, self.crystals];
		
		let mut x = 0.02 * w;
		let y = 0.15 * h;
		
		for i in 0..4 {
			//symbols
			
			let (sprite_w, sprite_h) = (*sprites[i]).get_size();
			let scale = (h* 0.7) / (sprite_h as f64); 
			image(sprites[i], view.trans(x, y).scale(scale,scale), g);
			
			//numbers
			let font_size = sprite_h as f64 * scale;
			x += sprite_w as f64 * scale as f64 + 0.02 * w as f64;
			
			text::Text::new_color([1.0, 1.0, 1.0, 1.0], (font_size * 1.5) as u32).draw(
						& resources[i].to_string(), 
						&mut self.font, 
						&draw_state, 
						view.trans(x, h - y), g
					);
			x += 0.1 * w;
		}
		None
	}
}