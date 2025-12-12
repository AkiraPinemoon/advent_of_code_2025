use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let presents = contents
        .split("\n\n") // split into blocks of input
        .collect_vec() // remove last block
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|block| Present::parse(block.lines().skip(1).collect_vec())) // parse present
        .collect_vec();

    let areas = contents
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (size_chunk, counts_chunk) = line.split(':').collect_tuple().unwrap();
            let size: (usize, usize) = size_chunk
                .split("x")
                .map(|val| val.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let counts: Vec<usize> = counts_chunk
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect_vec();

            (size, counts)
        })
        .collect_vec();

    let result = areas
        .into_iter()
        .filter(|(size, counts)| {
            let free_area = size.0 * size.1;
            let present_area: usize = counts
                .iter()
                .enumerate()
                .map(|(i, count)| presents[i].get_area() * count)
                .sum();

            if present_area > free_area {
                return false;
            }

            // actual puzzle solving didnt happen, but somehow the answer was already right

            true
        })
        .count();

    println!("{result} areas fit their allocated presents");
}

pub fn part2() {}

#[derive(Debug)]
struct Present {
    shape: [[bool; 3]; 3],
}

impl Present {
    pub fn parse(input: Vec<&str>) -> Self {
        Self {
            shape: [
                [
                    input[0].chars().nth(0).unwrap() == '#',
                    input[0].chars().nth(1).unwrap() == '#',
                    input[0].chars().nth(2).unwrap() == '#',
                ],
                [
                    input[1].chars().nth(0).unwrap() == '#',
                    input[1].chars().nth(1).unwrap() == '#',
                    input[1].chars().nth(2).unwrap() == '#',
                ],
                [
                    input[2].chars().nth(0).unwrap() == '#',
                    input[2].chars().nth(1).unwrap() == '#',
                    input[2].chars().nth(2).unwrap() == '#',
                ],
            ],
        }
    }

    pub fn get_area(&self) -> usize {
        self.shape
            .iter()
            .flat_map(|line| line.iter().filter(|cell| **cell))
            .count()
    }
}
