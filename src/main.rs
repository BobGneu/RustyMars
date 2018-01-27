

mod game;
mod menu;

use game::{Game};
use menu::{main_menu_prompt};

fn main() {
    let mut choice: char;
    let mut game_instance: Game;

    loop {
        choice = main_menu_prompt();

        if choice == 'q' {
            break;
        }

        game_instance = Game::new();

        game_instance.run_loop();
        game_instance.report_results();
    }
}