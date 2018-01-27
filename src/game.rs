pub struct Game {
    pub started: bool, 
    pub sent_colonists: bool, 
    pub turn_counter: u64,

    pub earth: HomeData,
    pub colony: ColonyData
}

pub struct HomeData {
    pub funds: f64
}

pub struct ColonyData {
    pub name: String,
    pub population: u32
}

impl Game{
    pub fn new() -> Game {
        (Game {
            turn_counter: 0, 
            sent_colonists: false, 
            started: false,
            earth: HomeData {
                funds: 200.0
            }, 
            colony: ColonyData {
                name: "Mars".to_owned(), 
                population: 0
            } 
        })
    }

    pub fn is_over(&self) -> bool {
        return self.earth.funds <= 0.0 || 
            (self.sent_colonists && self.colony.population == 0);
    }

    pub fn end_turn(&mut self) {
        self.turn_counter += 1;
    }

    pub fn run_loop(&self) {

    }

    pub fn report_results(&self) {

    }
}
