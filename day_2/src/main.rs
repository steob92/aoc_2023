use core::num;
// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;

#[derive(Clone)]
enum Color{
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
struct GameResult{
    n_red : i32,
    n_green : i32,
    n_blue : i32,
}

impl GameResult{
    fn check_result(self : &Self, other: &GameResult) -> bool{
        (self.n_red < other.n_red) || (self.n_green < other.n_green) || (self.n_blue < other.n_blue)  
    }

    fn get_power(self :&Self) -> i32{
        self.n_red * self.n_blue * self.n_green
    }
}

#[derive(Clone)]
struct Draw{
    colors: Vec<Color>,
    n_cubes : Vec<i32>,
}

impl Draw{
    pub fn get_total(self : &Self) -> GameResult {
        let mut total = GameResult{
            n_red : 0,
            n_green : 0,
            n_blue : 0,
        };

        for i in 0..self.colors.len(){
            match self.colors[i] {
                Color::Red => {
                    total.n_red += self.n_cubes[i];
                },
                Color::Green => {
                    total.n_green += self.n_cubes[i];
                },
                Color::Blue => {
                    total.n_blue += self.n_cubes[i];
                },
            }
        }
        total
    }


    pub fn get_minimums(self : &Self) -> GameResult {
        let mut minimum_cubes = GameResult{
            n_red : 0,
            n_green : 0,
            n_blue : 0,
        };

        for i in 0..self.colors.len(){
            match self.colors[i] {
                Color::Red => {
                    if minimum_cubes.n_red < self.n_cubes[i]{
                        minimum_cubes.n_red = self.n_cubes[i];
                    };
                },
                Color::Green => {
                    if minimum_cubes.n_green < self.n_cubes[i]{
                        minimum_cubes.n_green = self.n_cubes[i];
                    };
                },
                Color::Blue => {
                    if minimum_cubes.n_blue < self.n_cubes[i]{
                        minimum_cubes.n_blue = self.n_cubes[i];
                    };
                },
            }
        }
        minimum_cubes
    }
}

struct Game{
    game_id : i32,
    pub draws : Vec<Draw>,
}

impl Game{
    fn find_minimum(self: &Self) -> GameResult{
        let mut minimum = self.draws[0].get_minimums();
        for draw in self.draws.iter(){
            let current = draw.get_minimums();
            if minimum.n_red < current.n_red{
                minimum.n_red = current.n_red;
            }
            if minimum.n_green < current.n_green{
                minimum.n_green = current.n_green;
            }
            if minimum.n_blue < current.n_blue{
                minimum.n_blue = current.n_blue;
            }
        }
        minimum
    }
}

fn get_color( c : &str) -> Color{
    match c{
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Color not found"),
    }
}

fn parse_strigs( s: Vec<String>) -> Vec<Game>{


    let mut games: Vec<Game> = Vec::new();
    for line in s.iter(){
        println!("{}", line);

        let sub_str : Vec<_>= line.split(":").collect();

        let game_id = sub_str[0][5..].parse::<i32>().unwrap();
        let mut game_draws: Vec<Draw> = Vec::new();

        let mut current_game: Game = Game{game_id : game_id, draws : game_draws};

        let draws: Vec<_> = sub_str[1].split(";").collect();
        for draw in draws.iter() {
            let options: Vec<_> = draw.split(",").collect();
            let mut color:Vec<Color> = Vec::new();
            let mut count:Vec<i32> = Vec::new();
            
            for opt in options.iter(){
                let num_color : Vec<_> = opt.split_ascii_whitespace().collect();
                
                count.push(num_color[0].parse::<i32>().unwrap());
                color.push(get_color(num_color[1]));
            }
            let draw = Draw{colors:color, n_cubes:count};
            current_game.draws.push(draw);
        }
        games.push(current_game);
    }
    games
}

fn main() {

    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();


    let games = parse_strigs(my_vec);

    let possible_totals = GameResult{  n_red : 12, n_green : 13 ,  n_blue : 14};

    // Star 1
    let mut id_totals = 0;
    // Star 2
    let mut power: i32 = 0;
    for game in games.iter(){
        // println!("Game ID: {}", game.game_id);
        let mut game_success = true;

        let minimum_cubes = game.find_minimum();
        println!("{:?}",minimum_cubes);
        power += minimum_cubes.get_power();

        for draw in game.draws.iter(){

            // println!("{:?}", &draw.get_total());
            // println!("{:?}", &possible_totals);
            // println!("{:?}", possible_totals.check_result(&draw.get_total()));
            let game_total = draw.get_total();
            if possible_totals.check_result(&game_total){
                game_success = false;
                break;
            }
        }
        if game_success{
            // println!("Passing Game ID: {}", game.game_id);

            id_totals += game.game_id;
            
        }
    }

    println!("Star 1 Game ID Totals: {}", id_totals);
    println!("Star 2 Power Totals: {}", power);

}
