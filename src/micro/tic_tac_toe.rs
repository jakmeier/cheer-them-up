//! Tic-Tac-Toe:
//! Whenever the player makes a turn, the filed he played on will be marked.
//! A fully marked board will grant extra resources.
//! A lost game resets the lmarks on the board.
//! A win also grants extra resources.
//! What resources will be rewarded can be set in the constructor.

use super::{AI, ClickableGame};
use definitions::Drawable;
use super::super::piston_window::*;
use super::super::gfx_device_gl::Resources;
use super::super::gfx_device_gl::command::CommandBuffer;
use super::super::gfx_graphics::GfxGraphics;
use super::super::find_folder;

use rand;

pub struct TicTacToeData {
	w: f64, h: f64,
	board: [(u8,bool);9],
	win_reward: [u32;4],
	full_board_reward: [u32;4],
	ai: bool, locked: f64, players_turn: bool,
	x_sprite: Texture<Resources>,
	o_sprite: Texture<Resources>,
}

impl TicTacToeData {
	/// The coordinates do not need to be initilazed here, this will happen in the function draw() anyway and since we cannot click the game before we have drawn it, I will leave it undefined here.
	pub fn new(w: &PistonWindow, win: [u32;4], full: [u32;4]) -> TicTacToeData {
	
		let img = find_folder::Search::ParentsThenKids(3, 3).for_folder("img").unwrap();
		let folder = img.join("x.png");
		let x = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
		
		let folder = img.join("o.png");
		let o = Texture::from_path(
			&mut *w.factory.borrow_mut(),
			&folder,
			Flip::None,
			&TextureSettings::new()
		)
        .unwrap();
	
		TicTacToeData {
			w: 0.0, h:0.0,
			board: [(0, false);9],
			win_reward: win,
			full_board_reward: full,
			ai: true, locked: 0.0, players_turn: true,
			x_sprite: x,
			o_sprite: o,
		}
	}

	// Checks if the move is valid and executes itif so.
	// Also sets state to locked for one second and changes the player whos turn it is.
	// No checks on the locking state or about win condition done in here!
	fn make_move(&mut self, i: usize, player: bool){
		let (state, marked) =  self.board[i];
		if state == 0 {
			if player && self.players_turn{
				self.board[i] = (1,true);
				self.locked = 1.0;
				self.players_turn = false;
			}
			else if !player && !self.players_turn{
				self.board[i] = (2,marked);
				self.locked = 1.0;
				self.players_turn = true;
			}
		}
	}

	///Checks for the winner, does not reset the board if a player won
	fn check_winner(&mut self) -> bool {
		let a;
		let mut win = false;
		if !self.players_turn {a = 1;}
		else {a = 2;}
		for i in 0..3 {
			let mut row = true;
			for j in 0..3 {
				row = row && self.board[i * 3 + j].0 == a;
			}
			win = win || row;
		}
		for j in 0..3 {
			let mut col = true;
			for i in 0..3 {
				col = col && self.board[i * 3 + j].0 == a;
			}
			win = win || col;
		}
		win = win || (self.board[0].0 == a && self.board[4].0 == a && self.board[8].0 == a );
		win = win || (self.board[2].0 == a && self.board[4].0 == a && self.board[6].0 == a );
		
		win
	}

	///Checks if the board is fully marked and resets it if it is full
	fn check_marks(&mut self) -> bool {
		let mut full = true;
		for i in 0..3 {
			for j in 0..3 {
				full = full && self.board[i * 3 + j].1;
			}
		}
		if full {self.remove_marks();}
		full
	}
	
	fn remove_marks(&mut self){
		for i in 0..9 {
			self.board[i].1 = false;			
		}
	}
	
	///Clears the board whether it is full
	fn check_full_board(&mut self){
		let mut full = true;
		for i in 0..3 {
			for j in 0..3 {
				full = full && self.board[i * 3 + j].0 != 0;
			}
		}
		if full {self.clear_board();}
	}
	
	///Measures time between turns, wait time stored in locked before the next turn happens
	///Also calls ai turn when needed and deletes marks if the player lost.
	pub fn on_update(&mut self, dt: f64){
		if self.locked > 0.0 { 
			self.locked -= dt; 
			if self.locked <= 0.0 {
				self.locked = 0.0;
				if self.check_winner() {
					self.clear_board();
					if self.players_turn {self.remove_marks();}
				}
				self.check_full_board();
				if self.ai && !self.players_turn {self.make_ai_turn();}		
			}			
		}
	}
	
	//Clear board from X and O
	fn clear_board(&mut self){
		for i in 0..9 {
			self.board[i].0 = 0;
		}
	}
	
}

impl Drawable for TicTacToeData {
	#[allow(unused_variables)] //draw state, help
	fn draw(&mut self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d, draw_state: DrawState, w: f64, h:f64, help: bool)
	{
		self.w = w;
		self.h = h;
		let width = w / 3.0;
		let height = h / 3.0;
		
		//Background
		rectangle([0.9, 0.9, 0.9, 1.0], [0.0, 0.0, w, h], view, g);
		
		//Marks + XO
		
		for i in 0..3 {
			for j in 0..3 {
				let mut sprite: Option<&Texture<Resources>> = None;
				let mut scale = 0.0;
				match self.board[i * 3 as usize + j] {
					(0,_) => {}
					(1,_) => { //X
						let (sprite_w, sprite_h) = self.x_sprite.get_size();
						let max_scale = width/(sprite_w as f64);
						scale = height/(sprite_h as f64);
						if scale > max_scale {scale = max_scale;}
						sprite = Some(&(self.x_sprite));
					}
					(2,_) => {//O
						let (sprite_w, sprite_h) = self.o_sprite.get_size();
						let max_scale = width/(sprite_w as f64);
						scale = height/(sprite_h as f64);
						if scale > max_scale {scale = max_scale;}
						sprite = Some(&(self.o_sprite));
					}
					_ => {}
				}
				let (_,marked) = self.board[i * 3 as usize + j];
				if marked {
					rectangle([0.0, 0.4, 0.1, 1.0], [0.0, 0.0, width, height], view.trans(j as f64 * width, i as f64 * height), g);
				}
				
				if let Some(pic) = sprite {
					image(pic, view.trans(j as f64 * width, i as f64 * height).scale(scale,scale), g);
				}
			}
		}
		
		//Board grid
		let color = [0.0, 0.0, 0.0, 1.0];
		let line_strength = self.w * 0.01;
		for i in 0..3 {
			line(color, line_strength, [i as f64 * width, 0.0, i as f64 * width, h], view, g);
		}
		for i in 0..3 {
			line(color, line_strength, [0.0, i as f64 * height, w, i as f64 * height ], view, g);
		}
	}
}

impl ClickableGame for TicTacToeData {
	// coordintes are relative to the board
	fn click (&mut self, x: f64, y: f64) -> Option <[u32;4]> {
		let mut result = None;
		if self.locked == 0.0 && x > 0.0 && y > 0.0 && x < self.w && y < self.h {
			let i = (x / (self.w / 3.0)) as usize;
			let j = (y /(self.h / 3.0)) as usize;
			self.make_move(j * 3 + i, true);
			if self.check_winner() {result = Some(self.win_reward);}
			if self.check_marks() {
				if let Some(reward) = result {result = Some([reward[0]+self.full_board_reward[0], reward[1]+self.full_board_reward[1], reward[2]+self.full_board_reward[2], reward[3]+self.full_board_reward[3]]); }
				else {result = Some(self.full_board_reward);}
			}
		}
		result
	}
}

impl AI for TicTacToeData {
	fn activate_ai(&mut self, b: bool) {self.ai = b;}
	
	fn make_ai_turn(&mut self){
		let rn = rand::random::<usize>();
		for i in  0..9 {
			let index = (i + rn)%9;
			if self.board[index].0 == 0 {self.make_move(index, false); break;}	
		}
	}
	///probably never used in this game
	fn save_turn(&mut self) {
	}
}

