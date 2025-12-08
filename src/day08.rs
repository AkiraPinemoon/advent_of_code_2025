use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/08.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let junctions = contents
        .lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .take(3)
                .map(|v| v.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            Junction { x, y, z }
        })
        .collect_vec();

    let short_connections = junctions
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a != b)
        .sorted_by_cached_key(|(a, b)| a.distance(b))
        .collect_vec();

    let mut remaining_connections = 1000;
    let mut next_circuit_id = 0;

    let mut circuits: HashMap<Junction, u32> = HashMap::new();

    for (a, b) in short_connections.iter() {
        match (circuits.get(&a).cloned(), circuits.get(&b).cloned()) {
            (Some(ac), Some(bc)) => {
                if ac != bc {
                    for (_k, v) in circuits.iter_mut() {
                        if *v == bc {
                            *v = ac;
                        }
                    }
                }
            }

            (Some(ac), None) => {
                circuits.insert(**b, ac);
            }

            (None, Some(bc)) => {
                circuits.insert(**a, bc);
            }

            (None, None) => {
                circuits.insert(**a, next_circuit_id);
                circuits.insert(**b, next_circuit_id);
                next_circuit_id += 1;
            }
        }

        remaining_connections -= 1;

        if remaining_connections == 0 {
            break;
        }
    }

    let circuit_sizes = circuits
        .values()
        .fold(HashMap::new(), |mut acc, cid| {
            *acc.entry(*cid).or_insert(0) += 1;
            acc
        })
        .into_values()
        .sorted()
        .rev()
        .collect_vec();

    println!(
        "there resulting value is {}",
        circuit_sizes.into_iter().take(3).product::<i32>()
    );
}

pub fn part2() {
    let mut file = File::open("input/08.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let junctions = contents
        .lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .take(3)
                .map(|v| v.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            Junction { x, y, z }
        })
        .collect_vec();

    let short_connections = junctions
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a != b)
        .sorted_by_cached_key(|(a, b)| a.distance(b))
        .collect_vec();

    let mut next_circuit_id = 0;
    let mut circuits: HashMap<Junction, u32> = HashMap::new();

    for (a, b) in short_connections.iter() {
        match (circuits.get(&a).cloned(), circuits.get(&b).cloned()) {
            (Some(ac), Some(bc)) => {
                if ac != bc {
                    for (_k, v) in circuits.iter_mut() {
                        if *v == bc {
                            *v = ac;
                        }
                    }
                }
            }

            (Some(ac), None) => {
                circuits.insert(**b, ac);
            }

            (None, Some(bc)) => {
                circuits.insert(**a, bc);
            }

            (None, None) => {
                circuits.insert(**a, next_circuit_id);
                circuits.insert(**b, next_circuit_id);
                next_circuit_id += 1;
            }
        }

        let cirquit_count = circuits.values().dedup().count();

        if circuits.len() == junctions.len() && cirquit_count == 1 {
            println!(
                "the last two junctions to connect are {:?} and {:?} with wall distance {}",
                a,
                b,
                a.x * b.x
            );

            break;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Junction {
    x: u32,
    y: u32,
    z: u32,
}

impl Junction {
    fn distance(&self, other: &Junction) -> u64 {
        (((self.x as i64 - other.x as i64).abs().pow(2)
            + (self.y as i64 - other.y as i64).abs().pow(2)
            + (self.z as i64 - other.z as i64).abs().pow(2)) as f64)
            .sqrt() as u64
    }
}
