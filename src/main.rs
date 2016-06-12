extern crate multy_task_lib;
extern crate piston_window;

use std::rc::Rc;

use piston_window::*;
use multy_task_lib::Game;
use multy_task_lib::definitions::Settings;

fn main() {
	let mut temp_conf = Settings::new();
	temp_conf.set_general_scaling_factor(3.0);
	
	let conf : Rc<Settings> = Rc::new(Settings::from_file("config.txt"));
	
	let (screen_width, screen_height) = conf.get_screen_dimensions();
	
	let window: PistonWindow = WindowSettings::new(
        "Multy Task",
        [screen_width, screen_height]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
	
	let mut game = Game::new(&window, &conf);	
	
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
