/*
<<< shope >>
*/

//! The user interface for the tower defence. Towers and Walls, which are mostly treated like towers anyway, can be bought as well as some cool other features might be implemented in the future. 

use std::rc::Rc;

use constants::*;
use definitions::{DrawRequest, DefenceUserInteraction, GameState, Settings};

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub struct Shop {
	selected: Option<usize>,
	w: f64, h: f64,
	config: Rc<Settings>,
}

impl Shop {
	pub fn new(config: &Rc<Settings>) -> Shop {
		Shop {
			selected: None,
			w: 1.0, h: 1.0,
			config: config.clone(),
		}
	}
	
	/// Helper function that takes the mouse coordinates as arguments and returns a tower id if a corresponding button is hovered over
	fn find_button(&self, x: f64, y:f64, state: &GameState) -> Option<usize> {
		let s = self.w / NUMBER_OF_TOWERS as f64;
		let s = if s < self.h {s} else {self.h};
		if x > 0.0 && x < self.w && y > 0.0 && y < self.h {
			let mut j = x / s;
			for i in 0..NUMBER_OF_TOWERS {
				if state.tower_researched[i] { j -= 1.0;}
				if j < 0.0 {
					return Some(i);
				}
			}
		}
		None
	}
	
	#[allow(unused_variables)]
	pub fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, w: f64, h:f64, mouse: [f64;2], sprite_array: &[Texture<Resources>], dx: f64, dy: f64, state: &GameState) -> Option<DrawRequest> {
		
		self.w = w;
		self.h = h;
		
		rectangle([0.2, 0.2, 0.7, 1.0], [0.0, 0.0, w, h ], view, g);
		
		let s = w / NUMBER_OF_TOWERS as f64;
		let s = if s < h {s} else {h};
		let mut j = 0;
		
		for i in 0..NUMBER_OF_TOWERS {
			if state.tower_researched[i] {
			//Background for each button
				let color = if j % 2 == 0 {[0.25,0.25,0.25,0.7]} else {[0.0,0.0,0.0,0.7]};
				rectangle(color, [j as f64 * s, 0.0, s, s], view, g);
			// Images of towers
				let (sprite_w, sprite_h) = sprite_array[i].get_size();
				let ratio: f64 = sprite_w as f64 / sprite_h as f64;
				let high_formed = ratio < 1.0;
				let scale = if high_formed {s/sprite_h as f64} else {s/(sprite_w as f64)};
				let x = j as f64 * s + if high_formed {(1.0 - ratio)* s/2.0} else {0.0};
				let y = if high_formed {0.0} else {(ratio-1.0)* s/2.0};
				image(&(sprite_array[i]), view.trans(x,y).scale(scale, scale), g);
				
				j += 1;
			}
		}
		
		let x = mouse[0];
		let y = mouse[1];
		// Selected Draw request
		if let Some(i) =  self.selected {
			return Some(DrawRequest::DrawTower{tower_id:i});
		}
		
		// Price Draw request 
		if let Some(i) = self.find_button(x,y, state) {
			return Some(DrawRequest::ResourcePrice{price: TOWER_PRICE_LIST[i], coordinates: view.trans(x,y), font_size: self.config.get_std_font_size()});
		}
				
		None
	}

	pub fn on_click (&mut self, x: f64, y: f64, state: &GameState) -> Option<DefenceUserInteraction> {
		if let Some(i) =  self.selected {
			self.selected = None;
			let (w,h) = TOWER_SIZE_LIST[i];
			return Some(DefenceUserInteraction::BuyTower{x:w, y:h, tower_id:i});
		}
		self.selected = self.find_button(x,y, state);
		None
	}
	
}