//! Contains constants used in the game to balance it. At the moment there is only one difficulty, but maybe there will be more later.


// UI (ALL UI elements should be here, in case different reoultions are supported in the future, this could help a lot.)
pub const STD_FONT_SIZE: u32 = 20;
pub const HEALTH_BAR_HEIGHT: f64 = 8.0;
pub const BATTLEFIELD_UI_SCALE: f64 = 1.0;

//Meta
pub const START_RESSOURCES: [u32;4] = [20,20,20,20];
pub const EPS: f64 = 1.0/1048576.0;

// Map
pub const CONCRETE_PRICE: [u32;4] = [0,0,3,0];
pub const IRON_FACTORY_PRICE: [u32;4] = [0,10,5,0];
pub const IRON_FACTORY_UPGRADE_PRICE: [u32;4] = [1,0,1,0];
pub const UNIVERSITY_PRICE: [u32;4] = [5,5,5,5];

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
	
	pub const GENERAL_BATTLEFIELD_SPRITE_LIST: [&'static str; 2] = ["highway_from_hell.png", "cross.png"];
	
	// Enemy

		//ids
		pub const BASIC_EID: usize = 0;
		
		// Speed list
		pub const ENEMY_SPEED: [f64; 3] = [25.0, 40.0, 75.0 ];
		
		// Stats for each enemy type
		pub const STD_ENEMY_ATTACK: f64 = 7.0;
		pub const STD_ENEMY_ATTACK_RATIO: f64 = 0.5;
		
		//Sprite constants	
		
		pub const ENEMY_SPRITE_LIST: [&'static str; 1] = ["enemy_i.png"];
		pub const NUMBER_OF_ES: usize = 1;
		
		//size
		pub const STD_ENEMY_W: f64 = 50.0;
		pub const STD_ENEMY_H: f64 = 50.0;	
		
	// Tower
		
		//ids
		pub const BASIC_TID: usize = 0;
		pub const AOE_TID: usize = 1;
		
		
		pub const DEFAULT_TOWER_W: f64 = 75.0;
		pub const DEFAULT_TOWER_H: f64 = 100.0;
		
		pub const NUMBER_OF_TOWERS: usize = 2;
		pub const TOWER_SPRITE_LIST: [&'static str; NUMBER_OF_TOWERS] =["jar.png", "box.png"];
		pub const TOWER_PRICE_LIST: [[u32;4];NUMBER_OF_TOWERS] = [
				[2,0,0,2],
				[0,4,0,0],
			];
		pub const TOWER_BASE_HEALTH_LIST: [f64;2] = [60.0, 200.0];
		pub const TOWER_BASE_ATTACK_RATIO_LIST: [f64;2] = [1.0, 1.0];
		pub const TOWER_BASE_ATTACK_LIST: [f64;2] = [5.0, 10.0];
		
	// Projectile
		pub const PROJECTILE_SPRITE_LIST: [&'static str; 2] = ["projectile_i.png", "projectile_i.png"];
		pub const PROJECTILE_VELOCITY: f64 = 1000.0;
		pub const PROJECTILE_SIZE: (f64,f64) = (40.0,20.0);