use utils::ClickableRectangle;
use utils::JkmButton;
use utils::JkmStyle;

use map::MapUserInteraction;
use DrawRequest;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

use rand;

//constants
use CONCRETE_PRICE;
use IRON_FACTORY_PRICE;
use IRON_FACTORY_UPGRADE_PRICE;


enum ButtonType{
	Buy, 
	Sell,
	Concrete,
	BuildIronFactory,
	Lumber,
	UpgradeIronFactory,
}

enum LandType {
	Empty,
	Tree{fir: bool, grow_state: f64},
	Concreted,
	IronFactory{level:u32, stored:f64},
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
	
	pub fn new(x: f64, y: f64, w: f64, h: f64, price: u32) -> Land {
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
		l.refresh_buttons();
		l
	}
	
	/// dt: time in seconds that passed since tha last call
	/// rn: random number for this update, should be different each call
	pub fn update(&mut self, dt: f64, rn: u32) -> Option<MapUserInteraction> {
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
			LandType::IronFactory{level, stored} => {
				if self.bought {
					let mut stored_after = stored + (level as f64 * dt * 0.1);
					if stored_after > 1.0 {
						stored_after -= 1.0;
						self.land_type = LandType::IronFactory{level:level, stored: stored_after};
						self.notification = 3;
						self.notification_y = 0.0;
						return Some(MapUserInteraction::AddResources{coins:0, wood:0, iron:1, crystals:0});
					}
					self.land_type = LandType::IronFactory{level:level, stored: stored_after};
				}
			}
		}
		if refresh_buttons_later {self.refresh_buttons();}
		None
	}
	
	pub fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, sprite_array: &[Texture<Resources>]/*, ref mut font: &mut Glyphs*/){ 
		//Background, brounish
		let mut color = [0.305, 0.231, 0.173, 1.0];
		
		match self.land_type {
			LandType::Empty | LandType::Tree{..} => {/* keep value from above */}
			LandType::Concreted | LandType::IronFactory{..} => { color =  [0.3, 0.3, 0.3, 1.0]}
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
				match *t {
					ButtonType::Buy => {
						(*b).draw(g, view, &(sprite_array[0]), x, y, 2.0 * d, 2.0 * e);
						if hover {
							result = Some(DrawRequest::ResourcePrice{price: [self.buy_price, 0, 0, 0], coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
							//text::Text::new_color([1.0,1.0, 1.0, 1.0], font_size as u32).draw( &(self.buy_price.to_string() + " Gold"), *font, &draw_state, view.trans(x, y + (3.0*e) ), g);
						}
					}
					ButtonType::Sell => {
						(*b).draw(g, view, &(sprite_array[1]), x, y, 2.0 * d, 2.0 * e);
						if hover {
							result = Some(DrawRequest::ResourcePrice{price: [self.sell_price, 0, 0, 0], coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
							//text::Text::new_color([1.0,1.0, 1.0, 1.0], font_size as u32).draw( &(self.sell_price.to_string() + " Gold"), *font, &draw_state, view.trans(x, y + (3.0*e) ), g);
						}
					}
					ButtonType::Concrete => {
						(*b).draw(g, view, &(sprite_array[2]), x, y, 2.0 * d, 2.0 * e);
						if hover {
							result = Some(DrawRequest::ResourcePrice{price: [0, 0, CONCRETE_PRICE, 0], coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
							//text::Text::new_color([1.0,1.0, 1.0, 1.0], font_size as u32).draw( &((CONCRETE_PRICE).to_string() + " Iron"), *font, &draw_state, view.trans(x, y + (3.0*e) ), g);
						}
					}
					ButtonType::BuildIronFactory => {
						(*b).draw(g, view, &(sprite_array[3]), x, y, 2.0 * d, 2.0 * e);
						if hover {
							result = Some(DrawRequest::ResourcePrice{price: [0, 0, IRON_FACTORY_PRICE, 0], coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
							//text::Text::new_color([1.0,1.0, 1.0, 1.0], font_size as u32).draw( &((IRON_FACTORY_PRICE).to_string() + " Iron"), *font, &draw_state, view.trans(x, y + (3.0*e) ), g);
						}
					}
					ButtonType::Lumber => {
						(*b).draw(g, view, &(sprite_array[4]), x, y, 2.0 * d, 2.0 * e);
					}
					ButtonType::UpgradeIronFactory => {
						(*b).draw(g, view, &(sprite_array[5]), x, y, 2.0 * d, 2.0 * e);
						if hover {
							result = Some(DrawRequest::ResourcePrice{price: [0, 0, IRON_FACTORY_UPGRADE_PRICE, 0], coordinates: view.trans(x, y + (3.0*e)), font_size:font_size as u32});
							//text::Text::new_color([1.0,1.0, 1.0, 1.0], font_size as u32).draw( &((IRON_FACTORY_UPGRADE_PRICE).to_string() + " Iron"), *font, &draw_state, view.trans(x, y + (3.0*e) ), g);
						}
					}
				}
			}
		}
		result
	}
	
	pub fn click_buttons (&mut self, x: f64, y: f64) -> Option<MapUserInteraction> {
		let mut result = None;
		let mut refresh_required: bool = false;
		if self.show_buttons {
			for tuple in (self.buttons).iter_mut() {
				let  (ref mut b, ref t): ( JkmButton, ButtonType) = *tuple;
				if b.click(x,y) {
					match *t {
						ButtonType::Buy => {
							result = Some(MapUserInteraction::BuyLand{index: 0 as u32, price: self.buy_price }); //index unkown here
							break;
						}
						ButtonType::Sell => {
							result = Some(MapUserInteraction::SellLand{index: 0 as u32, price: self.sell_price});
							break;
						}
						ButtonType::Concrete => {
							result = Some(MapUserInteraction::ConcreteLand{index: 0 as u32});
							break;
						}
						ButtonType::BuildIronFactory => {
							result = Some(MapUserInteraction::BuildIronFactory{index: 0 as u32});
							break;
						}
						ButtonType::Lumber => {
							match self.land_type {
								LandType::Tree{grow_state, ..} => {
									self.land_type = LandType::Empty;
									refresh_required = true;
									self.notification = 2;
									self.notification_y = 0.0;
									result = Some(MapUserInteraction::AddResources{coins:0, wood:(grow_state * 2.0) as u32, iron:0, crystals:0});
									break;
								}
								_=> { unreachable!() }
							}	
						}
						ButtonType::UpgradeIronFactory => {					
							result = Some(MapUserInteraction::UpgradeIronFactory{index: 0 as u32});
							break;
						}
					}		
				}
			}
		}
		if refresh_required {self.refresh_buttons();}
		result
	}
	
	/// Adjusts the button list that belongs to the land depending on the current LandType and ownership.
	pub fn refresh_buttons(&mut self){
		self.buttons = Vec::new();
		if self.bought {
			match self.land_type {
				LandType::Empty => {
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]) ,ButtonType::Concrete));
				}
				LandType::Concreted => {
					self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::BuildIronFactory));
				}
				LandType::Tree{grow_state, ..} => {
					if grow_state > 0.5 { self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::Rectangle, [0.0,0.0,0.3,0.8]) ,ButtonType::Lumber)); }
					else {self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  * self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]) ,ButtonType::Concrete));}
				}
				LandType::IronFactory{level, ..} => {
					if level < 10 {self.buttons.push((JkmButton::new(0.0, 0.0, (2.0  *self.w / 3.0), (2.0 * self.h/ 3.0), JkmStyle::OuterCircle, [0.1,0.1,0.1,0.9]), ButtonType::UpgradeIronFactory));}
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
	pub fn buy(&mut self){
		self.bought = true;
		self.refresh_buttons();
	}
	pub fn sell(&mut self) -> bool {
		let result = self.bought;
		self.bought = false;
		if result {	self.refresh_buttons(); }
		result
	}
	pub fn concrete(&mut self){		
		self.land_type = LandType::Concreted;
		self.refresh_buttons();
	}
	pub fn build_iron_factory(&mut self) -> bool{
		match self.land_type{
			LandType::Concreted => {
				self.land_type = LandType::IronFactory{level: 1, stored: 0.0};
				self.refresh_buttons();
				true
			}
			_=> {false}
		}
	}
	pub fn upgrade_iron_factory(&mut self){
		if let LandType::IronFactory{ref mut level, ..} = self.land_type{
			*level += 1;	
		}
		self.refresh_buttons();
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
