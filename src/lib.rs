/*
<<< multy_task >>
*/

pub mod micro;
pub mod cash;
pub mod map;
pub mod utils;

extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate find_folder;
extern crate freetype;
extern crate rand;

use micro::{PersistentWinnerState, AbsolutelyChangeableState, AI};
use map::MapUserInteraction;
use piston_window::*;

pub const CONCRETE_PRICE: u32 = 3;
pub const IRON_FACTORY_PRICE: u32 = 5;
pub const IRON_FACTORY_UPGRADE_PRICE: u32 = 1;

pub enum DrawRequest{
	ResourcePrice{price: [u32;4], coordinates: math::Matrix2d, font_size:u32},
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

/// Root structure for the game. It contains all the mini games (micro) as well as the higher level game parts (macro) and connects them.
pub struct Game {
		screen_width: f64, screen_height: f64,
		mouse_x: f64, mouse_y: f64, 
		map: map::Map, map_coordinates: [f64; 4],
		mini_game: micro::rock_paper_scissors::GameObj,
		clock: f64,
		coin_paid: bool,
		cash: cash::CashHeader,
	}
	
	impl Game {
		pub fn new(w: &PistonWindow) -> Game {
			
			let screen_width = w.size().width as f64;
			let screen_height = w.size().height as f64;
			let header_height = screen_height * 0.05;
			let game_height = screen_height - header_height;
			let x_seperation = screen_width * 0.6;
			
			Game{	
				screen_width: screen_width,	screen_height: screen_height,
				mouse_x: 0.0, mouse_y: 0.0, 
				map: map::Map::new(w, 7, 10), map_coordinates: [x_seperation, header_height, screen_width - x_seperation, game_height],
				mini_game: micro::rock_paper_scissors::GameObj::new(w),
				clock: 0.0,
				coin_paid: false,
				cash: cash::CashHeader::new(w),
			}
		}
		pub fn on_update(&mut self, upd: UpdateArgs) {
			self.clock += upd.dt;
			let mini_game_one_timer = 4-(self.clock as u8 % 5);
			self.mini_game.set_time(mini_game_one_timer);
			if mini_game_one_timer == 0 {
				if self.coin_paid {self.mini_game.make_turn(); } //ai
				self.mini_game.lock_input(true);
				if self.coin_paid {self.mini_game.save_turn();} //ai
				self.coin_paid = false;
				self.mini_game.set_visibility(true, true);
			}
			else if !self.coin_paid{
				self.mini_game.lock_input(false);
				self.mini_game.set_visibility(true, false);
				if self.mini_game.get_winner() == 1 {self.cash.add_coins(1); }
				self.coin_paid = true;
			}
			let resources_produced = self.map.on_update(upd);
			self.cash.add_coins(resources_produced[0]);
			self.cash.add_wood(resources_produced[1]);
			self.cash.add_iron(resources_produced[2]);
			self.cash.add_crystals(resources_produced[3]);
		}
		#[allow(unused_variables)]
		pub fn on_draw(&mut self, ren:RenderArgs, e: PistonWindow){
			e.draw_2d(|c,g| {
				clear([1.0, 1.0, 1.0, 1.0], g);				
				self.cash.draw(g, c.transform, c.draw_state, self.screen_width, self.map_coordinates[1], [self.mouse_x, self.mouse_y]);
				self.mini_game.draw(g, c.transform.trans(0.0, self.map_coordinates[1]), c.draw_state, self.map_coordinates[0], self.map_coordinates[3], [self.mouse_x, self.mouse_y-self.map_coordinates[1]]);
				match self.map.draw(g, c.transform.trans(self.map_coordinates[0], self.map_coordinates[1]), c.draw_state, self.map_coordinates[2], self.map_coordinates[3], [self.mouse_x - self.map_coordinates[0], self.mouse_y-self.map_coordinates[1]])
				{
					Some(DrawRequest::ResourcePrice{price, coordinates, font_size}) => {
						self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
					}
					None => {}
				}
			});
		}
		pub fn on_input(&mut self, inp: Input){
			if let Some(pos) = inp.mouse_cursor_args() {
				self.mouse_x = pos[0] as f64;
				self.mouse_y = pos[1] as f64;
			}
			match inp{
				//Input::Motion::MouseCursor
				Input::Press(but) => {
					match but{
						Button::Keyboard(Key::A) => {
							self.mini_game.change_state_p1(1);
						}
						Button::Keyboard(Key::S) => {
							self.mini_game.change_state_p1(2);
						}
						Button::Keyboard(Key::D) => {
							self.mini_game.change_state_p1(3);
						}
						Button::Keyboard(Key::J) => {
							self.mini_game.change_state_p2(1);
						}
						Button::Keyboard(Key::K) => {
							self.mini_game.change_state_p2(2);
						}
						Button::Keyboard(Key::L) => {
							self.mini_game.change_state_p2(3);
						}
						_ => {}
					}
				}
				Input::Release(but) => {
					match but {
						Button::Mouse(MouseButton::Left) => {
								match self.map.on_click(self.mouse_x - self.map_coordinates[0], self.mouse_y - self.map_coordinates[1]){
									Some(msg) => { self.handle_map_interaction(msg) }
									None => {}
								}
						}
						_ => {}
					}
				}
				_ => {}
			}
		}
		
		fn handle_map_interaction(&mut self, msg: MapUserInteraction) {
			match msg {
				MapUserInteraction::BuyLand{index, price} => {
					if self.cash.take_coins(price){
						self.map.buy_land(index);
					}
				}
				MapUserInteraction::SellLand{index, price} => {
					if self.map.sell_land(index) {
						self.cash.add_coins(price);
					}
				}
				MapUserInteraction::ConcreteLand{index} => {
					if self.cash.take_iron(CONCRETE_PRICE){
						self.map.concrete_land(index);
					}
				}
				MapUserInteraction::BuildIronFactory{index} => {
					if self.cash.take_iron(IRON_FACTORY_PRICE){
						if !self.map.build_iron_factory_on_land(index) {
							//not concreted yet
							self.cash.add_iron(IRON_FACTORY_PRICE);
						}
					}
				}
				MapUserInteraction::UpgradeIronFactory{index} => {
					if self.cash.take_iron(IRON_FACTORY_UPGRADE_PRICE){
						self.map.upgrade_iron_factory(index);
					}
				}
				MapUserInteraction::AddResources{coins, wood, iron, crystals} => {
					self.cash.add_coins(coins);
					self.cash.add_wood(wood);
					self.cash.add_iron(iron);
					self.cash.add_crystals(crystals);
				}
			}
		}
		
	}