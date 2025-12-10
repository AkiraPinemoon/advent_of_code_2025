use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let machines = contents.lines().map(Machine::parse).collect_vec();

    let result: usize = machines
        .into_iter()
        .map(|machine| {
            // since a || a = 0 each button can only be pressed 0 or 1 times
            let mut solutions = machine
                .buttons
                .iter()
                .powerset() // for each combination of pressed buttons
                .map(|buttons| {
                    (
                        buttons.len(),
                        buttons.iter().fold(0_usize, |acc, button| acc ^ *button),
                    )
                }) // calculate the resulting state
                .filter(|(_, result)| *result == machine.goal) // compare to goal
                .map(|(count, _)| count)
                .sorted_by_cached_key(|count| *count); // sort by press count

            let best = solutions.next().unwrap();

            // println!("{best}");
            best
        })
        .sum();

    println!("{result}");
}

pub fn part2() {}

#[derive(Debug)]
struct Machine {
    goal: usize, // each bit is one indicator
    buttons: Vec<usize>,
}

impl Machine {
    pub fn parse(input: &str) -> Self {
        let blocks = input.split(' ').collect_vec();
        let goal = blocks[0][1..blocks[0].len() - 1] // remove surrounding brackets
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '#' => 2_usize.pow(i as u32),
                '.' => 0,
                _ => panic!(),
            }) // '#' means on, '.' means off
            .sum();

        let buttons = blocks[1..blocks.len() - 1] // skip first and last block
            .iter()
            .map(|block| {
                block[1..block.len() - 1] // remove surrounding brackets
                    .split(',')
                    .filter_map(|s| s.parse::<usize>().ok()) // parse button indices
                    .map(|index| 2_usize.pow(index as u32))
                    .sum()
            })
            .collect_vec();

        Machine { goal, buttons }
    }
}
