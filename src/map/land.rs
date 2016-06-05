use utils::ClickableRectangle;
use utils::JkmButton;
use utils::JkmStyle;

use constants::*;
use definitions::{DrawRequest, MapUserInteraction, GameState, TowerAttribute};

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

use rand;

enum ButtonType{
	Buy, 
	Sell,
	Concrete,
	BuildIronFactory,
	Lumber,
	UpgradeIronFactory{level: u32 },
	BuildUniversity,
	BuildBlacksmith,
	BuildOracle,
	BuildBank,
	UpgradeUniversity{level: u32 },
	UpgradeBank {level: u32},
	Industrialisation,
	EconomyResearch,
	ResearchTower { index: usize},
	UpgradeGold { level: usize },
	UpgradeIron { level: usize },
	UpgradeCrystal { level: usize },
	UpgradeTower { tid: usize, kind: TowerAttribute, level: u32 },
	BuildBlacksmithII, BuildBarracks, BuildArcheryRange,
}

enum LandType {
	Empty,
	Tree{fir: bool, grow_state: f64},
	Concreted,
	IronFactory{level:u32, stored:f64},
	University{level:u32},
	Blacksmith,
	Bank{level:u32, stored:f64},
	Oracle,
	BlacksmithII, Barracks, ArcheryRange,
}

pub struct Land {
	land_type: LandType, rn: u32,
	x: f64, y: f64, w: f64, h:f64,
	buttons: Vec<(JkmButton, ButtonType)>,
	show_buttons: bool,
	buy_price: u32, sell_price: u32,
	bought: bool,
	notification: u32,
	notification_y: f64,
}



impl Land {
	
	pub fn new(x: f64, y: f64, w: f64, h: f64, price: u32, upgrades: &GameState) -> Land {
		let buttons: Vec<(JkmButton, ButtonType)> = Vec::new();
		
		let  mut l = Land {
			land_type: LandType::Empty, rn: rand::random::<u32>(),
			x: x, y: y, 
			buttons: buttons,
			w: w, h: h,
			show_buttons: false,
			buy_price: price,
			sell_price: (price / 2),
			bought: false,
			notification: 0,
			notification_y: 0.0,
		};
		l.refresh_buttons(upgrades);
		l
	}
	
	/// dt: time in seconds that passed since tha last call
	/// rn: random number for this update, should be different each call
	pub fn update(&mut self, dt: f64, rn: u32, upgrades: &GameState) -> Option<MapUserInteraction> {
		if self.notification > 0 { self.notification_y += self.h * dt;}
		if self.notification_y > self.h { self.notification = 0; self.notification_y = 0.0; }
		
		let mut refresh_buttons_later = false;
		
		match self.land_type {
			LandType::Empty => {
				if (rn % 10000) == (self.rn % 10000) {
					if rn % 2 == 0 {self.land_type = LandType::Tree{fir:true, grow_state: 0.2}; }
					else {self.land_type = LandType::Tree{fir:false, grow_state: 0.2};}
					refresh_buttons_later = true;
				}
			}
			LandType::Tree{ref mut grow_state, ..} => {
				let no_wood_before = *grow_state <= 0.5;
				*grow_state +=   dt * ( ((self.rn /2) + (rn/2)) % 100  ) as f64 * 0.0002 ;
				if *grow_state > 3.0 {*grow_state = 3.0;}
				if no_wood_before && *grow_state >= 0.5 { refresh_buttons_later = true; }
			}
			LandType::Concreted => {}
			LandType::IronFactory{level, ref mut stored} => {
				if self.bought {
					*stored += level as f64 * dt * 0.1;
					if *stored > 1.0 {
						*stored -= 1.0;
						self.notification = 3; // iron
						self.notification_y = 0.0;
						return Some(MapUserInteraction::AddResources{coins:0, wood:0, iron:1, crystals:0});
					}
				}
			}
			LandType::University{..} => {}
			LandType::Blacksmith => {}
			LandType::Bank{level, ref mut stored} => {
				if self.bought {
					*stored += level as f64 * dt / 17.0; // one gold for each level every 17 seconds
					if *stored > 1.0 {
						*stored -= 1.0;
						self.notification = 1; // gold
						self.notification_y = 0.0;
						return Some(MapUserInteraction::AddResources{coins:1, wood:0, iron:0, crystals:0});
					}
				}
			}
			LandType::Oracle => {}
			LandType::BlacksmithII => {}
			LandType::Barracks => {}
			LandType::ArcheryRange => {}
		}
		if refresh_buttons_later {self.refresh_buttons(upgrades);}
		None
	}
	
	pub fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, sprite_array: &[Texture<Resources>]/*, ref mut font: &mut Glyphs*/){ 
		//Background, brounish
		let color;
		
		match self.land_type {
			LandType::Empty | LandType::Tree{..} | LandType::Oracle => color = [0.305, 0.231, 0.173, 1.0],
			_ =>  color =  [0.3, 0.3, 0.3, 1.0],
		}
		
		
		rectangle(color, [self.x, self.y, self.w, self.h ], view, g);
		
		//sprite
		match self.land_type {
			LandType::Empty => {
				let (sprite_w, sprite_h) = sprite_array[0].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[0]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::Tree{fir, grow_state} => {
				let state = grow_state as usize;
				let mut offset = 1 as usize;
				if !fir {offset += 4 }
				let (sprite_w, sprite_h) = sprite_array[offset + state].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				let mut grow_scale = 1.0;
				if state == 0 {	grow_scale = grow_state; }
				image(&(sprite_array[offset + state]), view.trans(self.x + ((1.0-grow_scale) * self.w / 2.0), self.y + ((1.0-grow_scale) * self.h)).scale(x_scale * grow_scale, y_scale * grow_scale), g);
			}
			LandType::Concreted => {} 
			LandType::IronFactory{..} => {
				let (sprite_w, sprite_h) = sprite_array[9].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[9]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::University{level} => {
				let (sprite_w, sprite_h) = sprite_array[14+level as usize].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[14+level as usize]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::Blacksmith => {
				let (sprite_w, sprite_h) = sprite_array[18].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[18]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::Bank{..} => {
				let (sprite_w, sprite_h) = sprite_array[19].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[19]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::Oracle => {
				let (sprite_w, sprite_h) = sprite_array[20].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[20]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::BlacksmithII => {
				let (sprite_w, sprite_h) = sprite_array[20].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[21]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::Barracks => {
				let (sprite_w, sprite_h) = sprite_array[20].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[22]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
			LandType::ArcheryRange => {
				let (sprite_w, sprite_h) = sprite_array[20].get_size();
				let x_scale = self.w/(sprite_w as f64);
				let y_scale = self.h/(sprite_h as f64);
				image(&(sprite_array[23]), view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
		}
		
		//fog (of war?)
		if !self.bought {
			let color = [0.1, 0.1, 0.1, 0.5];
			rectangle(color, [self.x, self.y, self.w, self.h ], view, g);
		}
		
		//notifications
		if self.notification > 0 {
			let (sprite_w, sprite_h) = sprite_array[9 + self.notification as usize].get_size();
			let x_scale = self.w/(sprite_w as f64);
			let y_scale = self.h/(sprite_h as f64);
			let scale = if x_scale < y_scale { x_scale } else {y_scale};
			image(&(sprite_array[9 + self.notification as usize]), view.trans(self.x, self.y - self.notification_y).scale(scale, scale), g);
		}
		
		
	}
	
	pub fn draw_buttons (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, sprite_array: &[Texture<Resources>], mouse: [f64;2])
		-> Option<DrawRequest>
	{
		let mut result = None;
		if self.show_buttons {
			let d = self.w / 3.0;
			let e = self.h / 3.0;
			let left_col = self.x - d;
			let right_col = self.x + (2.0 * d);
			
			for (i, tuple) in (self.buttons).iter_mut().enumerate() {
				let  (ref mut b, ref t): ( JkmButton, ButtonType) = *tuple;
				
				let mut x = left_col;
				let y = self.y - e + 3.0 * e *(i as u32 / 2) as f64; //make use of floor-rounding behavior
				if i % 2 == 1 {
					x = right_col;
				}
				let hover = (*b).on_area(mouse[0], mouse[1]);
				let font_size = self.h / 3.0;
				let mut sprite_index = None;
				let mut second_sprite_index = None;
				let mut price = None;
				match *t {
					ButtonType::Buy => {
						sprite_index = Some(0);
						price = Some([self.buy_price, 0, 0, 0]);
					}
					ButtonType::Sell => {
						sprite_index = Some(1);
						price = Some([self.sell_price, 0, 0, 0]);
					}
					ButtonType::Concrete => {
						sprite_index = Some(2);
						price = Some(CONCRETE_PRICE);
					}
					ButtonType::BuildIronFactory => {
						sprite_index = Some(3);
						price = Some(IRON_FACTORY_PRICE);
					}
					ButtonType::Lumber => {
						sprite_index = Some(4);
					}
					ButtonType::UpgradeIronFactory{level} => {
						sprite_index = Some(5);
						price = Some(IRON_FACTORY_UPGRADE_PRICE[level as usize]);
					}
					ButtonType::UpgradeBank{level} => {
						sprite_index = Some(5);
						price = Some(BANK_UPGRADE_PRICE[level as usize]);
					}
					ButtonType::BuildUniversity => {
						sprite_index = Some(6);
						price = Some(UNIVERSITY_PRICE);
					}
					ButtonType::UpgradeUniversity{level} => {
						sprite_index = Some(5);
						price = Some(UPGRADE_UNIVERSITY_PRICE[level as usize]);
					}
					ButtonType::Industrialisation => {
						sprite_index = Some(7);
						price = Some(INDUSTRIALISATION_PRICE);
					}				
					ButtonType::BuildBlacksmith => {
						sprite_index = Some(8);
						price = Some(BLACKSMITH_PRICE);
					}
					ButtonType::BuildOracle => {
						sprite_index = Some(9);
						price = Some(ORACLE_PRICE);
					}
					ButtonType::BuildBank => {
						sprite_index = Some(10);
						price = Some(BANK_PRICE);
					}
					ButtonType::ResearchTower{index} => {
						match index {
							AOE_TID => {
								sprite_index = Some(11);
								price = Some(RESEARCH_TOWER_PRICE_LIST[AOE_TID]);
							}
							WALL_TID => {
								sprite_index = Some(12);
								price = Some(RESEARCH_TOWER_PRICE_LIST[WALL_TID]);
							}
							_ => {
								println!("Unexpected index for tower research: {}", index);
							}
						}						
					}
					ButtonType::EconomyResearch => {
						sprite_index = Some(13);
						price = Some(ECONOMY_RESEARCH_PRICE);
					}
					ButtonType::UpgradeGold{level} => {
						sprite_index = Some(14);
						price = Some(ORACLE_RESEARCH_PRICE_LIST[level]);
					}
					ButtonType::UpgradeIron{level} => {
						sprite_index = Some(15);
						price = Some(ORACLE_RESEARCH_PRICE_LIST[level]);
					}
					ButtonType::UpgradeCrystal{level} => {
						sprite_index = Some(16);
						price = Some(ORACLE_RESEARCH_PRICE_LIST[level]);
					}
					ButtonType::UpgradeTower { tid, ref kind, level } => {
						sprite_index = match tid {
							BASIC_TID => Some(20),
							AOE_TID => Some(21),
							WALL_TID => Some(22),
							_ => { 
								println!("Unexpected TID to upgrade: {}", tid);
								None
							}
						};
						second_sprite_index = match *kind {
							TowerAttribute::Attack => Some(17),
							TowerAttribute::Defence => Some(18),
							TowerAttribute::Range => Some(19),
						};
						price = Some(tower_upgrade_cost(level));
					}
					ButtonType::BuildBlacksmithII => {
						sprite_index = Some(23);
						price = Some(BLACKSMITH_II_PRICE);
					}
					ButtonType::BuildBarracks => {
						sprite_index = Some(24);
						price = Some(BARRACKS_PRICE);
					}
					ButtonType::BuildArcheryRange => {
						sprite_index = Some(25);
						price = Some(ARCHERY_RANGE_PRICE);
					}
				}
				if let Some(i) = sprite_index {
					(*b).draw(g, view, &(sprite_array[i]), x, y, 2.0 * d, 2.0 * e);
				}
				if let Some(i) = second_sprite_index {
					(*b).draw(g, view, &(sprite_array[i]), x, y, 2.0 * d, 2.0 * e);
				}
				
				if let Some(p) = price {
					if hover {	
						result = Some(DrawRequest::ResourcePrice{price: p, coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
					}
				}
			}
		}
		result
	}
	
	pub fn click_buttons (&mut self, x: f64, y: f64, upgrades: &GameState) -> Option<MapUserInteraction> {
		let mut result = None;
		let mut refresh_required: bool = false;
		if self.show_buttons {
			for tuple in (self.buttons).iter_mut() {
				let  (ref mut b, ref t): ( JkmButton, ButtonType) = *tuple;
				if b.click(x,y) {
					match *t {
						ButtonType::Buy => {
							result = Some(MapUserInteraction::BuyLand{index: 0 as u32, price: self.buy_price }); //index unkown here
						}
						ButtonType::Sell => {
							result = Some(MapUserInteraction::SellLand{index: 0 as u32, price: self.sell_price});
						}
						ButtonType::Concrete => {
							result = Some(MapUserInteraction::ConcreteLand{index: 0 as u32});
						}
						ButtonType::BuildIronFactory => {
							result = Some(MapUserInteraction::BuildIronFactory{index: 0 as u32});
						}
						ButtonType::Lumber => {
							match self.land_type {
								LandType::Tree{grow_state, ..} => {
									self.land_type = LandType::Empty;
									refresh_required = true;
									self.notification = 2;
									self.notification_y = 0.0;
									result = Some(MapUserInteraction::AddResources{coins:0, wood:(grow_state * 2.0) as u32, iron:0, crystals:0});
								}
								_=> { unreachable!() }
							}	
						}
						ButtonType::UpgradeIronFactory{level} => {					
							result = Some(MapUserInteraction::UpgradeIronFactory{index: 0 as u32, level: level});
						}
						ButtonType::UpgradeBank{level} => {					
							result = Some(MapUserInteraction::UpgradeBank{index: 0 as u32, level: level});
						}
						ButtonType::BuildUniversity => {
							result = Some(MapUserInteraction::BuildUniversity{index: 0 as u32});
						}
						ButtonType::UpgradeUniversity{level} => {
							result = Some(MapUserInteraction::UpgradeUniversity{index: 0 as u32, level: level});
						}
						ButtonType::Industrialisation => {
							result = Some(MapUserInteraction::Industrialise);
						}
						ButtonType::EconomyResearch => {
							result = Some(MapUserInteraction::ResearchEconomy);
						}
						ButtonType::BuildBlacksmith => {
							result = Some(MapUserInteraction::BuildBlacksmith{index: 0 as u32});
						}
						ButtonType::BuildOracle => {
							result = Some(MapUserInteraction::BuildOracle{index: 0 as u32});
						}
						ButtonType::BuildBank => {
							result = Some(MapUserInteraction::BuildBank{index: 0 as u32});
						}
						ButtonType::ResearchTower{index} => {
							result = Some(MapUserInteraction::ResearchTower{index: index} );
						}
						ButtonType::UpgradeGold{..} => {
							result = Some(MapUserInteraction::UpgradeGold);
						}
						ButtonType::UpgradeIron{..} => {
							result = Some(MapUserInteraction::UpgradeIron);
						}
						ButtonType::UpgradeCrystal{..} => {
							result = Some(MapUserInteraction::UpgradeCrystal);
						}
						ButtonType::UpgradeTower{tid, ref kind, level} => {
							result = Some(MapUserInteraction::UpgradeTower{tid: tid, kind: (*kind).clone(), level: level});
						}
						ButtonType::BuildBlacksmithII => {
							result = Some(MapUserInteraction::BuildBlacksmithII{index: 0});
						}
						ButtonType::BuildBarracks => {
							result = Some(MapUserInteraction::BuildBarracks{index: 0});
						}
						ButtonType::BuildArcheryRange => {
							result = Some(MapUserInteraction::BuildArcheryRange{index: 0});
						}
					}
					break;
				}
			}
		}
		if refresh_required {self.refresh_buttons(upgrades);}
		result
	}
	
	/// Adjusts the button list that belongs to the land depending on the current LandType and ownership.
	pub fn refresh_buttons(&mut self, upgrades: &GameState){
		self.buttons = Vec::new();
		if self.bought {
			match self.land_type {
				LandType::Empty => {
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildOracle));
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]) ,ButtonType::Concrete));
				}
				LandType::Concreted => {
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildUniversity));
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildBlacksmith));
					if upgrades.industrialisation {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildIronFactory));
					}
					if upgrades.economy {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildBank));
					}			
				}
				LandType::Tree{grow_state, ..} => {
					if grow_state > 0.5 { 
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::Rectangle, [0.0,0.0,0.3,0.8]) ,ButtonType::Lumber)); 
					}
					else {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildOracle));
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]) ,ButtonType::Concrete));
					}
				}
				LandType::IronFactory{level, ..} => {
					if level < IRON_FACTORY_UPGRADES as u32 + 1 {self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::UpgradeIronFactory{level: level-1}));}
				}
				LandType::University{level} => {
					if !upgrades.industrialisation { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5,0.5,1.0,0.9]), ButtonType::Industrialisation)); }
					if level >= 2 && !upgrades.economy { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5,0.5,1.0,0.9]), ButtonType::EconomyResearch)); }
					if level < UNIVERSITY_UPGRADES as u32 {self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::UpgradeUniversity{level:level}));}
				}
				LandType::Blacksmith => {
					if !upgrades.tower_researched[AOE_TID] { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5,0.5,1.0,0.9]), ButtonType::ResearchTower{index:AOE_TID})); }
					if !upgrades.tower_researched[WALL_TID] { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5,0.5,1.0,0.9]), ButtonType::ResearchTower{index:WALL_TID})); }
					if upgrades.industrialisation {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildBlacksmithII));
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildBarracks));
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildArcheryRange));
					}
					
				}
				LandType::Bank{level, ..} => {
					if level < BANK_UPGRADES as u32 + 1 {self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::UpgradeBank{level: level-1 }));}
				}
				LandType::Oracle => {
					if (upgrades.gold_upgrade as usize) < ORACLE_RESEARCH_LEVELS {
						 self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.3,1.0,0.3,0.9]), ButtonType::UpgradeGold{level: upgrades.gold_upgrade as usize}));
					}
					if (upgrades.iron_upgrade as usize) < ORACLE_RESEARCH_LEVELS {
						 self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.3,1.0,0.3,0.9]), ButtonType::UpgradeIron{level: upgrades.iron_upgrade as usize}));
					}
					if (upgrades.crystal_upgrade as usize) < ORACLE_RESEARCH_LEVELS {
						 self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.3,1.0,0.3,0.9]), ButtonType::UpgradeCrystal{level: upgrades.crystal_upgrade as usize}));
					}
				}
				LandType::BlacksmithII => {
					if upgrades.tower_researched[BASIC_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: BASIC_TID, kind: TowerAttribute::Attack, level: upgrades.tower_upgrades[BASIC_TID][0] as u32}));
					}
					if upgrades.tower_researched[AOE_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: AOE_TID, kind: TowerAttribute::Attack, level: upgrades.tower_upgrades[AOE_TID][0] as u32}));
					}
				}
				LandType::Barracks => {
					if upgrades.tower_researched[BASIC_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: BASIC_TID, kind: TowerAttribute::Defence, level: upgrades.tower_upgrades[BASIC_TID][1] as u32}));
					}
					if upgrades.tower_researched[AOE_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: AOE_TID, kind: TowerAttribute::Defence, level: upgrades.tower_upgrades[AOE_TID][1] as u32}));
					}
					if upgrades.tower_researched[WALL_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: WALL_TID, kind: TowerAttribute::Defence, level: upgrades.tower_upgrades[WALL_TID][1] as u32}));
					}
				}
				LandType::ArcheryRange => {
					if upgrades.tower_researched[BASIC_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: BASIC_TID, kind: TowerAttribute::Range, level: upgrades.tower_upgrades[BASIC_TID][2] as u32}));
					}
					if upgrades.tower_researched[AOE_TID] {
						self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.5, 0.2,0.2,0.9]), ButtonType::UpgradeTower{tid: AOE_TID, kind: TowerAttribute::Range, level: upgrades.tower_upgrades[AOE_TID][2] as u32}));
					}
				}
				
			}
			self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::Rectangle, [0.0,0.0,0.3,0.8]) ,ButtonType::Sell));
		}
		else { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::Rectangle, [0.0,0.0,0.3,0.8]) ,ButtonType::Buy)); }
		
	}
	
	pub fn set_coordinates(&mut self, x: f64, y: f64, w: f64, h: f64) {
		self.x = x;
		self.y = y;
		self.w = w;
		self.h = h;
	}

// Downwards: interface for game state updates
	pub fn buy(&mut self, s: &GameState){
		self.bought = true;
		self.refresh_buttons(s);
	}
	pub fn sell(&mut self, s: &GameState) -> bool {
		let result = self.bought;
		self.bought = false;
		if result {	self.refresh_buttons(s); }
		result
	}
	pub fn concrete(&mut self, s: &GameState){		
		self.land_type = LandType::Concreted;
		self.refresh_buttons(s);
	}
	pub fn build_iron_factory(&mut self, s: &GameState) -> bool{
		match self.land_type{
			LandType::Concreted => {
				self.land_type = LandType::IronFactory{level: 1, stored: 0.0};
				self.refresh_buttons(s);
				true
			}
			_=> {false}
		}
	}
	pub fn upgrade_iron_factory(&mut self, s: &GameState){
		if let LandType::IronFactory{ref mut level, ..} = self.land_type{
			*level += 1;	
		}
		self.refresh_buttons(s);
	}
	pub fn upgrade_bank(&mut self, s: &GameState){
		if let LandType::Bank{ref mut level, ..} = self.land_type{
			*level += 1;	
		}
		self.refresh_buttons(s);
	}
	pub fn build_university(&mut self, s: &GameState){
		self.land_type = LandType::University{level: 0};
		self.refresh_buttons(s);
	}
	pub fn upgrade_university(&mut self, s: &GameState){
		if let LandType::University{ref mut level} = self.land_type{
			*level += 1;	
		}
		self.refresh_buttons(s);
	}
	pub fn build_blacksmith(&mut self, s: &GameState){
		self.land_type = LandType::Blacksmith{};
		self.refresh_buttons(s);
	}
	pub fn build_bank(&mut self, s: &GameState){
		self.land_type = LandType::Bank{level: 1, stored:0.0};
		self.refresh_buttons(s);
	}
	pub fn build_oracle(&mut self, s: &GameState){
		self.land_type = LandType::Oracle{};
		self.refresh_buttons(s);
	}
	pub fn build_barracks(&mut self, s: &GameState){
		self.land_type = LandType::Barracks;
		self.refresh_buttons(s);
	}
	pub fn build_blacksmith_ii(&mut self, s: &GameState){
		self.land_type = LandType::BlacksmithII;
		self.refresh_buttons(s);
	}
	pub fn build_archery_range(&mut self, s: &GameState){
		self.land_type = LandType::ArcheryRange{};
		self.refresh_buttons(s);
	}
}


impl ClickableRectangle for Land {
	fn coordinates(&self) -> (f64, f64, f64, f64) {
		(self.x, self.y, self.w, self.h)
	}
	fn on_click(&mut self) {
		self.show_buttons = ! self.show_buttons; 
	}
	fn on_click_elsewhere(&mut self){
		self.show_buttons = false;
	}
}
