/*
<<< micro >>
*/

//! Contains all mini games which belong to the category micro. This category includes the games which can be played from the very beginning and do not require any resources to play.

pub mod rock_paper_scissors;
pub mod tic_tac_toe;

/// These games can decide the winner at any arbitary moment in time. This means, rounds should be handled outside of the game.
pub trait PersistentWinnerState {
	fn get_winner(&self) -> u8;
	fn set_visibility(&mut self, p1: bool, p2: bool);
	/// Sets the time that's left before the round is finshed. This is used only to display the time when drawing the game, it does not affect the game behavior.
	fn set_time(&mut self, time: u8);
	fn lock_input(&mut self, b: bool);
}

/// Games that have a single value for each player, representing the entire game state. Furthermore, the state can be changed absolutely, which means there are no restrictions on the new state depending on the old state.
pub trait AbsolutelyChangeableState {
	fn change_state_p1(&mut self, s: u32);
	fn change_state_p2(&mut self, s: u32);
}

/// Games that need an AI to play against the player. The AI has to be called once each round to make its turn and then again to take note of the result.
pub trait AI {
	fn activate_ai(&mut self, b: bool);
	/// Must be called before evluating the turn
	fn make_ai_turn(&mut self);
	/// Must be called after make_turn, but before the new turn begins. Ideally make the game state immutable before calling this function and then evaluate it. This ensures the AI sees the correct state but it can't cheat.
	fn save_turn(&mut self);
}

/// Games that need to check for click events.
pub trait ClickableGame {
	/// x and y are relative to the games position
	/// optionally returns a reward caused by the click
	fn click (&mut self, x: f64, y: f64) -> Option <[u32;4]>;
}
