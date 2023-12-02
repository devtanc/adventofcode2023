use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const STR_NUMS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum Numbers {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub fn gold_star_1() -> String {
    println!("Day 01 - Gold Star 1");
    // let filepath: &'static str = "./day01/test.txt";
    let filepath: &'static str = "./day01/input.txt";
    let lines: Vec<Vec<char>> = interpret_data(filepath);

    let mut total: usize = 0;

    for line in lines.iter() {
        let mut integers: Vec<usize> = Vec::new();
        for character in line.iter() {
            match character {
                '0'..='9' => integers.push(character.to_digit(10).unwrap() as usize),
                _ => (),
            }
        }

        if integers.len() == 1 {
            total += (integers[0] * 10) + integers[0];
        } else {
            total += integers[0] * 10 + integers[integers.len() - 1];
        }
    }

    return total.to_string();
}

pub fn gold_star_2() -> String {
    println!("Day 01 - Gold Star 2");
    // let filepath: &'static str = "./day01/test2.txt";
    let filepath: &'static str = "./day01/input.txt";
    let lines: Vec<Vec<(usize, usize)>> = interpret_data2(filepath);

    let mut total: usize = 0;
    lines.iter().for_each(|line| {
        println!("{:?}", line);
        total += (line.first().unwrap().1 * 10) + line.last().unwrap().1;
    });

    return total.to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(filepath: &'static str) -> Vec<Vec<char>> {
    let mut totals: Vec<Vec<char>> = Vec::new();
    println!("filepath {:?}", filepath);
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(data) = line {
                let chars: Vec<char> = data.chars().collect();
                totals.push(chars);
            }
        }
    } else {
        println!("Error reading file");
    }
    return totals;
}

fn interpret_data2(filepath: &'static str) -> Vec<Vec<(usize, usize)>> {
    let mut totals: Vec<Vec<(usize, usize)>> = Vec::new();
    println!("filepath {:?}", filepath);
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            let mut nums: Vec<(usize, usize)> = Vec::new();
            if let Ok(data) = line {
                STR_NUMS.iter().enumerate().for_each(|(i, &x)| {
                    let val = i + 1;
                    let idxs: Vec<_> = data.match_indices(x).collect();
                    idxs.iter().for_each(|(idx, _)| {
                        nums.push((*idx, val));
                    });
                });

                let line_chars: Vec<char> = data.chars().collect();

                line_chars
                    .iter()
                    .enumerate()
                    .for_each(|(i, character)| match character {
                        '0'..='9' => nums.push((i, character.to_digit(10).unwrap() as usize)),
                        _ => (),
                    });

                nums.sort_by(|a, b| a.0.cmp(&b.0));
                totals.push(nums);
            }
        }
    } else {
        println!("Error reading file");
    }
    return totals;
}
