//! This module is responsible for scheduling the enemy waves.

use super::enemy::Enemy;
use super::enemy::basic_enemy::BasicEnemy;

pub struct Ctrl {
	spawn_x: f64, spawn_y: f64,
	clock: f64, state: u32,
}

impl Ctrl {
	pub fn new(spawn_x: f64, spawn_y: f64) -> Ctrl {
		Ctrl{
			spawn_x: spawn_x, spawn_y: spawn_y,
			clock: 0.0, state: 0,
			}
	}
	
	pub fn update(&mut self, dt: f64, vec: &mut Vec<Box<Enemy>>) {
		self.clock += dt;
		match self.state {
			0 => { if self.clock > 10.0 {self.state = 1;} },
			1 => { self.clock = 0.0; self.state = 2; }
			_ => { self.level_one(vec); }
		}
	}
	
	fn level_one(&mut self, vec: &mut Vec<Box<Enemy>>) {
		if self.clock > 4.0 {
			self.clock = 0.0;
			let new_enemy = Box::new(BasicEnemy::new( self.spawn_x, self.spawn_y) );
			vec.push(new_enemy);
		}
	}
}