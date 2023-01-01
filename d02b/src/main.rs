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
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None
        }
    }

    pub fn from_shape_outcome(shape: Shape, desired_outcome: Outcome) -> Shape {
        match desired_outcome {
            Outcome::Draw => shape,
            Outcome::Win => match shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Loss => match shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            }
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn from_str(str: &str) -> Option<Outcome> {
        match str {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let mut score = 0;

    for line in input.lines() {
        let (opponent_choice, desired_outcome) = line.split_once(" ").expect("Failed to split line");
        let opponent_choice = Shape::from_str(opponent_choice).expect("Invalid shape string");
        let desired_outcome = Outcome::from_str(desired_outcome).expect("Invalid outcome string");

        score += match desired_outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };

        let my_choice = Shape::from_shape_outcome(opponent_choice, desired_outcome);

        score += match my_choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        println!("score so far: {}", score);
    }

    println!("score: {score}");
}
