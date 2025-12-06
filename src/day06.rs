use std::{fs::File, io::Read};

use itertools::{Itertools, izip};

pub fn part1() {
    let mut file = File::open("input/06.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines().collect_vec();

    let cols = izip!(
        lines[0].chars(),
        lines[1].chars(),
        lines[2].chars(),
        lines[3].chars(),
        lines[4].chars()
    )
    .collect_vec();

    let mut problems = Vec::new();
    let mut acc = (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );
    for (i, col) in cols.iter().enumerate() {
        acc.0.push(col.0);
        acc.1.push(col.1);
        acc.2.push(col.2);
        acc.3.push(col.3);
        acc.4.push(col.4);

        if i == cols.len() - 1
            || col.0 == ' ' && col.1 == ' ' && col.2 == ' ' && col.3 == ' ' && col.4 == ' '
        {
            problems.push((
                acc.0.trim().parse::<u64>().unwrap(),
                acc.1.trim().parse::<u64>().unwrap(),
                acc.2.trim().parse::<u64>().unwrap(),
                acc.3.trim().parse::<u64>().unwrap(),
                acc.4.chars().next().unwrap(),
            ));
            acc = (
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            );
        }
    }

    let solutions = problems
        .into_iter()
        .map(|p| match p.4 {
            '+' => p.0 + p.1 + p.2 + p.3,
            '*' => p.0 * p.1 * p.2 * p.3,
            _ => panic!("unknown operator"),
        })
        .collect_vec();

    let sum = solutions.into_iter().sum::<u64>();

    println!("the sum of all solutions is {sum}");
}

pub fn part2() {
    let mut file = File::open("input/06.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines().collect_vec();

    let cols = izip!(
        lines[0].chars(),
        lines[1].chars(),
        lines[2].chars(),
        lines[3].chars(),
        lines[4].chars()
    )
    .collect_vec();

    let mut problems = Vec::new();
    let mut acc = Vec::new();

    for col in cols.iter().rev() {
        if col.0 == ' ' && col.1 == ' ' && col.2 == ' ' && col.3 == ' ' && col.4 == ' ' {
            continue;
        }

        let num: u64 = format!("{}{}{}{}", col.0, col.1, col.2, col.3)
            .trim()
            .parse()
            .unwrap();

        acc.push(num);

        if col.4 != ' ' {
            problems.push((acc.clone(), col.4));
            acc.clear();
        }
    }

    let solutions: Vec<u64> = problems
        .into_iter()
        .map(|p| match p.1 {
            '+' => p.0.into_iter().sum(),
            '*' => p.0.into_iter().product(),
            _ => panic!("unknown operator"),
        })
        .collect_vec();

    let sum = solutions.into_iter().sum::<u64>();

    println!("the sum of all solutions is {sum}");
}
