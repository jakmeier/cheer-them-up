//! Traits, Structs, Enums that don't belong toa specific module are defined in here.

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
}

/// Draws the object within the given area. Scaling is managed internally.
pub trait Drawable {
	fn draw(&mut self, g: &mut gfx_graphics::GfxGraphics<gfx_device_gl::Resources, gfx_device_gl::command::CommandBuffer>,
		view: math::Matrix2d,
		draw_state: DrawState,
		w: f64, h:f64,
		mouse: [f64;2],
		)
		-> Option<DrawRequest>
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
}

/// Used by the defence module to communicate with the root of the project. Mostly to request constructions and upgrades that are only allowed if there are enough resources.
pub enum DefenceUserInteraction{
	BuyTower{x: f64, y:f64, tower_id: usize},
}

/// Stores information about which meta upgrades have been bought already and which have not
pub struct GameState {
	// Towers (from any buildings)
	pub tower_researched: [bool; NUMBER_OF_TOWERS],
	// University
	pub industrialisation: bool, pub economy: bool, pub chocolate: bool, pub cotton_candy: bool,
	// Oracle
	pub gold_upgrade: u8, pub crystal_upgrade: u8, pub iron_upgrade: u8, 
}
impl GameState{
	pub fn new() -> GameState {
		GameState{
			tower_researched: [false; NUMBER_OF_TOWERS],
			chocolate: false, cotton_candy: false, industrialisation: false, economy: false,
			 gold_upgrade: 0, crystal_upgrade: 0, iron_upgrade: 0, 
		}
	}
}


