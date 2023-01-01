use std::fs;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_str(str: &str) -> Option<Shape> {
        match str {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn determine(a: Shape, b: Shape) -> Outcome {
        if a == b {
            return Outcome::Draw;
        }

        match a {
            Shape::Rock => match b {
                Shape::Scissors => Outcome::Win,
                _ => Outcome::Loss,
            },
            Shape::Scissors => match b {
                Shape::Paper => Outcome::Win,
                _ => Outcome::Loss,
            },
            Shape::Paper => match b {
                Shape::Rock => Outcome::Win,
                _ => Outcome::Loss,
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let mut score = 0;

    for line in input.lines() {
        let (opponent_choice, my_choice) = line.split_once(" ").expect("Failed to split line");
        let opponent_choice = Shape::from_str(opponent_choice).expect("Invalid shape string");
        let my_choice = Shape::from_str(my_choice).expect("Invalid shape string");

        score += match my_choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        score += match Outcome::determine(my_choice, opponent_choice) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            _ => 0,
        };

        println!("score so far: {}", score);
    }

    println!("score: {score}");
}
