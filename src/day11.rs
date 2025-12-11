use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let devices = contents.lines().map(Device::parse).collect_vec();

    // the directed graph must be acyclic, otherwise the problem would be stated bad.

    let mut cyclic_buffer = devices.clone();

    while !cyclic_buffer.is_empty() {
        let next_iter = cyclic_buffer
            .iter()
            .filter(|device| {
                device
                    .outputs
                    .iter()
                    .any(|name| cyclic_buffer.iter().find(|d| d.name == *name).is_some())
            })
            .cloned()
            .collect_vec();

        if next_iter.len() == cyclic_buffer.len() {
            println!("cyclic dependencies found");
            return;
        }

        cyclic_buffer = next_iter;
    }

    let mut memo = HashMap::new();
    let paths = count_paths(&mut memo, "you", "out", &devices);

    println!("found {paths} possible paths");
}

pub fn part2() {
    let mut file = File::open("input/11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let devices = contents.lines().map(Device::parse).collect_vec();

    // the directed graph must be acyclic, otherwise the problem would be stated bad.

    let mut cyclic_buffer = devices.clone();

    while !cyclic_buffer.is_empty() {
        let next_iter = cyclic_buffer
            .iter()
            .filter(|device| {
                device
                    .outputs
                    .iter()
                    .any(|name| cyclic_buffer.iter().find(|d| d.name == *name).is_some())
            })
            .cloned()
            .collect_vec();

        if next_iter.len() == cyclic_buffer.len() {
            println!("cyclic dependencies found");
            return;
        }

        cyclic_buffer = next_iter;
    }

    let mut memo = HashMap::new();
    let svr_dac = count_paths(&mut memo, "svr", "dac", &devices);
    println!("svr => dac : {svr_dac}");

    memo.clear();
    let dac_fft = count_paths(&mut memo, "dac", "fft", &devices);
    println!("dac => fft : {dac_fft}");

    memo.clear();
    let fft_out = count_paths(&mut memo, "fft", "out", &devices);
    println!("fft => out : {fft_out}");

    memo.clear();
    let svr_fft = count_paths(&mut memo, "svr", "fft", &devices);
    println!("svr => fft : {svr_fft}");

    memo.clear();
    let fft_dac = count_paths(&mut memo, "fft", "dac", &devices);
    println!("fft => dac : {dac_fft}");

    memo.clear();
    let dac_out = count_paths(&mut memo, "dac", "out", &devices);
    println!("dac => out : {fft_out}");

    // since there are no cycles in the graph, each node can only be hit once.
    // if a path were in both (svr -> dac -> fft -> out) and (svr -> fft -> dac -> out) it would need to contain either
    // fft -> dac -> fft or dac -> fft -> dac, but because nodes cant be hit twice, this is impossible,
    // so (svr -> dac -> fft -> out) + (svr -> fft -> dac -> out) can't contain doubles

    let paths = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;

    println!("found {paths} possible paths");
}

#[derive(Clone)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn parse(input: &str) -> Self {
        let name = input[0..3].to_string();
        let outputs = input[5..]
            .split(' ')
            .map(|chunk| chunk.to_string())
            .collect_vec();

        Self { name, outputs }
    }
}

fn count_paths(
    memo: &mut HashMap<String, usize>,
    from: &str,
    to: &str,
    devices: &Vec<Device>,
) -> usize {
    if let Some(result) = memo.get(from) {
        println!("memo hit => {result}");
        return *result;
    }

    if from == to {
        return 1;
    }

    let Some(current) = devices.iter().find(|d| d.name == from) else {
        return 0;
    };

    let result = current
        .outputs
        .iter()
        .map(|output| count_paths(memo, &output, to, devices))
        .sum();

    memo.insert(from.to_string(), result);

    result
}
