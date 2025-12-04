use std::{fs::File, io::Read};

pub fn part1() {
    let mut file = File::open("input/04.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let mut accessible_count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                let mut count = 0;

                if y > 0 && x > 0 && map[y - 1][x - 1] == true {
                    count += 1;
                }

                if y > 0 && map[y - 1][x] == true {
                    count += 1;
                }

                if y > 0 && x < map[0].len() - 1 && map[y - 1][x + 1] == true {
                    count += 1;
                }

                if x > 0 && map[y][x - 1] == true {
                    count += 1;
                }

                if x < map[0].len() - 1 && map[y][x + 1] == true {
                    count += 1;
                }

                if y < map.len() - 1 && x > 0 && map[y + 1][x - 1] == true {
                    count += 1;
                }

                if y < map.len() - 1 && map[y + 1][x] == true {
                    count += 1;
                }

                if y < map.len() - 1 && x < map[0].len() - 1 && map[y + 1][x + 1] == true {
                    count += 1;
                }

                if count < 4 {
                    accessible_count += 1
                }
            }
        }
    }

    println!("there are {accessible_count} accessible rolls of paper");
}

pub fn part2() {
    let mut file = File::open("input/04.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let mut removed_count = 0;
    let mut cleared = Vec::new();
    loop {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] {
                    let mut count = 0;

                    if y > 0 && x > 0 && map[y - 1][x - 1] == true {
                        count += 1;
                    }

                    if y > 0 && map[y - 1][x] == true {
                        count += 1;
                    }

                    if y > 0 && x < map[0].len() - 1 && map[y - 1][x + 1] == true {
                        count += 1;
                    }

                    if x > 0 && map[y][x - 1] == true {
                        count += 1;
                    }

                    if x < map[0].len() - 1 && map[y][x + 1] == true {
                        count += 1;
                    }

                    if y < map.len() - 1 && x > 0 && map[y + 1][x - 1] == true {
                        count += 1;
                    }

                    if y < map.len() - 1 && map[y + 1][x] == true {
                        count += 1;
                    }

                    if y < map.len() - 1 && x < map[0].len() - 1 && map[y + 1][x + 1] == true {
                        count += 1;
                    }

                    if count < 4 {
                        cleared.push((y, x));
                        removed_count += 1
                    }
                }
            }
        }

        if cleared.is_empty() {
            break;
        }

        for (y, x) in cleared.drain(..) {
            map[y][x] = false;
        }
    }

    println!("there are {removed_count} rolls of paper that can be removed");
}
