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

use micro::{PersistentWinnerState, AbsolutelyChangeableState, AI, ClickableGame};
use map::MapUserInteraction;
use piston_window::*;

pub const CONCRETE_PRICE: [u32;4] = [0,0,3,0];
pub const IRON_FACTORY_PRICE: [u32;4] = [0,10,5,0];
pub const IRON_FACTORY_UPGRADE_PRICE: [u32;4] = [1,0,1,0];
pub const UNIVERSITY_PRICE: [u32;4] = [5,5,5,5];

/// Message used when a module needs to draw something and it lacks information from another module.
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
		header_height: f64,
		game_split_coordinates: [f64;2],
		mouse_x: f64, mouse_y: f64, 
		map: map::Map, 
		mini_game: micro::rock_paper_scissors::GameObj,
		game_two: micro::tic_tac_toe::TicTacToeData,
		clock: f64,
		coin_paid: bool,
		cash: cash::CashHeader,
	}
	
	impl Game {
		pub fn new(w: &PistonWindow) -> Game {
			
			let screen_width = w.size().width as f64;
			let screen_height = w.size().height as f64;
			let header_height = screen_height * 0.05;
			let map_height = screen_height * 0.55;
			let x_seperation = screen_width * 0.7;
			
			Game{	
				screen_width: screen_width,	screen_height: screen_height,
				header_height: header_height,
				game_split_coordinates : [x_seperation, header_height + map_height],
				mouse_x: 0.0, mouse_y: 0.0, 
				map: map::Map::new(w, 15, 5),
				mini_game: micro::rock_paper_scissors::GameObj::new(w),
				game_two: micro::tic_tac_toe::TicTacToeData::new(w, [0, 0, 1, 0], [0, 0, 0, 1]),
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
				if self.coin_paid {self.mini_game.make_ai_turn(); } //ai
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
			self.game_two.on_update(upd.dt);
		}
		#[allow(unused_variables)] //ren
		pub fn on_draw(&mut self, ren:RenderArgs, e: PistonWindow){
			e.draw_2d(|c,g| {
				clear([1.0, 1.0, 1.0, 1.0], g);				
				self.cash.draw(g, c.transform, c.draw_state, self.screen_width, self.header_height, [self.mouse_x, self.mouse_y]);
				match self.map.draw(g, c.transform.trans(0.0, self.header_height), c.draw_state, self.screen_width, self.game_split_coordinates[1]-self.header_height, [self.mouse_x, self.mouse_y-self.header_height])
				{
					Some(DrawRequest::ResourcePrice{price, coordinates, font_size}) => {
						self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
					}
					None => {}
				}
				self.mini_game.draw(g, c.transform.trans(0.0, self.game_split_coordinates[1]), c.draw_state, self.game_split_coordinates[0], self.screen_height - self.game_split_coordinates[1], [self.mouse_x, self.mouse_y-self.game_split_coordinates[1]]);
				self.game_two.draw(g, c.transform.trans(self.game_split_coordinates[0], self.game_split_coordinates[1]), c.draw_state, self.screen_width - self.game_split_coordinates[0], self.screen_height - self.game_split_coordinates[1], [self.mouse_x - self.game_split_coordinates[0], self.mouse_y - self.game_split_coordinates[1]]);
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
								match self.map.on_click(self.mouse_x, self.mouse_y - self.header_height){
									Some(msg) => { self.handle_map_interaction(msg) }
									None => {}
								}
								if let Some(reward) = self.game_two.click(self.mouse_x - self.game_split_coordinates[0], self.mouse_y - self.game_split_coordinates[1])
								{self.cash.add_resources(reward);}
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
					if self.cash.test_and_pay(CONCRETE_PRICE){
						self.map.concrete_land(index);
					}
				}
				MapUserInteraction::BuildIronFactory{index} => {
					if self.cash.test_and_pay(IRON_FACTORY_PRICE){
						if !self.map.build_iron_factory_on_land(index) {
							//not concreted yet
							self.cash.add_resources(IRON_FACTORY_PRICE);
						}
					}
				}
				MapUserInteraction::BuildUniversity{index} => {
					if self.cash.test_and_pay(UNIVERSITY_PRICE){
						self.map.build_university(index);	
					}
				}
				MapUserInteraction::UpgradeIronFactory{index} => {
					if self.cash.test_and_pay(IRON_FACTORY_UPGRADE_PRICE){
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