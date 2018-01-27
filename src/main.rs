mod game;
mod menu;

use game::{Game};
use menu::{main_menu_prompt};

fn main() {
    let mut choice: char;
    let mut game_instance: Game;

    game_instance = Game::new();

    loop {
        choice = main_menu_prompt();

        if choice == 'q' {
            break;
        } else if choice == 'n' {
            game_instance = Game::new();
        } else if choice == 'l' {
            // list options
            // game_instance = Game::load(pathToSave)
        } else {
            println!("
## Invalid choice. Please try again.
");
        }

        game_instance.run_loop();
        game_instance.report_results();
    }
}
