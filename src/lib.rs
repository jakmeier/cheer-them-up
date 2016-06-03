/*
<<< multy_task >>
*/

pub mod micro;
pub mod cash;
pub mod map;
pub mod utils;
pub mod defence;
pub mod constants;
pub mod definitions;

extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate find_folder;
extern crate freetype;
extern crate rand;


use micro::{PersistentWinnerState, AbsolutelyChangeableState, AI, ClickableGame};
use piston_window::*;

use constants::*;
use definitions::{DrawRequest, Drawable, DefenceUserInteraction, MapUserInteraction, GameState};


/// Root structure for the game. It contains all the mini games (micro) as well as the higher level game parts (macro) and connects them.
pub struct Game {
		screen_width: f64, screen_height: f64,
		header_height: f64,
		game_split_coordinates: [f64;2],
		eco_def_split_coordinate: f64,
		mouse_x: f64, mouse_y: f64, 
		map: map::Map, 
		mini_game: micro::rock_paper_scissors::GameObj,
		game_two: micro::tic_tac_toe::TicTacToeData,
		defence: defence::Defence,
		clock: f64,
		coin_paid: bool,
		cash: cash::CashHeader,
		state: GameState,
	}
	
	impl Game {
		pub fn new(w: &PistonWindow) -> Game {
			
			let screen_width = w.size().width as f64;
			let screen_height = w.size().height as f64;
			let header_height = screen_height * 0.05;
			let map_height = screen_height * 0.65;
			let def_split = screen_width/4.0 * 2.5;
			let x_seperation = def_split * 0.7;
			
			let mut state = GameState::new();
			state.tower_researched[BASIC_TID] = true;
			
			Game{	
				screen_width: screen_width,	screen_height: screen_height,
				header_height: header_height,
				game_split_coordinates : [x_seperation, header_height + map_height],
				eco_def_split_coordinate: def_split,
				mouse_x: 0.0, mouse_y: 0.0, 
				map: map::Map::new(w, 10, 6),
				mini_game: micro::rock_paper_scissors::GameObj::new(w),
				game_two: micro::tic_tac_toe::TicTacToeData::new(w, [0, 0, 1, 0], [0, 0, 0, 1]),
				defence: defence::Defence::new(w, 100, BATTLEFIELD_W, BATTLEFIELD_H),
				clock: 0.0,
				coin_paid: false,
				cash: cash::CashHeader::new(w),
				state: state,
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
			let resources_produced = self.map.on_update(upd, &self.state);
			self.cash.add_coins(resources_produced[0]);
			self.cash.add_wood(resources_produced[1]);
			self.cash.add_iron(resources_produced[2]);
			self.cash.add_crystals(resources_produced[3]);
			
			self.defence.on_update(upd);
			
			self.game_two.on_update(upd.dt);
			
		}
		#[allow(unused_variables)] //ren
		pub fn on_draw(&mut self, ren:RenderArgs, e: PistonWindow){
			e.draw_2d(|c,g| {
				clear([1.0, 1.0, 1.0, 1.0], g);				
				self.cash.draw(g, c.transform, c.draw_state, self.screen_width, self.header_height, [self.mouse_x, self.mouse_y]);
				
				//map
				match self.map.draw(g, c.transform.trans(0.0, self.header_height), c.draw_state, self.eco_def_split_coordinate, self.game_split_coordinates[1]-self.header_height, [self.mouse_x, self.mouse_y-self.header_height])
				{
					Some(DrawRequest::ResourcePrice{price, coordinates, font_size}) => {
						self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
					}
					_ => {}
				}
				//defence
				match self.defence.draw(g, c.transform.trans(self.eco_def_split_coordinate, self.header_height), c.draw_state, (self.screen_width - self.eco_def_split_coordinate), (self.screen_height-self.header_height), [self.mouse_x-self.eco_def_split_coordinate, self.mouse_y-self.header_height], &self.state)
				{
					Some(DrawRequest::ResourcePrice{price, coordinates, font_size}) => {
						self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
					}
					_ => {}
				}
				
				self.mini_game.draw(g, c.transform.trans(0.0, self.game_split_coordinates[1]), c.draw_state, self.game_split_coordinates[0], self.screen_height - self.game_split_coordinates[1], [self.mouse_x, self.mouse_y-self.game_split_coordinates[1]]);
				self.game_two.draw(g, c.transform.trans(self.game_split_coordinates[0], self.game_split_coordinates[1]), c.draw_state, self.eco_def_split_coordinate - self.game_split_coordinates[0], self.screen_height - self.game_split_coordinates[1], [self.mouse_x - self.game_split_coordinates[0], self.mouse_y - self.game_split_coordinates[1]]);
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
								//map
								match self.map.on_click(self.mouse_x, self.mouse_y - self.header_height, &self.state){
									Some(msg) => { self.handle_map_interaction(msg) }
									None => {}
								}
								
								//defence
								match self.defence.on_click(self.mouse_x - self.eco_def_split_coordinate, self.mouse_y - self.header_height, &self.state){
									Some(msg) => { self.handle_defence_interaction(msg) }
									None => {}
								}
								
								//game_two
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
						self.map.land_matrix[index as usize].buy(&self.state);	
					}
				}
				MapUserInteraction::SellLand{index, price} => {
					if self.map.land_matrix[index as usize].sell(&self.state) {
						self.cash.add_coins(price);
					}
				}
				MapUserInteraction::ConcreteLand{index} => {
					if self.cash.test_and_pay(CONCRETE_PRICE){
						self.map.land_matrix[index as usize].concrete(&self.state);	
					}
				}
				MapUserInteraction::BuildIronFactory{index} => {
					if self.cash.test_and_pay(IRON_FACTORY_PRICE){
						if !self.map.land_matrix[index as usize].build_iron_factory(&self.state) {
							//not concreted yet
							self.cash.add_resources(IRON_FACTORY_PRICE);
						}
					}
				}
				MapUserInteraction::UpgradeIronFactory{index, level} => {
					if self.cash.test_and_pay(IRON_FACTORY_UPGRADE_PRICE[level as usize]){
						self.map.land_matrix[index as usize].upgrade_iron_factory(&self.state);	
					}
				}
				MapUserInteraction::UpgradeBank{index, level} => {
					if self.cash.test_and_pay(BANK_UPGRADE_PRICE[level as usize]){
						self.map.land_matrix[index as usize].upgrade_bank(&self.state);	
					}
				}
				MapUserInteraction::BuildUniversity{index} => {
					if self.cash.test_and_pay(UNIVERSITY_PRICE){
						self.map.land_matrix[index as usize].build_university(&self.state);		
					}
				}
				MapUserInteraction::UpgradeUniversity{index, level} => {
					if self.cash.test_and_pay(UPGRADE_UNIVERSITY_PRICE[level as usize]){
						self.map.land_matrix[index as usize].upgrade_university(&self.state);	
					}
				}
				
				MapUserInteraction::AddResources{coins, wood, iron, crystals} => {
					self.cash.add_coins(coins);
					self.cash.add_wood(wood);
					self.cash.add_iron(iron);
					self.cash.add_crystals(crystals);
				}
				MapUserInteraction::BuildBlacksmith{index} => {
					if self.cash.test_and_pay(BLACKSMITH_PRICE){
						self.map.land_matrix[index as usize].build_blacksmith(&self.state);		
					}
				}
				MapUserInteraction::BuildOracle{index} => {
					if self.cash.test_and_pay(ORACLE_PRICE){
						self.map.land_matrix[index as usize].build_oracle(&self.state);		
					}
				}
				MapUserInteraction::BuildBank{index} => {
					if self.cash.test_and_pay(BANK_PRICE){
						self.map.land_matrix[index as usize].build_bank(&self.state);	
					}
				}
				MapUserInteraction::Industrialise => {
					if self.cash.test_and_pay(INDUSTRIALISATION_PRICE){
						self.state.industrialisation = true;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::ResearchEconomy => {
					if self.cash.test_and_pay(ECONOMY_RESEARCH_PRICE){
						self.state.economy = true;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::ResearchTower{index} => {
					if self.cash.test_and_pay(RESEARCH_TOWER_PRICE_LIST[index]){
						self.state.tower_researched[index] = true;	
						//self.defence.shop.make_available(index);
						self.map.update_all_buttons(&self.state);
					}
					
				}
			}
		}
		
		fn handle_defence_interaction(&mut self, msg: DefenceUserInteraction) {
			match msg {
				DefenceUserInteraction::BuyTower{x,y,tower_id} => {
					if self.cash.test_and_pay( TOWER_PRICE_LIST[tower_id] ){
						self.defence.build_tower(x,y,tower_id);
					}
				}
			}
		}
		
	}