/*

<<< utils >>
	
*/
//! Reusable utilities like Buttons can be found in here.

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

/// Provides functions to test clicks and hovers on the rectangle given by the coordinates function 
pub trait ClickableRectangle {
	//requires
	/// -> x, y, w, h
	fn coordinates(&self) -> (f64, f64, f64, f64); 	
	fn on_click(&mut self);
	fn on_click_elsewhere (&mut self);
	
	//provides
	fn on_area(&self, x: f64, y:f64) -> bool{
		let (x0, y0, w, h) = self.coordinates();
		{x > x0 && x < (x0 + w) && y > y0 && y < (y0 + h)}
	}
	fn click(&mut self, x: f64, y:f64) -> bool{
		if self.on_area(x,y) {
			self.on_click();
			true
		}
		else { 
			self.on_click_elsewhere();
			false 
		}
	}
}

/// Different Styles for Buttons. The actual implementation of the styles are in the draw_buttons function defined in land.rs
pub enum JkmStyle{
	OuterCircle,
	Rectangle,
	PictureOnly,
}

/// Custom buttons which can be clicked and have different Style options
pub struct JkmButton {
	x: f64, y: f64, w: f64, h:f64,
	style: JkmStyle,
	color: [f32; 4],
}

impl JkmButton {
	pub fn new(x: f64, y: f64, w: f64, h: f64, style: JkmStyle, col: [f32;4] ) -> JkmButton {
		JkmButton {
			x: x, y: y, 
			w: w, h: h,
			style: style,
			color: col,	
		}
	}
	pub fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, sprite: &Texture<Resources>, new_x: f64, new_y: f64, new_w: f64, new_h: f64){ 
		self.x = new_x;
		self.y = new_y;
		self.w = new_w;
		self.h = new_h;
		let (sprite_w, sprite_h) = sprite.get_size();
		let x_scale = self.w as f64 / (sprite_w as f64);
		let y_scale = self.h as f64 / (sprite_h as f64);
		match self.style {
			JkmStyle::OuterCircle => {
				ellipse(self.color, [self.x - (self.w * 0.20710678), self.y - (self.h * 0.20710678), self.w * 1.41421, self.h * 1.41421], view, g);
				image(sprite, view.trans(self.x + (self.w * 0.125), self.y + (self.h * 0.125)).scale(0.75 * x_scale, 0.75 * y_scale), g);
			}
			JkmStyle::Rectangle => {
				rectangle(self.color, [self.x, self.y, self.w, self.h], view, g);
				image(sprite, view.trans(self.x + (self.w * 0.2), self.y + (self.h * 0.2)).scale(0.6 * x_scale, 0.6 * y_scale), g);
			}
			_ => {
				image(sprite, view.trans(self.x, self.y).scale(x_scale, y_scale), g);
			}
		}
		
	}
}

impl ClickableRectangle for JkmButton {
	fn coordinates(&self) -> (f64, f64, f64, f64) {
		(self.x, self.y, self.w, self.h)
	}
	fn on_click(&mut self) {}
	fn on_click_elsewhere(&mut self) {}
}