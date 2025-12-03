use std::{fs::File, io::Read};

pub fn part1() {
    let mut file = File::open("input/03.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: u32 = contents
        .lines()
        .map(|line| {
            let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let first = bank[0..(bank.len() - 1)].iter().max().unwrap();
            let first_index = bank.iter().position(|&x| x == *first).unwrap();
            let last = bank[first_index + 1..].iter().max().unwrap();

            println!("first: {first}, last: {last}");

            let joltage = 10 * first + last;

            joltage
        })
        .sum();

    println!("the cummulative joltage is is {sum}")
}

pub fn part2() {
    let mut file = File::open("input/03.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: u64 = contents
        .lines()
        .map(|line| {
            let bank: Vec<u64> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();

            let mut selected = Vec::new();
            let mut beginning = 0;

            println!("Processing new line");

            for remaining_selections in (0..12).rev() {
                println!(
                    "{:#?}",
                    bank[beginning..(bank.len() - remaining_selections)]
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
                let next = bank[beginning..(bank.len() - remaining_selections)]
                    .iter()
                    .max()
                    .unwrap();
                beginning = beginning
                    + bank[beginning..(bank.len() - remaining_selections)]
                        .iter()
                        .position(|&x| x == *next)
                        .unwrap()
                    + 1;
                selected.push(next);

                println!("selected so far: {:#?}", selected);
            }

            let joltage = selected
                .into_iter()
                .fold(0, |acc, battery| acc * 10 + battery);

            joltage
        })
        .sum();

    println!("the cummulative joltage is is {sum}")
}
