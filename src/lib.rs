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
pub mod net;

extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate find_folder;
extern crate freetype;
extern crate rand;

use std::rc::Rc;

use micro::{PersistentWinnerState, AbsolutelyChangeableState, AI, ClickableGame};
use piston_window::*;

use constants::*;
use definitions::{DrawRequest, Drawable, DefenceUserInteraction, MapUserInteraction, GameState, TowerAttribute, Statistics, Settings, Language};


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
		stats: Statistics,
		paused: bool, 
		submitted: bool,
		font: Glyphs,
		config: Rc<Settings>,
	}
	
	impl Game {
		pub fn new(w: &PistonWindow, config: &Rc<Settings>) -> Game {
			
			let screen_width = w.size().width as f64;
			let screen_height = w.size().height as f64;
			let header_height = screen_height * 0.05;
			
			let mut state = GameState::new();
			state.tower_researched[BASIC_TID] = true;
			
			let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
			let ref font = assets.join("FiraSans-Regular.ttf");
			let factory = w.factory.borrow().clone();
			let glyphs = Glyphs::new(font, factory).unwrap();
			
			Game{	
				screen_width: screen_width,	screen_height: screen_height,
				header_height: header_height,
				game_split_coordinates : [screen_width, header_height],
				eco_def_split_coordinate: screen_width,
				mouse_x: 0.0, mouse_y: 0.0, 
				map: map::Map::new(w, 10, 6, &config),
				mini_game: micro::rock_paper_scissors::GameObj::new(w),
				game_two: micro::tic_tac_toe::TicTacToeData::new(w, [0, 0, 1, 0], [0, 0, 0, 2]),
				defence: defence::Defence::new(w, STARTING_LIFES, BATTLEFIELD_W, BATTLEFIELD_H, &state, &config),
				clock: 0.0,
				coin_paid: false,
				cash: cash::CashHeader::new(w),
				state: state,
				stats: Statistics::new(),
				paused: true, submitted: false,
				font: glyphs,
				config: config.clone(),
			}
		}
		pub fn on_update(&mut self, upd: UpdateArgs) {
			if self.paused {
				// Maybe some actions required to show the menu / statistics 
			}
			else if self.defence.alive()
			{
				self.clock += upd.dt;
							
				// Phases for splitting the screen correctly between the active modules			
				if self.clock < PHASE_SWITCH_TIME + START_TIME_PHASE_4 { self.screen_splitting();}
				
				// Game one scheduling
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
					if self.mini_game.get_winner() == 1 {
						let reward = constants::apply_bonus(1, self.state.gold_upgrade as u32);
						self.cash.add_coins(reward); 
						self.stats.resources_produced( [reward,0,0,0]); 
						self.stats.add_win_game_one();
					}
					self.coin_paid = true;
				}
				
				if self.clock > START_TIME_PHASE_2 {
					// Game two
					self.game_two.on_update(upd.dt);
				}
				if self.clock > START_TIME_PHASE_3 {
					// Map
					let resources_produced = self.map.on_update(upd, &self.state);
					self.stats.resources_produced(resources_produced);
					self.cash.add_coins(resources_produced[0]);
					self.cash.add_wood(resources_produced[1]);
					self.cash.add_iron(resources_produced[2]);
					self.cash.add_crystals(resources_produced[3]);
				}
				if self.clock > START_TIME_PHASE_4 {
					// Defence
					self.defence.on_update(upd, &self.state, &mut self.stats);
				}
				
			}
			else {
				self.paused = true;
			}
		}
		fn screen_splitting(&mut self) {
			let map_height = self.screen_height * 0.65;
			let def_split = self.screen_width * 0.625;
			let x_seperation = def_split * 0.7;
			
			if self.clock > (START_TIME_PHASE_2 - PHASE_SWITCH_TIME) && self.clock <= START_TIME_PHASE_2 {
				// move game split point to left
				let progress = (self.clock - (START_TIME_PHASE_2 - PHASE_SWITCH_TIME)) / PHASE_SWITCH_TIME;
				self.game_split_coordinates = [self.screen_width - progress * (self.screen_width - x_seperation), self.header_height];
			} 
			else if self.clock > START_TIME_PHASE_2 && self.clock <= (START_TIME_PHASE_3 - PHASE_SWITCH_TIME) {
				// fix game split point for now
				self.game_split_coordinates = [x_seperation, self.header_height];
			} 
			else if self.clock > (START_TIME_PHASE_3 - PHASE_SWITCH_TIME) && self.clock <= START_TIME_PHASE_3 {
				// move game split point down
				let progress = (self.clock - (START_TIME_PHASE_3 - PHASE_SWITCH_TIME)) / PHASE_SWITCH_TIME;
				self.game_split_coordinates = [x_seperation, self.header_height + progress * map_height];
			} 
			else if self.clock > START_TIME_PHASE_3 && self.clock < (START_TIME_PHASE_4 - PHASE_SWITCH_TIME){
				// fix game split point
				self.game_split_coordinates = [x_seperation, self.header_height + map_height];
			} 
			else if self.clock > (START_TIME_PHASE_4 - PHASE_SWITCH_TIME) && self.clock <= START_TIME_PHASE_4 {
				// move eco split point to left
				let progress = (self.clock - (START_TIME_PHASE_4 - PHASE_SWITCH_TIME)) / PHASE_SWITCH_TIME;
				self.eco_def_split_coordinate = self.screen_width - progress * (self.screen_width - def_split) ;
			} 
			else if self.clock > START_TIME_PHASE_4 {
				// fix eco _split point
				self.eco_def_split_coordinate = def_split;
			}
		}
		
		#[allow(unused_variables)] //ren
		pub fn on_draw(&mut self, ren:RenderArgs, e: PistonWindow){
			e.draw_2d(|c,g| {
				clear([1.0, 1.0, 1.0, 1.0], g);				
				self.cash.draw(g, c.transform, c.draw_state, self.screen_width, self.header_height);
				
				//defence
				match self.defence.draw(g, c.transform.trans(self.eco_def_split_coordinate, self.header_height), c.draw_state, (self.screen_width - self.eco_def_split_coordinate), (self.screen_height-self.header_height), [self.mouse_x-self.eco_def_split_coordinate, self.mouse_y-self.header_height], &self.state)
				{
					Some(DrawRequest::ResourcePrice{price, coordinates, font_size}) => {
						self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
					}
					_ => {}
				}
				
				self.mini_game.draw(g, 
					c.transform.trans(0.0, self.game_split_coordinates[1]), 
					c.draw_state, 
					self.game_split_coordinates[0], 
					self.screen_height - self.game_split_coordinates[1], 
					self.clock < START_TIME_PHASE_2);	
					
				self.game_two.draw(g, 
					c.transform.trans(self.game_split_coordinates[0], 
					self.game_split_coordinates[1]), 
					c.draw_state, 
					self.eco_def_split_coordinate - self.game_split_coordinates[0], 
					self.screen_height - self.game_split_coordinates[1],
					self.clock < START_TIME_PHASE_3);
				
				//map
				let mut draw_requests = self.map.draw(g, c.transform.trans(0.0, self.header_height), self.eco_def_split_coordinate, self.game_split_coordinates[1]-self.header_height, [self.mouse_x, self.mouse_y-self.header_height]);
				while let Some(dr) = draw_requests.pop() {
					match dr {
						DrawRequest::ResourcePrice{price, mut coordinates, font_size} => {
							if coordinates[0][2] < -1.0 { coordinates[0][2] = -1.0; }
							self.cash.draw_resource_price(g, coordinates, c.draw_state, price, font_size);
						}
						DrawRequest::Tooltip{text} => {
							let font_size = self.config.get_std_font_size();
							let y = self.game_split_coordinates[1];
							rectangle([0.2, 0.2, 0.2, 0.9], [0.0, y, self.screen_width, self.screen_height - y ], c.transform, g);
							text::Text::new_color([1.0,1.0,1.0,1.0], font_size).draw( &text, &mut self.font, &c.draw_state, c.transform.trans(0.5 * font_size as f64, y + 1.5 * font_size as f64), g);
						}
						_ => {}
					}
				}
				
				if self.paused {
					// draw overlay
					rectangle([0.2, 0.2, 0.2, 0.9], [0.1 * self.screen_width, 0.1 * self.screen_height, 0.8 * self.screen_width, 0.8 * self.screen_height ], c.transform, g);
					if self.defence.alive() {
						let player_name = self.config.get_name();
						if player_name == "guest" || player_name == "Gast" || player_name == "" {
							match self.config.get_language() {
								Language::English => {
									text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_std_font_size()).draw( "No name found in config.txt", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.2 * self.screen_height), g);
									text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_std_font_size()).draw( "Your highscore will not appear with your name", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.25 * self.screen_height), g);
								},
								Language::German  => {
									text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_std_font_size()).draw( "Kein Name in config.txt eingetragen", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.2 * self.screen_height), g);
									text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_std_font_size()).draw( "Deine Rekorde werden nicht unter deinem Namen angezeigt", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.25 * self.screen_height), g);
								},
							}
						}
						else {let mut greeting = String::from(
							match self.config.get_language() {
								Language::English => "Hello ",
								Language::German  => "Hallo ",
							});
							greeting.push_str(&player_name);
							text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( &greeting, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.25 * self.screen_height), g);
						}
						match self.config.get_language() {
							Language::English => text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( "Press space to play", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.4 * self.screen_height), g),
							Language::German  => text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( "Start mit Leertaste", &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.4 * self.screen_height), g),
						}
						
					}
					else if self.submitted {
						let h1: &str;
						let h2: &str;
						match self.config.get_language() {
							Language::English => {
								h1 = "     Game over     ";
								h2 = "  Score submitted  ";
							}
							Language::German => {
								h1 = "   Spiel vorbei    ";
								h2 = "Punkzahlt versendet";
							}
						}
						text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( h1, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.25 * self.screen_height), g);
						text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( h2, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.4 * self.screen_height), g);
					}
					else {
						let h1: &str;
						let h2: &str;
						let h3: &str;
						match self.config.get_language() {
							Language::English => {
								h1 = "     Game over     ";
								h2 = "Press enter to ";
								h3 = "submit the score";
							}
							Language::German => {
								h1 = "   Spiel vorbei    ";
								h2 = "Eingabetaste drücken";
								h3 = "um abzusenden";
							}
						}
						text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( h1, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.25 * self.screen_height), g);
						text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( h2, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.4 * self.screen_height), g);
						text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( h3, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.6 * self.screen_height), g);
					};
					let s = match self.config.get_language() {
						Language::English => "Score: ",
						Language::German => "Punktzahl: ",
					};
					text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( s, &mut self.font, &c.draw_state, c.transform.trans(0.2 * self.screen_width, 0.8 * self.screen_height), g);
					text::Text::new_color([1.0,1.0,1.0,1.0], self.config.get_title_font_size()).draw( &(self.stats.get_score().to_string()), &mut self.font, &c.draw_state, c.transform.trans(0.6 * self.screen_width, 0.8 * self.screen_height), g);
				}
			
			});
		}
		
		pub fn on_input(&mut self, inp: Input){
			if let Some(pos) = inp.mouse_cursor_args() {
				self.mouse_x = pos[0] as f64;
				self.mouse_y = pos[1] as f64;
			}
			if !self.defence.alive() {
				if !self.submitted {
					match inp{
						Input::Press(but) => {
							match but{
								Button::Keyboard(Key::Return) => {
									if net::send( &self.config.get_name(), self.stats.get_score() ) {
										self.submitted = true;
									}
									else {
										match self.config.get_language() {
											Language::English => println!("Score could not be sumbitted. Check you internet connection and try again."),
											Language::German => println!("Punktzahl konnte nicht gesendet werden. Überprüfe deine Internetverbindung und versuche es erneut."),
										}
									}
								}
								_ => {}
							}
						}
						_ => {}
					}
				}
			}
			else if self.paused {
				match inp{
					Input::Press(but) => {
						match but{
							Button::Keyboard(Key::Space) => {
								self.paused = false;
							}
							_ => {}
						}
					}
					_ => {}
				}
			}
			else {
				match inp{
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
							Button::Keyboard(Key::Space) => {
								self.paused = true;
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
									{
										let g = constants::apply_bonus(reward[0], self.state.gold_upgrade as u32);
										let i = constants::apply_bonus(reward[2], self.state.iron_upgrade as u32);
										let c = constants::apply_bonus(reward[3], self.state.crystal_upgrade as u32);
										self.cash.add_resources([g, reward[1], i, c]);
										self.stats.resources_produced([g, reward[1], i, c]);
										self.stats.add_win_game_two();
									}
								
							}
							_ => {}
						}
					}
					_ => {}
				}
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
					self.stats.resources_produced([coins, wood, iron, crystals]);
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
				MapUserInteraction::BuildBlacksmithII{index} => {
					if self.cash.test_and_pay(BLACKSMITH_II_PRICE){
						self.map.land_matrix[index as usize].build_blacksmith_ii(&self.state);	
					}
				}
				MapUserInteraction::BuildBarracks{index} => {
					if self.cash.test_and_pay(BARRACKS_PRICE){
						self.map.land_matrix[index as usize].build_barracks(&self.state);	
					}
				}
				MapUserInteraction::BuildArcheryRange{index} => {
					if self.cash.test_and_pay(ARCHERY_RANGE_PRICE){
						self.map.land_matrix[index as usize].build_archery_range(&self.state);	
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
				MapUserInteraction::UpgradeGold => {
					if self.cash.test_and_pay(ORACLE_RESEARCH_PRICE_LIST[self.state.gold_upgrade as usize]){
						self.state.gold_upgrade += 1;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::UpgradeIron => {
					if self.cash.test_and_pay(ORACLE_RESEARCH_PRICE_LIST[self.state.iron_upgrade as usize]){
						self.state.iron_upgrade += 1;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::UpgradeCrystal => {
					if self.cash.test_and_pay(ORACLE_RESEARCH_PRICE_LIST[self.state.crystal_upgrade as usize]){
						self.state.crystal_upgrade += 1;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::ResearchTower{index} => {
					if self.cash.test_and_pay(RESEARCH_TOWER_PRICE_LIST[index]){
						self.state.tower_researched[index] = true;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::UpgradeTower{tid, kind, level} => {
					if self.cash.test_and_pay(constants::tower_upgrade_cost(level)){
						let i = match kind {
							TowerAttribute::Attack => 0,
							TowerAttribute::Defence => 1,
							TowerAttribute::Range => 2,
						};
						self.state.tower_upgrades[tid][i] += 1;	
						self.map.update_all_buttons(&self.state);
						if i == 1 {
							self.defence.cascade_health_upgrade(&self.state);
						}
					}
				}
				MapUserInteraction::UpgradeCandy => {
					if self.cash.test_and_pay(tower_upgrade_cost(self.state.tower_upgrades[SLOW_TID][0] as u32)){
						self.state.tower_upgrades[SLOW_TID][0] += 1;	
						self.state.tower_upgrades[SLOW_TID][1] += 1;	
						self.state.tower_upgrades[SLOW_TID][2] += 1;	
						self.map.update_all_buttons(&self.state);
					}
				}
				MapUserInteraction::UpgradeSurprise => {
					if self.cash.test_and_pay(tower_upgrade_cost(self.state.tower_upgrades[ROCKET_TID][0] as u32)){
						self.state.tower_upgrades[ROCKET_TID][0] += 1;	
						self.state.tower_upgrades[ROCKET_TID][1] += 1;	
						self.state.tower_upgrades[ROCKET_TID][2] += 1;	
						self.map.update_all_buttons(&self.state);
					}
				}
				
			}
		}
		
		fn handle_defence_interaction(&mut self, msg: DefenceUserInteraction) {
			match msg {
				DefenceUserInteraction::BuyTower{x,y,tower_id} => {
					if self.cash.test_and_pay( TOWER_PRICE_LIST[tower_id] ){
						self.defence.build_tower(x,y,tower_id, &self.state);
					}
				}
			}
		}
		
	}