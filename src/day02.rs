use std::{fs::File, io::Read};

pub fn part1() {
    let mut file = File::open("input/02.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: i64 = contents
        .strip_suffix("\n")
        .unwrap()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start: i64 = parts.next().unwrap().parse().unwrap();
            let end: i64 = parts.next().unwrap().parse().unwrap();

            let mut sum = 0;

            for id in start..=end {
                let id_str = id.to_string();
                if id_str.len() % 2 != 0 {
                    continue;
                } // Skip odd-length IDs
                let first_half = &id_str[..id_str.len() / 2];
                let second_half = &id_str[id_str.len() / 2..];
                if first_half == second_half {
                    println!("found invalid id {id}");
                    sum += id;
                }
            }

            sum
        })
        .sum();

    println!("the sum of invalid ids is {sum}")
}

pub fn part2() {
    let mut file = File::open("input/02.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: i64 = contents
        .strip_suffix("\n")
        .unwrap()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start: i64 = parts.next().unwrap().parse().unwrap();
            let end: i64 = parts.next().unwrap().parse().unwrap();

            let mut sum = 0;

            for id in start..=end {
                let id_str = id.to_string();

                'lengths: for len in 1..=id_str.len() / 2 {
                    if (id_str.len() % len) != 0 {
                        continue 'lengths;
                    } // Skip if length is not a multiple of len

                    for i in 0..id_str.len() {
                        if id_str.chars().nth(i).unwrap() != id_str.chars().nth(i % len).unwrap() {
                            continue 'lengths;
                        } // Skip if characters do not match
                    }

                    println!("found invalid id {id}");
                    sum += id;
                    break 'lengths; // break the loop, so 12121212 will only be counted once (12 * 4) and not for (1212 * 2)
                }
            }

            sum
        })
        .sum();

    println!("the sum of invalid ids is {sum}")
}
