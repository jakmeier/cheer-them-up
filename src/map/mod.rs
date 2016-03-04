/*
<<< map >>
*/

//! Manages the map which is one step above the micro. With only the resources one can own in the micro games, the map is fully playable.

mod land;

use self::land::Land;
use utils::ClickableRectangle;
use DrawRequest;

use super::Drawable;
use super::piston_window::*;
use super::gfx_device_gl::Resources;
use super::gfx_device_gl::command::CommandBuffer;
use super::gfx_graphics::GfxGraphics;
use super::find_folder;
use rand;

// used for initialization of the land sizes, but it should be overwritten before drawing anyway
const STD_SIZE: f64 = 100.0;
/// Used by the land module to borrow the sprite array
pub const LAND_SPRITES_COUNT: usize = 14;
/// Used by the land module to borrow the sprite array
pub const BUTTON_SPRITES_COUNT: usize = 6;

/// Used to get messages from a land through the map up to the root of the project.
pub enum MapUserInteraction{
	BuyLand{index: u32, price: u32},
	SellLand{index: u32, price: u32},
	ConcreteLand{index: u32},
	BuildIronFactory{index: u32},
	UpgradeIronFactory{index: u32},
	AddResources{coins: u32, wood: u32, iron: u32, crystals: u32},
}

/// The main data structure for the map.
pub struct Map {
	cols: usize, rows: usize,
	land_matrix: Vec<Land>,
	land_sprites: [Texture<Resources>; LAND_SPRITES_COUNT ],
	button_sprites: [Texture<Resources>; BUTTON_SPRITES_COUNT ],
	font: Glyphs,
}


impl Map{
	pub fn new(w: &PistonWindow, cols: usize, rows: usize) -> Map {
		
		let mut lands: Vec<Land> = Vec::new();
		
		let img = find_folder::Search::ParentsThenKids(3, 3).for_folder("img").unwrap();
		let folder = img.join("grass.png");
		let grass = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let tree_img = find_folder::Search::ParentsThenKids(3, 3).for_folder("trees").unwrap();
		let folder = tree_img.join("tree_a_empty.png");
		let tree_a_0 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_a_almost_empty.png");
		let tree_a_1 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_a_almost_full.png");
		let tree_a_2 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_a_full.png");
		let tree_a_3 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_b_empty.png");
		let tree_b_0 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_b_almost_empty.png");
		let tree_b_1 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_b_almost_full.png");
		let tree_b_2 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = tree_img.join("tree_b_full.png");
		let tree_b_3 = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("industry.png");
		let industry = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("add_coins.png");
		let add_coins = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("add_wood.png");
		let add_wood = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("add_iron.png");
		let add_iron = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("add_crystal.png");
		let add_crystal = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let land_sprite_array = [grass, tree_a_0, tree_a_1, tree_a_2, tree_a_3, tree_b_0, tree_b_1, tree_b_2, tree_b_3, industry, 
								add_coins, add_wood, add_iron, add_crystal];
		
		/*		Button Sprites
		
		*/
		
		
		let folder = img.join("buy.png");
		let buy = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("sell.png");
		let sell = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("concrete.png");
		let concrete = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("crane.png");
		let build_iron_factory = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("axe.png");
		let axe = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("up.png");
		let upgrade = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let button_sprite_array = [buy, sell, concrete, build_iron_factory, axe, upgrade];
		
		// Font
		
		let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
		let ref font = assets.join("FiraSans-Regular.ttf");
		let factory = w.factory.borrow().clone();
		let glyphs = Glyphs::new(font, factory).unwrap();
		
		for i in 0..cols {
			for j in 0..rows {
				let rn = rand::random::<u32>();
				lands.push( Land::new( i as f64 * STD_SIZE, j as f64 * STD_SIZE, STD_SIZE, STD_SIZE, 2 + (rn % 5) ) );
			}
		}
		
		Map{
			cols: cols, rows: rows,
			land_matrix: lands,
			land_sprites: land_sprite_array,
			button_sprites: button_sprite_array,
			font: glyphs,
		}
	}
	
	/// Must be called in the update loop.
	/// Returns the produced resources. Only whole numbers can be produced, fractions are stored within the map. To be specific, the vlaues are saved in the land structures.
	pub fn on_update(&mut self, upd: UpdateArgs) -> [u32; 4] {
		let mut iron_produced = 0;
		let rn = rand::random::<u32>();
		for mut l in self.land_matrix.iter_mut() {
			match l.update(upd.dt, rn) {
				Some(MapUserInteraction::AddResources{iron, ..}) => { iron_produced += iron}
				_=> {}
			}
		}
		[0,0,iron_produced,0]
	}
	/// Checks if the current click was on clickable object in the map. 
	/// Can return a MapUserInteraction to interact with the root of the game.
	pub fn on_click(&mut self, x: f64, y: f64) -> Option<MapUserInteraction> {
		//check if the click is on a currently shown button_sprite_array
		for i in 0..self.rows {
			for j in 0..self.cols {
				match self.land_matrix[i * self.cols + j].click_buttons(x,y) {
					Some(MapUserInteraction::BuyLand{price, ..}) => {	return Some(MapUserInteraction::BuyLand{index: ((i as usize * self.cols + j)as u32), price: price}); }
					Some(MapUserInteraction::SellLand{price, ..}) => { return Some(MapUserInteraction::SellLand{index: ((i as usize * self.cols + j)as u32), price: price}); }
					Some(MapUserInteraction::ConcreteLand{..}) => {return Some(MapUserInteraction::ConcreteLand{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildIronFactory{..}) => {return Some(MapUserInteraction::BuildIronFactory{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::UpgradeIronFactory{..}) => {return Some(MapUserInteraction::UpgradeIronFactory{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::AddResources{coins, wood, iron, crystals}) => {return Some(MapUserInteraction::AddResources{coins:coins, wood:wood, iron:iron, crystals:crystals}); }
					_=> {}
				}
			}
		}
		//otherwise click on land
		for i in 0..self.rows {
			for j in 0..self.cols {
				self.land_matrix[i * self.cols + j].click(x,y);
			}
		}
		None
	}

}

impl Drawable for Map {
	#[allow(unused_variables)]
	fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, mouse: [f64;2]) -> Option<DrawRequest> {
		let mut result = None;
		
		let width = w / self.cols as f64;
		let height = h / self.rows as f64;
		
		//lands
		for i in 0..self.rows {
			for j in 0..self.cols {
				self.land_matrix[i * self.cols + j].set_coordinates( j as f64 * width,  i as f64 * height, width, height); //ineffiecient to do in each iteration
				self.land_matrix[i * self.cols + j].draw(g, view, &(self.land_sprites));
			}
		}
		
		//grid
		let color = [0.0, 0.0, 0.0, 0.5];
		for i in 0..self.cols+1 {
			line(color, 1.5, [i as f64 * width, 0.0, i as f64 * width, h as f64 ], view, g);
		}
		for i in 0..self.rows+1 {
			line(color, 1.5, [0.0, i as f64 * height, w as f64, i as f64 * height ], view, g);
		}
		
		//buttons
		for i in 0..self.rows {
			for j in 0..self.cols {
				if let Some (req) = self.land_matrix[i * self.cols + j].draw_buttons(g, view, &(self.button_sprites), mouse)
				 { result = Some(req); }
				
			}
		}
		result
	}	
}

impl Map { // interface for game state updates
	pub fn buy_land(&mut self, index: u32){
		self.land_matrix[index as usize].buy();
	}
	pub fn sell_land(&mut self, index: u32) -> bool {
		self.land_matrix[index as usize].sell()
	}
	pub fn concrete_land(&mut self, index: u32) {
		self.land_matrix[index as usize].concrete();
	}
	pub fn build_iron_factory_on_land(&mut self, index: u32) -> bool {
		self.land_matrix[index as usize].build_iron_factory()
	}
	pub fn upgrade_iron_factory(&mut self, index: u32) {
		self.land_matrix[index as usize].upgrade_iron_factory();
	}
}

