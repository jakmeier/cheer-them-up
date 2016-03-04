extern crate multy_task_lib;
extern crate piston_window;


use piston_window::*;
use multy_task_lib::Game;

fn main() {

	let screen_width = 960;
	let screen_height = 590;
	
	let window: PistonWindow = WindowSettings::new(
        "Multy Task",
        [screen_width, screen_height]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
	
	let mut game = Game::new(&window);	
	
    for e in window {
        match e.event {
            Some(Event::Update(upd)) => {
                game.on_update(upd);
            }
			Some(Event::Render(ren)) => {
				game.on_draw(ren, e);
			}
			Some(Event::Input(inp)) => {
				game.on_input(inp);
			}
            _ => {

            }
        }
    }
}
