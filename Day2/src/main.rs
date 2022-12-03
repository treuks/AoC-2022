use std::fs::File;
use std::io::{BufReader, BufRead};
pub enum Tools {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
struct RPS {
    score: u64,
}
/*
    1 Point  - Rock
    2 Points - Paper
    3 Points - Scissors
*/
impl RPS {

    fn play_turn(&mut self, first_tool: Tools, second_tool: Tools) {
        match (second_tool, first_tool) {
            (self::Tools::Rock, self::Tools::Rock) => {
                self.score += 1;
                self.score += 3; // Draw grants 3 points
            },
            (self::Tools::Rock, self::Tools::Paper) => {
                self.score += 1;
            },
            (self::Tools::Rock, self::Tools::Scissors) => {
                self.score += 1;
                self.score += 6; // Win grants 6 points
            },
            (self::Tools::Paper, self::Tools::Rock) => {
                self.score += 2;
                self.score += 6; // Win grants 6 points
            }
            (self::Tools::Paper, self::Tools::Paper) => {
                self.score += 2;
                self.score += 3; // Draw grants 3 points
            }
            (self::Tools::Paper, self::Tools::Scissors) => {
                self.score += 2;
            }
            (self::Tools::Scissors, self::Tools::Rock) => {
                self.score += 3;
            }
            (self::Tools::Scissors, self::Tools::Paper) => {
                self.score += 3;
                self.score += 6;
            }
            (self::Tools::Scissors, self::Tools::Scissors) => {
                self.score += 3;
                self.score += 3; // Draw grants 3 points
            }
        }
        println!("Score: {}", self.score);

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
                Tools::Rock
            }
            "Y" => {
                Tools::Paper
            }
            "Z" => {
                Tools::Scissors
            }
            _ => {
                panic!("Right hand's fucked")
            }
        };
        game.play_turn(left_hand, right_hand);
    }
    println!("Final score: {}", game.get_score());
}

