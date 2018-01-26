use std::io::{self, Read};

struct RustyMarsGameData {
    has_sent_colonists: bool, 
    turn_counter: u64,
    earth: EarthData,
    colony: ColonyData
}

struct EarthData {
    funds: f64
}

struct ColonyData {
    name: String,
    population: u32
}

fn main() {
    let mut game_data = initialize_game();

    run_game_loop(&mut game_data);

    report_results(&game_data);
}

fn initialize_game() -> RustyMarsGameData {
    println!("Welcome to RustyMars");
    println!("To begin press [Enter]");

    get_input(true);

    println!("-------------------------------------------");

    (RustyMarsGameData {
        turn_counter: 0, 
        has_sent_colonists: false, 
        earth: EarthData{
            funds: 200.0
        }, 
        colony: ColonyData {
            name: "Mars".to_owned(), 
            population: 0
        } 
    })
}

fn report_results(game_data: &RustyMarsGameData) {
    println!("-------------------------------------------");
    println!("    There are not currently any results\n\n\tbut between the two of us, \n\t\tYou won!");
}

fn run_game_loop(game_data: &mut RustyMarsGameData) {
    loop {
        print_header(game_data);
        print_menu();

        let mut input = get_input(false);

        // TODO: Lower case to avoid pedantic bullshit
        match input.remove(0) {
            'q' | 'Q' => {
                break;
            }
            'l' | 'L' => {
                game_data.earth.funds -= 100.0;
                game_data.colony.population += 1;
            }
            _ => {
                println!("{}", input);
            }
        }

        if is_game_over(game_data) {
            break;
        }

        do_game_turn(game_data);
    }
}

fn get_input(allow_empty: bool) -> String {
    let mut buffer = String::new();

    loop {
        io::stdin().read_line(&mut buffer);
        buffer = buffer.trim().to_owned();

        if !buffer.is_empty() || allow_empty {
            break;
        }
    }

    (buffer)
}

fn print_header(game_data: &RustyMarsGameData) {
    println!("RustyMars >>> {}
    Population: {}\t\tFunds: ${}", game_data.colony.name, game_data.colony.population, game_data.earth.funds);
}

fn print_menu() {
    print!("Menu: 
    L: Launch 

    ------------
    S: Save Game
    Q: Quit
        >");
}

fn is_game_over(game_data: &RustyMarsGameData) -> bool {
    return game_data.earth.funds <= 0.0 || 
        (game_data.has_sent_colonists && game_data.colony.population == 0);
}

fn do_game_turn(game_data: &mut RustyMarsGameData) {
    game_data.turn_counter += 1;
}