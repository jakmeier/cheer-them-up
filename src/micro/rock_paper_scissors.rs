//! Manages the states and the drawing of a rock-paper-scissors game. Input, turn-timing as well as rewards are not manged in this module, but it offers an API so it can be implemented easily.

use super::{PersistentWinnerState, AbsolutelyChangeableState, AI};
use definitions::{Drawable, DrawRequest};
use super::super::piston_window::*;
use super::super::gfx_device_gl::Resources;
use super::super::gfx_device_gl::command::CommandBuffer;
use super::super::gfx_graphics::GfxGraphics;
use super::super::find_folder;

use rand;
use rand::Rng;

pub struct GameObj{
	state_p1: u32,
	state_p2: u32,
	input_lock: bool,
	hide_p1: bool,  
	hide_p2: bool,
	scissors_sprite: Texture<Resources>,
	rock_sprite: Texture<Resources>,
	paper_sprite: Texture<Resources>,
	unknown_sprite: Texture<Resources>,
	t: u8,
	font: Glyphs,
	ai_activated: bool,
	ai: AiData,
}


//constructor
impl GameObj {
	pub fn new(w: &PistonWindow) -> GameObj {
		let img = find_folder::Search::ParentsThenKids(3, 3).for_folder("img").unwrap();
		let scissors = img.join("scissors.png");
		let scissors = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&scissors,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let rock = img.join("rock.png");
		let rock = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&rock,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let paper = img.join("paper.png");
		let paper = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&paper,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let unknown = img.join("unknown.png");
		let unknown = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&unknown,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
		let ref font = assets.join("FiraSans-Regular.ttf");
		let factory = w.factory.borrow().clone();
		let glyphs = Glyphs::new(font, factory).unwrap();
		
		GameObj{ 
			state_p1: 1, state_p2: 1, input_lock: false,
			hide_p1: false, hide_p2: true,
			scissors_sprite: scissors, rock_sprite: rock, paper_sprite: paper, unknown_sprite: unknown,
			t:0,
			font: glyphs,
			ai_activated: true,
			ai: AiData::new(),
		}
	}
}

impl Drawable for GameObj {
	#[allow(unused_variables)] //mouse
	fn draw (&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, mouse: [f64;2])  -> Option<DrawRequest> { 
		//Background, grey
		let color = [0.7, 0.7, 0.7, 1.0];
		rectangle(color, [0.0, 0.0, w, h], view, g);
		
		//the size which we want the image to be at most
		let tool_width = w / 4.0;
		let tool_height = h / 2.0; 
		
		let x1 = w / 8.0;
		let y = h / 4.0;
		let x2 = 3.0 * x1 + tool_width;
		
		// p1
		if self.hide_p1 {
			let (sprite_w, sprite_h) = self.unknown_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale {scale = max_scale;}
			image(&(self.unknown_sprite), view.trans(x1, y).scale(scale,scale), g);

		}
		else if self.state_p1 == 1{
			let (sprite_w, sprite_h) = self.scissors_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale { scale = max_scale;}
			image(&(self.scissors_sprite), view.trans(x1, y).scale(scale,scale), g);
		}
		else if self.state_p1 == 2 {
			let (sprite_w, sprite_h) = self.rock_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale { scale = max_scale;}
			image(&(self.rock_sprite), view.trans(x1, y).scale(scale,scale), g);
		}
		else /*if self.state_p1 == 3*/{
			let (sprite_w, sprite_h) = self.paper_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale { scale = max_scale;}
			image(&(self.paper_sprite), view.trans(x1, y).scale(scale,scale), g);
		}
		
		// p2
		if self.hide_p2 {
			let (sprite_w, sprite_h) = self.unknown_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale { scale = max_scale;}
			image(&(self.unknown_sprite), view.trans(x2, y).scale(scale,scale), g);

		}
		else if self.state_p2 == 1{
			let (sprite_w, sprite_h) = self.scissors_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale {scale = max_scale;}
			image(&(self.scissors_sprite), view.trans(x2, y).scale(scale,scale), g);
		}
		else if self.state_p2 == 2 {
			let (sprite_w, sprite_h) = self.rock_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale {scale = max_scale;}
			image(&(self.rock_sprite), view.trans(x2, y).scale(scale,scale), g);
		}
		else /*if self.state_p1 == 3*/{
			let (sprite_w, sprite_h) = self.paper_sprite.get_size();
			let max_scale = tool_width/(sprite_w as f64);
			let mut scale = tool_height/(sprite_h as f64);
			if scale > max_scale { scale = max_scale;}
			image(&(self.paper_sprite), view.trans(x2, y).scale(scale,scale), g);
		}
		
		//time counter
		let max_size = w/8.0 ;
		let mut font_size = h/4.0 ;
		if  font_size > max_size { font_size = max_size;} 
		//let font_size_to_print = Text::new(font_size);
		let x = (w - font_size) / 2.0;
		let y = h / 2.0;
		
		text::Text::new_color([0.0,1.0, 0.0, 1.0], (font_size * 1.5) as u32).draw(
					&self.t.to_string(), 
					&mut self.font, 
					&draw_state, 
					view.trans(x, y), g
				);
		None
	}
}

impl PersistentWinnerState for GameObj {
	fn get_winner(&self) -> u8 {
		if 
			self.state_p1 == 1 && self.state_p2 == 3
			|| self.state_p1 == 2 && self.state_p2 == 1
			|| self.state_p1 == 3 && self.state_p2 == 2
			{1}
			
		else if
			self.state_p2 == 1 && self.state_p1 == 3
			|| self.state_p2 == 2 && self.state_p1 == 1
			|| self.state_p2 == 3 && self.state_p1 == 2
			{2}
			
		else 
			{0}
	}
	fn set_visibility(&mut self, p1: bool, p2: bool){
		self.hide_p1 = !p1;
		self.hide_p2 = !p2;
	}
	fn set_time(&mut self, time: u8){
		self.t = time;
	}
	fn lock_input(&mut self, b: bool){
		self.input_lock = b;
	}
}

impl AbsolutelyChangeableState for GameObj {
	fn change_state_p1(&mut self, s: u32){
		if !self.input_lock{
			self.state_p1 = s;
		}
	}
	fn change_state_p2(&mut self, s: u32){
		if !self.input_lock{
			self.state_p2 = s;
		}
	}
}

impl AI for GameObj {
	fn activate_ai(&mut self, b: bool) { 
		self.ai_activated = b;
	}
	fn make_ai_turn(&mut self) {
		self.state_p2 = self.ai.new_turn();
	}
	fn save_turn(&mut self){
		self.ai.turn_result(self.state_p1, self.state_p2);
	}
}


struct AiData{
	rng: rand::ThreadRng,
	logp1: Vec<u32>,
	logp2: Vec<u32>,
}

impl AiData{
	pub fn new() -> AiData {
		AiData {
			rng: rand::thread_rng(),
			logp1: Vec::new(),
			logp2: Vec::new(),
		}
	}
	pub fn new_turn(&mut self) -> u32 {
		
		let mut prob = [0,0,0];
		let rn = self.rng.gen::<u32>() % 12;
		
		// if player fell asleep and doesn't change his turns, abuse it!
		let (turn, count) = self.last_repeated_turns();
		if count <= 12 { prob[turn as usize -1] = count;}
		else { prob[turn as usize -1 ] = 12;}
		
		let mut n = rn;
		while prob[0]+prob[1]+prob[2] < 12 {
			prob[(n%3) as usize] += 1;
			n += 1;
		}
		
		if rn < prob[0] { 2 }
		else if rn < prob[0] + prob[1] {3}
		else {1}
		
		
	}
	fn last_repeated_turns(&self) -> (u32, u32){
		let mut turn = 1;
		let mut count = 0;
		let size = self.logp1.len();
		if size > 0 { 
			turn = self.logp1[size-1];
			count = 1;
			while self.logp1[size-count] == turn && count < size {count += 1;}
		}
			
		(turn, count as u32)
	}
	
	pub fn turn_result(&mut self, player1_turn: u32, player2_turn: u32){
		//Maybe limit size, for instance halve whenever 1kB is used
		self.logp1.push(player1_turn);
		self.logp2.push(player2_turn);
	}
}