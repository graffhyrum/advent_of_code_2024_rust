use crate::problems::Problem;
use crate::util::Point;

pub struct DayThirteen;

impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        input.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

fn parse(input: &str)  {
    // Parse the input into an Arcade
    
}

// 3 tokens to push A
// 1 token to push B


struct Arcade {
    games: Vec<Game>
}

impl Arcade {
    fn new() -> Arcade {
        Arcade {
            games: Vec::new()
        }
    }

    fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }
    
    fn from_input(input: &str) -> Arcade {
        let result = Arcade::new();
        let lines = input.lines();
        
        while let Some(button_a_line) = lines.next() {
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();
            
            let button_a = parse_button(button_a_line);
            let button_b = parse_button(button_b_line);
            let prize = parse_point(prize_line);
            
            let game = Game {
                a_button: button_a,
                b_button: button_b,
                prize: prize
            };
            
            result.add_game(game);
        }
        
        result
    }
}

struct Game{
    a_button: Button,
    b_button: Button,
    prize: Point
}

struct Button {
    position: Point,
    token_cost: u8
}