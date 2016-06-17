//! This module is responsible for scheduling the enemy waves.

use super::enemy::Enemy;
use super::enemy::basic_enemy::BasicEnemy;
use super::enemy::slow_enemy::SlowEnemy;
use super::enemy::alien_enemy::Alien;
use super::enemy::aggressive_enemy::AggressiveEnemy;
use super::enemy::alien_boss::AlienBoss;
use super::enemy::devil_enemy::Devil;
use super::enemy::devil_boss::DevilBoss;
use super::enemy::ghost_enemy::Ghost;
use super::enemy::ghost_boss::GhostBoss;
use super::enemy::troll::Troll;

pub struct Ctrl {
	spawn_x: f64, spawn_y: f64,
	clock: f64, peace_clock: f64,
	state: u32, highest_state: u32,
	unit_counter: u32,
	loop_counter: u32, 
}

impl Ctrl {
	pub fn new(spawn_x: f64, spawn_y: f64) -> Ctrl {
		Ctrl{
			spawn_x: spawn_x, spawn_y: spawn_y,
			clock: 0.0, peace_clock: 0.0,
			state: 0, highest_state: 1,
			unit_counter: 0, 
			loop_counter: 0,
			}
	}
	
	pub fn update(&mut self, dt: f64, vec: &mut Vec<Box<Enemy>>) {
		
		if self.state > self.highest_state { self.highest_state = self.state; } 
		self.clock += dt;
		
		match self.state {
			// initial state
			0 => { 
				if self.clock > INITIAL_PEACE_TIME {
					self.state = FIRST_WAVE;
				} 
			},
			
			// start short/normal/long peace phasees, can be called over and over again
			// after the break phase, the state will go to the lowest unplayed wave
			START_SHORT_BREAK => { 
				self.clock = 0.0; 
				self.peace_clock = SHORT_BREAK_BETWEEN_WAVES;
				self.state = BREAK_STATE; 
			},
			START_NORMAL_BREAK => { 
				self.clock = 0.0; 
				self.peace_clock = NORMAL_BREAK_BETWEEN_WAVES;
				self.state = BREAK_STATE; 
			},
			START_LONG_BREAK => { 
				self.clock = 0.0; 
				self.peace_clock = LONG_BREAK_BETWEEN_WAVES;
				self.state = BREAK_STATE; 
			},
			BREAK_STATE => {
				if self.clock >= self.peace_clock {
					self.clock = 0.0;
					self.state = self.highest_state + 1;
					self.unit_counter = 0;
				}
			},
			
			// Recovery state
			RECOVERY_STATE => {
				self.state = START_SHORT_BREAK;
			},
			
			// Wave states
			FIRST_WAVE => {
				if !self.basic_enemies(vec, 2, 10.0, 0.0)
					{ self.state = START_NORMAL_BREAK ;}
			},
			// FIRST_WAVE + 1 = 11
			11 => {
				if !self.basic_enemies(vec, 5, 15.0, 0.0)
					{ self.state = START_NORMAL_BREAK ;}
			},
			12 => {
				if !self.slow_enemies(vec, 5, 20.0, 0.0)
					{ self.state = START_LONG_BREAK ;}
			},
			13 => {
				if !self.basic_enemies(vec, 5, 15.0, 1.0){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			14 => {
				if !self.basic_enemies(vec, 10, 20.0, 1.0){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			15 => {
				if !self.fast_enemies(vec, 5, 5.0, 0.0){ 
						self.state = START_LONG_BREAK ;
				}
			},
			16 => {
				if !self.basic_enemies(vec, 10, 20.0, 2.0){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			17 => {
				if !self.slow_enemies(vec, 10, 15.0, 1.0){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			18 => {
				if !self.slow_enemies(vec, 12, 15.0, 2.0){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			19 => {
				if !self.fast_enemies(vec, 6, 20.0, 1.0){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			20 => {
				if !self.basic_enemies(vec, 10, 20.0, 3.0){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			21 => {
				if !self.fast_enemies(vec, 10, 20.0, 2.0){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			22 => {
				if !self.aggressive_enemies(vec, 5, 10.0, 0.0){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			23 => {
				if !self.slow_enemies(vec, 15, 20.0, 2.0){ 
						self.state = START_LONG_BREAK ;
				}
			},
			24 => {
				self.alien_boss(vec, 0.0);
				self.state = START_NORMAL_BREAK ;
			},
			
			// End loop, spwaning the same waves over and over again with increasing strength
			// More waves are added after the first few loop iterations
			25 => {
				let level = (self.loop_counter + 2) as f64;
				let number = 15 + 2 * self.loop_counter;
				if !self.basic_enemies(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			26 => {
				let level = (self.loop_counter + 2) as f64;
				let number = 15 + 2 * self.loop_counter;
				if !self.slow_enemies(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			27 => {
				let level = (self.loop_counter + 2) as f64;
				let number = 15 + 2 * self.loop_counter;
				if !self.fast_enemies(vec, number, 15.0, level){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			28 => {
				let level = self.loop_counter as f64;
				let number = 15 + 2 * self.loop_counter;
				if !self.aggressive_enemies(vec, number, 15.0, level){ 
						self.state = START_NORMAL_BREAK ;	
				}
			},
			29 => {
				let level = self.loop_counter as f64;
				let number = 5 + self.loop_counter;
				let time = if self.loop_counter < 10 { 5.0 + self.loop_counter as f64} else { 15.0 };
				if !self.alien_gang(vec, number, time, level) {
					self.state = START_SHORT_BREAK ;
					if self.loop_counter == 0 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
				}
			},
			// devils
			30 => {
				let level = (self.loop_counter - 1) as f64;
				let number = 10 + 2 * self.loop_counter;
				if !self.devils(vec, number, 15.0, level){ 
						self.state = START_SHORT_BREAK ;	
				}
			},
			31 => {
				let level = (self.loop_counter - 1) as f64;
				self.devil_boss(vec, level);
				self.state = START_NORMAL_BREAK ;
				if self.loop_counter == 1 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
			},
			32 => {
				let level = (self.loop_counter - 2) as f64;
				let number = 5 + self.loop_counter;
				let time = if self.loop_counter < 10 { 5.0 + self.loop_counter as f64} else { 15.0 };
				if !self.devil_gang(vec, number, time, level) {
					self.state = START_SHORT_BREAK ;
					if self.loop_counter == 2 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
				}
			},
			// ghosts
			33 => {
				let level = (self.loop_counter - 3) as f64;
				let number = 8 + 2 * self.loop_counter;
				if !self.ghosts(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;	
				}
			},
			34 => {
				let level = (self.loop_counter - 3) as f64;
				self.ghost_boss(vec, level);
				self.state = START_NORMAL_BREAK ;
				if self.loop_counter == 3 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
			},
			35 => {
				let level = (self.loop_counter - 4) as f64;
				let number = 5 + self.loop_counter;
				let time = if self.loop_counter < 15 { 5.0 + (self.loop_counter - 4) as f64} else { 15.0 };
				if !self.ghost_gang(vec, number, time, level) {
					self.state = START_SHORT_BREAK ;
					if self.loop_counter == 4 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
				}
			},
			// Trolls
			36 => {
				let level = (self.loop_counter - 5) as f64;
				self.troll(vec, level);
				self.state = START_SHORT_BREAK ;
				if self.loop_counter == 5 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
			},
			37 => {
				let level = (self.loop_counter + 3) as f64;
				let number = 30 + 1 * self.loop_counter;
				if !self.slow_enemies(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			38 => {
				let level = (self.loop_counter - 6) as f64;
				let number = 8 + 2 * self.loop_counter;
				if !self.troll_and_aggressive_enemies(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;	
						if self.loop_counter == 7 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
				}
			},
			39 => {
				let level = self.loop_counter  as f64;
				let number = 30 + 1 * self.loop_counter;
				if !self.devils(vec, number, 20.0, level){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			40 => {
				let level = (self.loop_counter - 4) as f64;
				let number = 2 * self.loop_counter;
				if !self.troll_mix(vec, number, 20.0, level){ 
						self.state = START_SHORT_BREAK ;	
						if self.loop_counter == 8 {self.loop_counter += 1; self.highest_state = 24;} // going to 25 after the break
				}
			},
			// starting endless loop without changes other than strength
			41 => {
				self.loop_counter += 1; 
				self.state = 25; // go to 25 now
				self.highest_state = 25; // going to 26 after the next break
			}
			_ => { println!("unexpected state: {}", self.state); self.state = RECOVERY_STATE; }
		}
		
	}
	
	// Wave types
	// All waves return true as long as they are generating units and false as soon as they are done.
	// Before calling a wave the first time, the unit_counter field should be zero. Afterwards the field should not be changed outside of the wave function.
	
	fn basic_enemies(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(BasicEnemy::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn slow_enemies(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(SlowEnemy::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn fast_enemies(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Alien::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn aggressive_enemies(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(AggressiveEnemy::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn alien_boss(&mut self, vec: &mut Vec<Box<Enemy>>, level: f64) {
		let new_enemy = Box::new(AlienBoss::new( self.spawn_x, self.spawn_y, level) );
		vec.push(new_enemy);
	}
	// Sending an alien boss and n aliens (fast enemies)
	fn alien_gang(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if self.unit_counter == 0 { 
			let new_enemy = Box::new(AlienBoss::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		if n <= self.unit_counter + 1 { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Alien::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn devils(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Devil::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn devil_boss(&mut self, vec: &mut Vec<Box<Enemy>>, level: f64) {
		let new_enemy = Box::new(DevilBoss::new( self.spawn_x, self.spawn_y, level) );
		vec.push(new_enemy);
	}
	fn devil_gang(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if self.unit_counter == 0 { 
			let new_enemy = Box::new(DevilBoss::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		if n <= self.unit_counter + 1 { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Devil::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn ghosts(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { return false }
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Ghost::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn ghost_boss(&mut self, vec: &mut Vec<Box<Enemy>>, level: f64) {
		let new_enemy = Box::new(GhostBoss::new( self.spawn_x, self.spawn_y, level) );
		vec.push(new_enemy);
	}
	fn ghost_gang(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { 
			if self.clock >= t / n as f64 {
				let new_enemy = Box::new(GhostBoss::new( self.spawn_x, self.spawn_y, level) );
				vec.push(new_enemy);
				return false;
			}
		}
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(Ghost::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	
	fn troll(&mut self, vec: &mut Vec<Box<Enemy>>, level: f64) {
		let new_enemy = Box::new(Troll::new( self.spawn_x, self.spawn_y, level) );
		vec.push(new_enemy);
	}
	// n aggressive enemies and one troll
	fn troll_and_aggressive_enemies(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { 
			if self.clock >= t / n as f64 {
				let new_enemy = Box::new(Troll::new( self.spawn_x, self.spawn_y, level) );
				vec.push(new_enemy);
				return false ;
			}
		}
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy = Box::new(AggressiveEnemy::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
	// one troll, n/2 basic enemies and n/2 slow enemies
	fn troll_mix(&mut self, vec: &mut Vec<Box<Enemy>>, n: u32, t: f64, level: f64) -> bool {
		if n <= self.unit_counter { 
			if self.clock >= t / n as f64 {
				let new_enemy = Box::new(Troll::new( self.spawn_x, self.spawn_y, level) );
				vec.push(new_enemy);
				return false;
			}
		}
		if self.clock >= t / n as f64 {
			self.clock = 0.0;
			let new_enemy: Box<Enemy>;
			if self.unit_counter % 2 == 0 {
				new_enemy = Box::new(BasicEnemy::new( self.spawn_x, self.spawn_y, level) );
			}
			else {
				new_enemy = Box::new(SlowEnemy::new( self.spawn_x, self.spawn_y, level) );
			}
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
}

// Constantsv for this module
/**/const INITIAL_PEACE_TIME: f64 = 30.0;
const LONG_BREAK_BETWEEN_WAVES: f64 = 60.0;
const NORMAL_BREAK_BETWEEN_WAVES: f64 = 30.0;
const SHORT_BREAK_BETWEEN_WAVES: f64 = 15.0;/**/
 /*Debugging:*//*
const INITIAL_PEACE_TIME: f64 = 3.0;
const LONG_BREAK_BETWEEN_WAVES: f64 = 6.0;
const NORMAL_BREAK_BETWEEN_WAVES: f64 = 3.0;
const SHORT_BREAK_BETWEEN_WAVES: f64 = 1.5;
*/
	//states
const START_SHORT_BREAK: u32 = 1;
const START_NORMAL_BREAK: u32 = 2;
const START_LONG_BREAK: u32 = 3;
const BREAK_STATE: u32 = 4;
const RECOVERY_STATE: u32 = 8;
const FIRST_WAVE: u32 = 10;
