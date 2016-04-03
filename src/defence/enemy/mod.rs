/*
<<< enemy >>
*/

//! The super-module of the different enemies.


use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;

pub mod basic_enemy;

pub trait Enemy {
	fn get_coordinates(&self) -> (f64, f64);
	fn get_enemy_type_id(&self) -> usize;

	fn draw(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, w: f64, h: f64, sprite_array: &[Texture<Resources>]) {
		let (sprite_w, sprite_h) = sprite_array[self.get_enemy_type_id()].get_size();
		let x_scale = w/(sprite_w as f64);
		let y_scale = h/(sprite_h as f64);
		let (x,y) = self.get_coordinates();
		image(&(sprite_array[self.get_enemy_type_id()]), view.trans(x,y).scale(x_scale, y_scale), g);
	}
	fn update(&mut self, dt: f64 ) {
		//TODO
	}
}