//! Traits, Structs, Enums that don't belong to a specific module are defined in here.

use std::fs::File;
use std::io::prelude::*;

use piston_window::*;
use gfx_device_gl;
use gfx_graphics;
use constants::*;

/// Message used when a module needs to draw something and it lacks information from another module.
pub enum DrawRequest{
	/// Draws at mouse position.
	ResourcePrice{price: [u32;4], coordinates: math::Matrix2d, font_size:u32},
	/// Draws at mouse position.
	DrawTower{tower_id: usize},
	/// Draws at the standard location for tooltips
	Tooltip{text: String},
}

/// Must be implemened by all mini games
/// Draws the object within the given area. Scaling is managed internally.
pub trait Drawable {
	fn draw(&mut self, g: &mut gfx_graphics::GfxGraphics<gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>,
		view: math::Matrix2d,
		draw_state: DrawState,
		w: f64, h:f64,
		)
		;
}

/// Used by the map module to get messages from a land through the map up to the root of the project.
pub enum MapUserInteraction{
	BuyLand{index: u32, price: u32},
	SellLand{index: u32, price: u32},
	ConcreteLand{index: u32},
	BuildIronFactory{index: u32},
	UpgradeIronFactory{index: u32, level: u32},
	UpgradeBank{index: u32, level: u32},
	AddResources{coins: u32, wood: u32, iron: u32, crystals: u32},
	BuildUniversity {index: u32},
	UpgradeUniversity{index: u32, level: u32},
	BuildOracle {index: u32},
	BuildBlacksmith {index: u32},
	BuildBank {index: u32},
	Industrialise,
	ResearchEconomy,
	ResearchTower{index: usize}, 
	UpgradeGold, UpgradeIron, UpgradeCrystal,
	UpgradeTower{ tid: usize, kind: TowerAttribute, level: u32 },
	BuildBlacksmithII {index: u32}, 
	BuildBarracks {index: u32}, 
	BuildArcheryRange {index: u32},
	UpgradeSurprise, UpgradeCandy,
}

/// Used by the defence module to communicate with the root of the project. Mostly to request constructions and upgrades that are only allowed if there are enough resources.
pub enum DefenceUserInteraction{
	BuyTower{x: f64, y:f64, tower_id: usize},
}

/// Used for tower upgrades to distinguish between differnt upgrades that ate handled the same way in the code.
#[derive(Clone)]
pub enum TowerAttribute {
	Attack,
	Defence,
	Range,
}

/// Stores information about which meta upgrades have been bought already and which have not
pub struct GameState {
	// Towers (from any buildings)
	pub tower_researched: [bool; NUMBER_OF_TOWERS],
	// University
	pub industrialisation: bool, pub economy: bool, pub chocolate: bool, pub cotton_candy: bool,
	// Oracle
	pub gold_upgrade: u8, pub crystal_upgrade: u8, pub iron_upgrade: u8, 
	// Tower upgrades
	pub tower_upgrades: [[u8;3]; NUMBER_OF_TOWERS],
}
impl GameState{
	pub fn new() -> GameState {
		GameState{
			tower_researched: [false; NUMBER_OF_TOWERS],
			chocolate: false, cotton_candy: false, industrialisation: false, economy: false,
			 gold_upgrade: 0, crystal_upgrade: 0, iron_upgrade: 0, 
			 tower_upgrades: [[0,0,0]; NUMBER_OF_TOWERS],
		}
	}
}

/// Used to save all kinds of statistics. With this statistics, a score can be computet by this struct.
pub struct Statistics {
	units_killed: u32,
	boss_kills: Vec<(u32,u32)>, // stores number of kills and the value of each different boss type killed
	wins_game_one: u32,
	wins_game_two: u32, 
	resources_produced: [u32;4],
}

impl Statistics {
	pub fn new() -> Statistics {
		Statistics {
			units_killed: 0,
			boss_kills: Vec::new(),
			wins_game_one: 0,
			wins_game_two: 0, 
			resources_produced: [0;4],
		}
	}
	pub fn add_win_game_one(&mut self) { self.wins_game_one += 1; }
	pub fn add_win_game_two(&mut self) { self.wins_game_two += 1; }
	pub fn add_unit_kill(&mut self, size: u32) { 
		if size == 1 {self.units_killed += 1;}
		else {
			let mut done = false;
			for &mut(ref mut count, value) in self.boss_kills.iter_mut() {
				if size == value { *count += 1; done = true; break; }
			}
			if !done { self.boss_kills.push((1, size)); }
		}
	}
	pub fn resources_produced(&mut self, res: [u32;4]) { 
		for i in 0..4 {
			self.resources_produced[i] += res[i]; 
		}
	}
	
	pub fn get_score(&self) -> u32 {
		let mut score = self.units_killed * 20;
		for &(n, v) in self.boss_kills.iter() {
			score += n * v * 20;
		}
		score += self.wins_game_one * 5;
		score += self.wins_game_two * 5;
		for i in 0..4 {
			score += self.resources_produced[i]; 
		}
		score
	} 
}

/// Stores values loaded from the config.txt file and makes  them available to the app
pub struct Settings {
	name: String,
	language: String,
	screen_width: u32, screen_height: u32,
	general_scaling_factor: f64, battlefield_scaling_factor: f64,
	std_font_size: u32, title_font_size: u32, 
}

// constructors
impl Settings {
	/// creates a new Settings object with the standar values:
	/**
	* language: english
	* screen_width: 960,
	* screen_height: 590,
	* general_scaling_factor: 1.0, 
	* battlefield_scaling_factor: 1.0,
	* std_font_size: 20, 
	* title_font_size: 60,
	**/
	pub fn new() -> Settings {
		Settings {
			name: String::from("guest"),
			language: String::from("en"),
			screen_width: 960,
			screen_height: 590,
			general_scaling_factor: 1.0, battlefield_scaling_factor: 1.0,
			std_font_size: 20, title_font_size: 60,
		}
	}
	
	pub fn from_file(path: &str) -> Settings {
		let mut result = Settings::new();
		match File::open(&path) {
			Err(e) => {
				println!("Configuration file could not be found.");
				println!("{}", e);
				println!("The standard configuration will be used.");
			},
			Ok(mut file) => {
				let mut s = "".to_string();
				match file.read_to_string(&mut s) {
					Err(e) => {
						println!("Cannot read configuration file.");
						println!("{}", e);
						println!("The standard configuration will be used.");
						
					},
					Ok(_) => {
						let mut key = "".to_string();
						let mut value = "".to_string();
						let mut buf = s.chars();
						while let Some(mut c) = buf.next() {
							if c == '#' {
								if let Some(just_c) = buf.next() {c = just_c} else { println!("Configuration file ended unexpectedly."); break; }
								while c != ']' {
									key.push(c);
									if let Some(just_c) = buf.next() {c = just_c} else { println!("Configuration file ended unexpectedly."); break; }
								}
								if c != ']' { println!("Corrupted configuration file, unexpected char near {}. Expected ] , found {}. ", key, c); }
								if let Some(just_c) = buf.next() {c = just_c} else { println!("Configuration file ended unexpectedly."); break; }
								if c != '[' { println!("Corrupted configuration file, unexpected char near {}. Expected [ , found {}. ", key, c); }
								else {
									if let Some(just_c) = buf.next() {c = just_c} else { println!("Configuration file ended unexpectedly."); break; }
									while c != ']' {
										value.push(c);
										if let Some(just_c) = buf.next() {c = just_c} else { println!("Configuration file ended unexpectedly."); break; }
									}
									if key == "name" {result.name = value}
									else if key == "000" {result.language = value;}
									else {
										let v : f64 = if let Ok(v) = value.parse::<f64>(){v}
												else if let Ok(v) = value.parse::<u32>(){v as f64}
												else { println!("File corrupted: No valid value for key {}.", key); break;};
										match &key[..] {
											"001" => result.screen_width = v as u32,
											"002" => result.screen_height = v as u32,
											"003" => result.general_scaling_factor = v,
											"004" => result.battlefield_scaling_factor = v,
											"005" => result.std_font_size = v as u32,
											"006" => result.title_font_size = v as u32,
											_ => println!("Corrupted configuration file, unexpected key: {}. Value was {}. ", key, v)
										}
									}
									key = "".to_string();
									value = "".to_string();
								}
							}
						}
					}
				}
			}
		}
		result
	}
}

// getter and setter
impl Settings {
	pub fn get_name(&self) -> String {
		self.name.clone()
	}
	pub fn set_screen_dimensions(&mut self, wh: (u32,u32)) {
		self.screen_width = wh.0;
		self.screen_height = wh.1;
	}
	pub fn get_screen_dimensions(&self) -> (u32,u32) {
		(self.screen_width, self.screen_height)
	}
	
	pub fn set_general_scaling_factor(&mut self, q: f64) {
		self.general_scaling_factor = q;
	}
	pub fn get_general_scaling_factor(&self) -> f64 {
		self.general_scaling_factor
	}
	
	pub fn set_battlefield_scaling_factor(&mut self, q: f64) {
		self.battlefield_scaling_factor = q;
	}
	pub fn get_battlefield_scaling_factor(&self) -> f64{
		self.battlefield_scaling_factor
	}
	
	pub fn set_base_std_font_size(&mut self, s: u32) {
		self.std_font_size = s;
	}
	pub fn get_std_font_size(&self) -> u32 {
		(self.std_font_size as f64 * self.general_scaling_factor) as u32
	}

	pub fn set_base_title_font_size(&mut self, s: u32) {
		self.title_font_size = s;
	}	
	pub fn get_title_font_size(&self) -> u32 {
		(self.title_font_size as f64 * self.general_scaling_factor) as u32
	}
	
	pub fn get_language(&self) -> String {
		self.language.clone()
	}
	
	pub fn set_language(&mut self, s: String) {
		self.language = s;
	}
}