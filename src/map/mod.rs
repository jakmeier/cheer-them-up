/*
<<< map >>
*/

//! Manages the map which is one step above the micro. With only the resources one can own in the micro games, the map is fully playable.

mod land;

use self::land::Land;
use utils::ClickableRectangle;
use definitions::{MapUserInteraction, DrawRequest, Drawable, GameState};

use super::piston_window::*;
use super::gfx_device_gl::Resources;
use super::gfx_device_gl::command::CommandBuffer;
use super::gfx_graphics::GfxGraphics;
use super::find_folder;
use rand;

// used for initialization of the land sizes, but it should be overwritten before drawing anyway
const STD_SIZE: f64 = 100.0;

/// The main data structure for the map.
pub struct Map {
	cols: usize, rows: usize,
	pub land_matrix: Vec<Land>,
//	land_sprites: [Texture<Resources>; LAND_SPRITES_COUNT ],
	land_sprites: Vec<Texture<Resources>>,
//	button_sprites: [Texture<Resources>; BUTTON_SPRITES_COUNT ],
	button_sprites: Vec<Texture<Resources>>,
//	font: Glyphs,
}


impl Map{
	pub fn new(w: &PistonWindow, cols: usize, rows: usize) -> Map {
		
		let mut lands: Vec<Land> = Vec::new();
		
		
		// Land sprites
		let mut land_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = ["grass.png", 
							"tree_a_empty.png", "tree_a_almost_empty.png", "tree_a_almost_full.png", "tree_a_full.png", 
							"tree_b_empty.png", "tree_b_almost_empty.png", "tree_b_almost_full.png", "tree_b_full.png",
							"industry.png", "add_coins.png", "add_wood.png", "add_iron.png", "add_crystal.png", 
							"university_i.png", "university_ii.png", "university_iii.png", "university_iv.png",
							"blacksmith.png", "bank.png", "magic_stones.png",
							"blacksmith_ii.png", "barracks.png", "archery_range.png",	
							] ;
		let folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("map").unwrap();
		for s in sprite_names.iter() {
			let f = folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			land_sprites.push(sprite);
		}
		
		//		Button Sprites
		
		let mut button_sprites: Vec<Texture<Resources>> = Vec::new();
		let sprite_names = [
							"buy.png", "sell.png",
							"concrete.png", 
							"crane.png", 
							"axe.png", 
							"up.png", //5
							"cap.png",
							"cog.png",
							"hammer.png",
							"dream_catcher.png",
							"money_bag.png", //10
							"research_aoe.png",
							"research_wall.png",
							"libra.png",
							"gold_upgrade.png",
							"iron_upgrade.png", //15
							"crystal_upgrade.png",
							"tu_attack.png",
							"tu_defence.png",
							"tu_range.png",
							"tu_basic.png", //20
							"tu_aoe.png",
							"tu_wall.png",
							"build_blacksmith_ii.png",
							"build_barracks.png",
							"build_archery_range.png", //25
							"candy.png",
							"chocolate.png",
							] ;
		let folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("button").unwrap();
		for s in sprite_names.iter() {
			let f = folder.join(s);
			let sprite = Texture::from_path( &mut *w.factory.borrow_mut(), &f, Flip::None, &TextureSettings::new()).unwrap();
			button_sprites.push(sprite);
		}
	
		// Font
		
		//let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
		//let ref font = assets.join("FiraSans-Regular.ttf");
		//let factory = w.factory.borrow().clone();
		//let glyphs = Glyphs::new(font, factory).unwrap();
		
		let fake_init_state = GameState::new();
		for i in 0..cols {
			for j in 0..rows {
				let rn = rand::random::<u32>();
				lands.push( Land::new( i as f64 * STD_SIZE, j as f64 * STD_SIZE, STD_SIZE, STD_SIZE, 2 + (rn % 5), &fake_init_state ) );
			}
		}
		
		Map{
			cols: cols, rows: rows,
			land_matrix: lands,
			land_sprites: land_sprites,
			button_sprites: button_sprites,
			//font: glyphs,
		}
	}
	
	/// Must be called in the update loop.
	/// Returns the produced resources. Only whole numbers can be produced, fractions are stored within the map. To be specific, the vlaues are saved in the land structures.
	pub fn on_update(&mut self, upd: UpdateArgs, state: &GameState) -> [u32; 4] {
		let mut coins_produced = 0;
		let mut wood_produced = 0;
		let mut iron_produced = 0;
		let mut crystals_produced = 0;
		let rn = rand::random::<u32>();
		for mut l in self.land_matrix.iter_mut() {
			match l.update(upd.dt, rn, state) {
				Some(MapUserInteraction::AddResources{coins, wood, iron, crystals}) => {
					iron_produced += iron;
					coins_produced += coins;
					wood_produced += wood;
					crystals_produced += crystals;
				}
				_=> {}
			}
		}
		[coins_produced, wood_produced, iron_produced, crystals_produced]
	}
	/// Checks if the current click was on clickable object in the map. 
	/// Coordinates are relative to the map.
	/// Can return a MapUserInteraction to interact with the root of the game.
	pub fn on_click(&mut self, x: f64, y: f64, state: &GameState) -> Option<MapUserInteraction> {
		//check if the click is on a currently shown button_sprite_array
		for i in 0..self.rows {
			for j in 0..self.cols {
				match self.land_matrix[i * self.cols + j].click_buttons(x,y, state) {
					Some(MapUserInteraction::BuyLand{price, ..}) => {	return Some(MapUserInteraction::BuyLand{index: ((i as usize * self.cols + j)as u32), price: price}); }
					Some(MapUserInteraction::SellLand{price, ..}) => { return Some(MapUserInteraction::SellLand{index: ((i as usize * self.cols + j)as u32), price: price}); }
					Some(MapUserInteraction::ConcreteLand{..}) => {return Some(MapUserInteraction::ConcreteLand{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildIronFactory{..}) => {return Some(MapUserInteraction::BuildIronFactory{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::UpgradeIronFactory{level, ..}) => {return Some(MapUserInteraction::UpgradeIronFactory{index:((i as usize * self.cols + j)as u32), level: level}); }
					Some(MapUserInteraction::UpgradeBank{level, ..}) => {return Some(MapUserInteraction::UpgradeBank{index:((i as usize * self.cols + j)as u32), level: level}); }
					Some(MapUserInteraction::AddResources{coins, wood, iron, crystals}) => {return Some(MapUserInteraction::AddResources{coins:coins, wood:wood, iron:iron, crystals:crystals}); }
					Some(MapUserInteraction::BuildUniversity{..}) => {return Some(MapUserInteraction::BuildUniversity{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::UpgradeUniversity{level, ..}) => {return Some(MapUserInteraction::UpgradeUniversity{index:((i as usize * self.cols + j)as u32), level: level}); }
					Some(MapUserInteraction::BuildOracle{..}) => {return Some(MapUserInteraction::BuildOracle{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildBlacksmith{..}) => {return Some(MapUserInteraction::BuildBlacksmith{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildBank{..}) => {return Some(MapUserInteraction::BuildBank{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildBlacksmithII{..}) => {return Some(MapUserInteraction::BuildBlacksmithII{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildBarracks{..}) => {return Some(MapUserInteraction::BuildBarracks{index:((i as usize * self.cols + j)as u32)}); }
					Some(MapUserInteraction::BuildArcheryRange{..}) => {return Some(MapUserInteraction::BuildArcheryRange{index:((i as usize * self.cols + j)as u32)}); }
					Some(upgrade) => { return Some(upgrade); }
					None => {}
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

	pub fn update_all_buttons(&mut self, upgrades: &GameState) {
		for land in self.land_matrix.iter_mut() {
			land.refresh_buttons(upgrades);
		}
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

