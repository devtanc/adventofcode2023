use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::usize::MAX;

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "Red" | "red" => Ok(Color::Red),
            "Green" | "green" => Ok(Color::Green),
            "Blue" | "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn gold_star_1() -> String {
    println!("Day 02 - Gold Star 1");
    // let filepath: &'static str = "./day02/test.txt";
    let filepath: &'static str = "./day02/input.txt";
    let games: Vec<_> = interpret_data(filepath);
    let mut total = 0;

    for game in games {
        let game_id = game.0.clone();
        if is_game_valid(game, 12, 13, 14) {
            // println!("Game {:?} is valid", game_id);
            total += game_id;
        } else {
            // println!("Game {:?} is not valid", game_id);
        }
    }

    return total.to_string();
}

pub fn gold_star_2() -> String {
    println!("Day 02 - Gold Star 2");
    // let filepath: &'static str = "./day02/test.txt";
    let filepath: &'static str = "./day02/input.txt";
    let games: Vec<_> = interpret_data(filepath);

    let mut total = 0;

    for game in games {
        let game_id = game.0.clone();
        let mins: (usize, usize, usize) = get_min_cubes_for_game(game);
        println!("Game {:?} mins {:?}", game_id, mins);
        total += mins.0 * mins.1 * mins.2;
    }

    return total.to_string();
}

fn get_min_cubes_for_game(game: (usize, Vec<Vec<(Color, usize)>>)) -> (usize, usize, usize) {
    let mut minimums = (0, 0, 0);
    for round in game.1 {
        for cube_set in round {
            match cube_set.0 {
                Color::Red => {
                    minimums.0 = cmp::max(minimums.0, cube_set.1);
                }
                Color::Green => {
                    minimums.1 = cmp::max(minimums.1, cube_set.1);
                }
                Color::Blue => {
                    minimums.2 = cmp::max(minimums.2, cube_set.1);
                }
            }
        }
    }

    return minimums;
}

fn is_game_valid(
    game: (usize, Vec<Vec<(Color, usize)>>),
    max_red: usize,
    max_green: usize,
    max_blue: usize,
) -> bool {
    let mut valid = true;
    let game_rounds = game.1;

    for round in game_rounds {
        for cube_set in round {
            let color = cube_set.0;
            let count = cube_set.1;
            match color {
                Color::Red => {
                    if count > max_red {
                        valid = false;
                        break;
                    }
                }
                Color::Green => {
                    if count > max_green {
                        valid = false;
                        break;
                    }
                }
                Color::Blue => {
                    if count > max_blue {
                        valid = false;
                        break;
                    }
                }
            }
        }
    }

    return valid;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(filepath: &'static str) -> Vec<(usize, Vec<Vec<(Color, usize)>>)> {
    let mut totals: Vec<_> = Vec::new();
    println!("filepath {:?}", filepath);
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(data) = line {
                let line_parts: Vec<&str> = data.split(":").collect();

                let start: Vec<&str> = line_parts[0].split_whitespace().collect();
                let id: usize = start[1].parse::<usize>().unwrap();

                let mut rounds: Vec<Vec<(Color, usize)>> = Vec::new();

                let game_rounds: Vec<&str> = line_parts[1].split(";").collect();

                for round in game_rounds {
                    let cubes: Vec<(Color, usize)> = round
                        .split(",")
                        .map(|str| {
                            let cube_set: Vec<&str> = str.trim().split_whitespace().collect();
                            (
                                cube_set[1].parse::<Color>().unwrap(),
                                cube_set[0].parse::<usize>().unwrap(),
                            )
                        })
                        .collect();
                    rounds.push(cubes);
                }

                totals.push((id, rounds));
            }
        }
    } else {
        println!("Error reading file");
    }
    return totals;
}
