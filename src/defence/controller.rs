//! This module is responsible for scheduling the enemy waves.

use super::enemy::Enemy;
use super::enemy::basic_enemy::BasicEnemy;
use super::enemy::slow_enemy::SlowEnemy;
use super::enemy::fast_enemy::FastEnemy;

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
				if !self.basic_enemies(vec, 20, 20.0, 2.0){ 
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
				if !self.fast_enemies(vec, 10, 20.0, 1.0){ 
						self.state = START_SHORT_BREAK ;
				}
			},
			20 => {
				if !self.fast_enemies(vec, 10, 20.0, 2.0){ 
						self.state = START_NORMAL_BREAK ;
						self.loop_counter = 2;
				}
			},
			
			// End loop, spwaning the same waves over and over again with increasing strength
			21 => {
				let level = self.loop_counter as f64;
				if !self.basic_enemies(vec, 15, 20.0, level){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			22 => {
				let level = self.loop_counter as f64;
				if !self.slow_enemies(vec, 15, 20.0, level){ 
						self.state = START_NORMAL_BREAK ;
				}
			},
			23 => {
				let level = self.loop_counter as f64;
				if !self.fast_enemies(vec, 15, 15.0, level){ 
						self.state = START_NORMAL_BREAK ;
						self.loop_counter += 1;
						self.highest_state = 20; // going to 21 after the break
				}
			},
			
			
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
			let new_enemy = Box::new(FastEnemy::new( self.spawn_x, self.spawn_y, level) );
			vec.push(new_enemy);
			self.unit_counter += 1;
		}
		true
	}
}

// Constantsv for this module
const INITIAL_PEACE_TIME: f64 = 30.0;
const LONG_BREAK_BETWEEN_WAVES: f64 = 60.0;
const NORMAL_BREAK_BETWEEN_WAVES: f64 = 30.0;
const SHORT_BREAK_BETWEEN_WAVES: f64 = 15.0;
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
