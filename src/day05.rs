use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/05.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let fresh_ranges = contents
        .split("\n\n")
        .into_iter()
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let items = contents
        .split("\n\n")
        .into_iter()
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let count = items
        .iter()
        .filter(|id| {
            fresh_ranges
                .iter()
                .any(|&(start, end)| *id >= &start && *id <= &end)
        })
        .count();

    println!("there are {count} fresh ingredients");
}

pub fn part2() {
    let mut file = File::open("input/05.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut fresh_ranges = contents
        .split("\n\n")
        .into_iter()
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    // merge ranges
    loop {
        let mut merged_ranges = Vec::new();

        // in a loop
        'canditates: for (i, &(start, end)) in fresh_ranges.iter().enumerate() {
            // pick the next range from fresh_ranges
            for (j, &(other_start, other_end)) in fresh_ranges.iter().enumerate() {
                // see if it overlaps with any other range
                if i != j && start <= other_end && other_start <= end {
                    // if it does, merge them and add the merge result and all unmerged ranges to merged_ranges, then break
                    let new_start = start.min(other_start);
                    let new_end = end.max(other_end);
                    merged_ranges.push((new_start, new_end));
                    for (k, range) in fresh_ranges.iter().enumerate() {
                        if k != i && k != j {
                            merged_ranges.push(*range);
                        }
                    }
                    break 'canditates;
                }
            }
        }

        if merged_ranges.is_empty() {
            // if none overlap, we're done
            break;
        }
        fresh_ranges = merged_ranges;
    }

    let count = fresh_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum::<u64>();

    println!("there are {count} fresh ingredients");
}
