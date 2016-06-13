//! Contains constants used in the game to balance it. At the moment there is only one difficulty, but maybe there will be more later.


// UI (ALL UI elements should be here, in case different resolutions are supported in the future, this could help a lot.)
//pub const GENERAL_SCALE_FACTOR: f64 = 3.0;

//pub const STD_FONT_SIZE: u32 = 20; //* GENERAL_SCALE_FACTOR as u32;
//pub const TITLE_FONT_SIZE: u32 = 60;// * GENERAL_SCALE_FACTOR as u32;
//pub const BATTLEFIELD_UI_SCALE: f64 = 1.0 * GENERAL_SCALE_FACTOR;

pub const HEALTH_BAR_HEIGHT: f64 = 8.0;

//Meta
/*pub const START_RESSOURCES: [u32;4] = [0,0,0,0];
pub const STARTING_LIFES: u32 = 50;

pub const START_TIME_PHASE_2: f64 = 15.0;
pub const START_TIME_PHASE_3: f64 = 30.0;
pub const START_TIME_PHASE_4: f64 = 120.0;
pub const PHASE_SWITCH_TIME: f64 = 2.0;*/

/**/
pub const START_RESSOURCES: [u32;4] = [500,500,500,500];
pub const STARTING_LIFES: u32 = 30;
pub const START_TIME_PHASE_2: f64 = 5.0;
pub const START_TIME_PHASE_3: f64 = 10.0;
pub const START_TIME_PHASE_4: f64 = 15.0;
pub const PHASE_SWITCH_TIME: f64 = 1.0;/**/

pub const EPS: f64 = 1.0/1048576.0;

// Map
pub const CONCRETE_PRICE: [u32;4] = [0,0,3,0];

pub const IRON_FACTORY_PRICE: [u32;4] = [0,10,5,0];
pub const IRON_FACTORY_UPGRADES: usize = 10;
pub const IRON_FACTORY_UPGRADE_PRICE: [[u32;4]; IRON_FACTORY_UPGRADES] = 
	[
		[0,3,0,1],
		[0,5,0,2],
		[0,7,0,3],
		[0,8,0,5],
		[0,10,0,7],
		[0,13,0,8],
		[0,15,0,10],
		[0,25,0,15],
		[0,30,0,20],
		[0,40,0,25],
	];

pub const UNIVERSITY_PRICE: [u32;4] = [2,2,2,2];
pub const UNIVERSITY_UPGRADES: usize = 3;
pub const UPGRADE_UNIVERSITY_PRICE: [[u32;4]; UNIVERSITY_UPGRADES] = 
	[	
		[5,0,0,1],
		[8,0,0,3],
		[15,0,0,10],
	];
pub const BLACKSMITH_PRICE: [u32;4] = [0,5,5,0];
pub const BLACKSMITH_II_PRICE: [u32;4] = [0,5,8,0];
pub const BARRACKS_PRICE: [u32;4] = [0,5,5,0];
pub const ARCHERY_RANGE_PRICE: [u32;4] = [0,8,5,0];

pub const BANK_PRICE: [u32;4] = [5,5,0,0];
pub const BANK_UPGRADES: usize = 5;
pub const BANK_UPGRADE_PRICE: [[u32;4]; BANK_UPGRADES] = 
	[
		[5,0,0,5],
		[10,0,0,5],
		[15,0,0,10],
		[25,0,0,10],
		[35,0,0,20],
	];

pub const ORACLE_PRICE: [u32;4] = [0,0,0,3];
	
// UPGRADES / RESEARCHES
pub const INDUSTRIALISATION_PRICE: [u32;4] = [0,0,10,0];
pub const ECONOMY_RESEARCH_PRICE: [u32;4] = [5,0,10,0];
pub const RESEARCH_TOWER_PRICE_LIST: [[u32;4]; NUMBER_OF_TOWERS] = 
	[
		[0,0,0,0],
		[0,5,0,0],
		[0,0,5,0],		
		[5,5,5,0],		
		[10,5,10,5],		
	];
pub const ORACLE_RESEARCH_LEVELS: usize = 8;
pub const ORACLE_RESEARCH_PRICE_LIST: [[u32;4]; ORACLE_RESEARCH_LEVELS] = 
	[
		[1,1,1,1],
		[2,2,2,2],
		[3,3,3,3],
		[5,5,5,5],
		[8,8,8,5],
		[12,12,12,5],
		[15,15,15,8],
		[20,20,20,10],
	];	
	
// Defence
	//General
	pub const BATTLEFIELD_W: f64 = 600.0;
	pub const BATTLEFIELD_H: f64 = 1000.0;
	pub const BASE_X: f64 = 200.0;
	pub const BASE_Y: f64 = 800.0;
	pub const BASE_W: f64 = 200.0;
	pub const BASE_H: f64 = 200.0;
	
	pub const BF_SHOP_SPLIT_RATIO: f64 = 0.875;
	
	// General Sprites
	pub const GENERAL_BATTLEFIELD_SPRITE_LIST: [&'static str; 4] = ["highway_from_hell.png", "cross.png", "delete.png", "explosion.png"];
	
	// Enemy

		//ids
		pub const NUMBER_OF_ES: usize = 3;
		pub const BASIC_EID: usize = 0;
		pub const SLOW_EID: usize = 1;
		pub const FAST_EID: usize = 2;
		
		// Speed list
		pub const ENEMY_SPEED: [f64; 3] = [25.0, 70.0, 110.0 ];
		
		// Stats for each enemy type
		pub const ENEMY_ATTACK: [f64; NUMBER_OF_ES] = [7.0, 5.0, 10.0];
		pub const ENEMY_ATTACK_SCALE: [f64; NUMBER_OF_ES] = [2.0, 1.0, 3.0];
		pub const ENEMY_ATTACK_RATIO: [f64; NUMBER_OF_ES] = [0.5, 0.3, 0.7];
		
		pub const ENEMY_HEALTH: [f64; NUMBER_OF_ES] = [30.0, 10.0, 60.0];
		pub const ENEMY_HEALTH_SCALE: [f64; NUMBER_OF_ES] = [30.0, 20.0, 50.0];
		
		//Sprite constants	
		
		pub const ENEMY_SPRITE_LIST: [&'static str; NUMBER_OF_ES] = ["enemy_i.png", "enemy_ii.png", "enemy_iii.png"];
		
		
		//size
		pub const STD_ENEMY_W: f64 = 50.0;
		pub const STD_ENEMY_H: f64 = 50.0;	
		
	// Tower
		
		//ids
		pub const BASIC_TID: usize = 0;
		pub const AOE_TID: usize = 1;
		pub const WALL_TID: usize = 2;
		pub const SLOW_TID: usize = 3;
		pub const ROCKET_TID: usize = 4;
		
		pub const NUMBER_OF_TOWERS: usize = 5;
		pub const TOWER_SPRITE_LIST: [&'static str; NUMBER_OF_TOWERS] =["jar.png", "box.png", "fence.png", "cotton_candy.png", "surprise.png"];
		pub const TOWER_PRICE_LIST: [[u32;4];NUMBER_OF_TOWERS] = [
				[2,0,2,0],
				[0,4,0,0],
				[0,0,5,0],
				[5,0,5,1],
				[5,5,10,4],
			];
		pub const TOWER_SIZE_LIST: [(f64,f64);NUMBER_OF_TOWERS] = [(70.0,100.0),(75.0,100.0),(75.0,50.0), (55.0, 80.0), (100.0,140.0) ];
		pub const TOWER_BASE_HEALTH_LIST: [f64;NUMBER_OF_TOWERS] = [60.0, 100.0, 200.0, 100.0, 150.0];
		pub const TOWER_BASE_ATTACK_RATIO_LIST: [f64;NUMBER_OF_TOWERS] = [1.0, 1.5, 100.0, 0.75, 1.0];
		pub const TOWER_BASE_ATTACK_LIST: [f64;NUMBER_OF_TOWERS] = [10.0, 8.0, 0.0, 0.0, 25.0];
		pub const TOWER_BASE_RANGE_LIST: [f64;NUMBER_OF_TOWERS] = [300.0, 140.0, 0.0, 200.0, 275.0];
		
		pub const EXPLOSION_BASE_RADIUS: f64 = 20.0;
		pub const EXPLOSION_VISIBILITY_TIME: f64 = 0.4;
		
	// Projectile
		pub const PROJECTILE_SPRITE_LIST: [&'static str; 3] = ["projectile_i.png", "cotton_candy_projectile.png", "surprise_projectile.png"];
		pub const PROJECTILE_VELOCITY: f64 = 1000.0;
		pub const SLOW_PROJECTILE_VELOCITY: f64 = 100.0;
		pub const BASIC_PROJECTILE_SIZE: (f64,f64) = (20.0,10.0);
		pub const SLOW_PROJECTILE_SIZE: (f64,f64) = (40.0,20.0);
		pub const ROCKET_PROJECTILE_SIZE: (f64,f64) = (40.0,20.0);
		
// Constant functions to determine game behaviour

/** Defines how the resource bonuses granted by the Oracle researches are calculated.

Currently in use: 1 + x^1.5

* 0 -> 1 => 1
* 1 -> 2 => 2
* 2 -> 3.8 => 4
* 3 -> 6.19 => 6
* 4 -> 9 => 9
* 5 -> 12.18 => 12
* 6 -> 15.69 => 16
* 7 -> 19.52 => 20
* 8 -> 23.63 => 24
*/
pub fn apply_bonus (res: u32, upgrade_level: u32) -> u32 {
	 res * (1 + ((upgrade_level as f32 * (upgrade_level as f32).sqrt() ) + 0.5) as u32)
}

/// Defines how much each standard upgrade level for towers cost
pub fn tower_upgrade_cost (lvl: u32) -> [u32;4] {
	[
		5 + lvl * lvl,
		5 + 3 * lvl,
		5 + 2 * lvl * lvl ,
		lvl
	]
}
