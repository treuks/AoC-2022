use std::fs::File;
use std::io::{BufReader, BufRead};
pub enum Tools {
    Rock,
    Paper,
    Scissors,
}
pub enum GameEnum {
    NeedsWin,
    NeedsDraw,
    NeedsLose,        
}
#[derive(Debug)]
struct RPS {
    score: u64,
}

impl RPS {

    fn play_turn(&mut self, tool: Tools, endgoal: GameEnum) {
        match endgoal {

            self::GameEnum::NeedsWin => {
                self.score += 6;
                match tool {
                    self::Tools::Paper => {
                        self.score += 3; // Scissors beat the paper, therefore you get 3 points
                    }
                    self::Tools::Rock => {
                        self.score += 2; // Paper beats rock, therefore you get 2 points
                    }
                    self::Tools::Scissors => {
                        self.score += 1; // Rock beats scissors, therefore you get 1 point
                    }
                }
            }
            self::GameEnum::NeedsDraw => {
                self.score += 3;
                match tool {
                    self::Tools::Scissors => {
                        self.score += 3; 
                    }
                    self::Tools::Paper => {
                        self.score += 2;
                    }
                    self::Tools::Rock => {
                        self.score += 1; 
                    }
                }
            }
            self::GameEnum::NeedsLose => {
                match tool {
                    self::Tools::Rock => {
                        self.score += 3; 
                    }
                    self::Tools::Scissors => {
                        self.score += 2; 
                    }
                    self::Tools::Paper => {
                        self.score += 1;
                    }
                }
            }
        }

    }
    fn get_score(&self) -> &u64 {
        return &self.score;
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut game = RPS { score: 0 };
    for line in reader.lines() {
        let linesxdxd = line.unwrap();
        let linexd = linesxdxd.split_once(" ").unwrap();
        let left_hand = match linexd.0 {
            "A" => {
                Tools::Rock
            }
            "B" => {
                Tools::Paper
            }
            "C" => {
                Tools::Scissors
            }
            _ => {
                panic!("Left hand's fucked")
            }
        };
        let right_hand = match linexd.1 {
            "X" => {
                GameEnum::NeedsLose
            }
            "Y" => {
                GameEnum::NeedsDraw
            }
            "Z" => {
                GameEnum::NeedsWin
            }
            _ => {
                panic!("Right hand's fucked")
            }
        };
        game.play_turn(left_hand, right_hand);
    }
    println!("Final score: {}", game.get_score());
}

