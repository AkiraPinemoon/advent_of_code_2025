use std::{fs::File, io::Read};

pub fn part1() {
    let mut file = File::open("input/01.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rotations = contents.lines().map(|l| {
        let (dir, distance) = l.split_at(1);
        match dir {
            "R" => distance.parse().unwrap(),
            "L" => -(distance.parse::<i32>().unwrap()),
            _ => panic!("unexpected input"),
        }
    });

    let mut position = 50;
    let mut count = 0;

    for rotation in rotations {
        position += rotation;
        position %= 100;
        if position == 0 {
            count += 1
        }
    }

    println!("0 was reached {count} times")
}

pub fn part2() {
    let mut file = File::open("input/01.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rotations = contents.lines().map(|l| {
        let (dir, distance) = l.split_at(1);
        match dir {
            "R" => distance.parse().unwrap(),
            "L" => -(distance.parse::<i32>().unwrap()),
            _ => panic!("unexpected input"),
        }
    });

    let mut position = 50;
    let mut count = 0;

    for rotation in rotations {
        for _ in 0..rotation.abs() {
            if rotation > 0 {
                position += 1
            } else {
                position -= 1
            }

            position %= 100;

            if position == 0 {
                count += 1
            }
        }

        // position += rotation;

        // while position >= 100 {
        //     position -= 100;
        //     count += 1;
        // }

        // while position < 0 {
        //     position += 100;
        //     count += 1;
        // }

        // if position == 0 {
        //     count += 1
        // }
    }

    println!("0 was reached {count} times")
}
