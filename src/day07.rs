use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();

    let mut beams = HashSet::new();
    beams.insert(
        lines
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap(),
    );
    let mut split_count = 0;

    for line in lines {
        let mut next_beams = HashSet::new();
        for beam in beams {
            if line.chars().nth(beam).unwrap() == '^' {
                split_count += 1;
                if beam > 0 {
                    next_beams.insert(beam - 1);
                }
                if beam + 1 < line.len() {
                    next_beams.insert(beam + 1);
                }
            } else {
                next_beams.insert(beam);
            }
        }

        beams = next_beams;
    }

    println!("the beam was split {split_count} times");
}

pub fn part2() {
    let mut file = File::open("input/07.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let maniold = contents
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut memo = HashMap::new();
    let path_count = get_beam_path_count(
        &mut memo,
        &maniold,
        0,
        maniold[0].iter().position(|&c| c == 'S').unwrap(),
    );

    println!("the beam can take {path_count} paths");
}

fn get_beam_path_count(
    memo: &mut HashMap<(usize, usize), usize>,
    manifold: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> usize {
    if let Some(&count) = memo.get(&(row, col)) {
        return count;
    }

    if row + 1 >= manifold.len() {
        return 1;
    }

    match manifold[row + 1][col] {
        '^' => {
            let left_paths = get_beam_path_count(memo, manifold, row + 1, col - 1);
            let right_paths = get_beam_path_count(memo, manifold, row + 1, col + 1);
            let res = left_paths + right_paths;
            memo.insert((row, col), res);
            res
        }
        _ => {
            let res = get_beam_path_count(memo, manifold, row + 1, col);
            memo.insert((row, col), res);
            res
        }
    }
}
