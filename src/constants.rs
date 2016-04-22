//! Contains constants used in the game to balance it. At the moment there is only one difficulty, but maybe there will be more later.


//General
pub const STD_FONT_SIZE: u32 = 20;

// Map
pub const CONCRETE_PRICE: [u32;4] = [0,0,3,0];
pub const IRON_FACTORY_PRICE: [u32;4] = [0,10,5,0];
pub const IRON_FACTORY_UPGRADE_PRICE: [u32;4] = [1,0,1,0];
pub const UNIVERSITY_PRICE: [u32;4] = [5,5,5,5];

// Defence
	//General
	pub const BATTLEFIELD_W: f64 = 600.0;
	pub const BATTLEFIELD_H: f64 = 1000.0;

	pub const BF_SHOP_SPLIT_RATIO: f64 = 0.875;
	
	// Enemy

		//ids
		pub const BASIC_EID: usize = 0;
		
		// Speed list
		pub const ENEMY_SPEED: [f64; 3] = [10.0, 20.0, 30.0 ];
		
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
				[0,5,0,0],
			];
		