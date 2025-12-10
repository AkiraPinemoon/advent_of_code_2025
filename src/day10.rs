use std::{collections::HashMap, fs::File, io::Read};

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
                .button_masks
                .iter()
                .powerset() // for each combination of pressed buttons
                .map(|buttons| {
                    (
                        buttons.len(),
                        buttons.iter().fold(0_usize, |acc, button| acc ^ *button),
                    )
                }) // calculate the resulting state
                .filter(|(_, result)| *result == machine.indicator_goal) // compare to goal
                .map(|(count, _)| count)
                .sorted_by_cached_key(|count| *count); // sort by press count

            let best = solutions.next().unwrap();

            // println!("{best}");
            best
        })
        .sum();

    println!("{result}");
}

pub fn part2() {
    let mut file = File::open("input/10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let machines = contents.lines().map(Machine::parse).collect_vec();

    let result: usize = machines
        .into_iter()
        .take(1)
        .map(|machine| {
            let mut state = machine.button_masks.clone();
            state.fill(0);
            let mut memo = HashMap::new();
            let result =
                find_fastest_joltage_solution(&mut None, &mut memo, &machine, &state).unwrap();

            println!("solved in {result} presses");

            result
        })
        .sum();

    println!("{result}");
}

#[derive(Debug)]
struct Machine {
    indicator_goal: usize, // each bit is one indicator
    buttons: Vec<Vec<usize>>,
    button_masks: Vec<usize>,
    joltage_goal: Vec<usize>,
}

impl Machine {
    pub fn parse(input: &str) -> Self {
        let blocks = input.split(' ').collect_vec();
        let indicator_goal = blocks[0][1..blocks[0].len() - 1] // remove surrounding brackets
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
                    .collect_vec()
            })
            .collect_vec();

        let button_masks = buttons
            .iter()
            .map(|button| button.iter().map(|index| 2_usize.pow(*index as u32)).sum())
            .collect_vec();

        let joltage_goal = blocks.last().unwrap()[1..blocks.last().unwrap().len() - 1]
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok()) // parse button indices
            .collect_vec();

        Machine {
            indicator_goal,
            buttons,
            button_masks,
            joltage_goal,
        }
    }
}

fn find_fastest_joltage_solution(
    current_best: &mut Option<usize>,
    memo: &mut HashMap<Vec<usize>, Option<usize>>,
    machine: &Machine,
    state: &Vec<usize>,
) -> Option<usize> {
    if let Some(best) = current_best {
        if state.iter().sum::<usize>() >= *best {
            return None;
        }
    }

    let joltage = get_joltatge_from_state(state, machine);

    if let Some(&result) = memo.get(&joltage) {
        return result;
    }

    println!("{state:?}");

    if joltage == machine.joltage_goal {
        let _ = current_best.insert(state.iter().sum());
        return Some(0);
    }

    let possible_buttons = machine
        .buttons
        .iter()
        .enumerate()
        .filter(|(_, button)| {
            button
                .iter()
                .all(|&index| joltage[index] < machine.joltage_goal[index])
        })
        .collect_vec();

    if possible_buttons.is_empty() {
        memo.insert(joltage.clone(), None);
        return None;
    }

    let result = possible_buttons
        .into_iter()
        .filter_map(|(i, _button)| {
            let mut state_after = state.clone();
            state_after[i] += 1;
            find_fastest_joltage_solution(current_best, memo, machine, &state_after)
                .map(|count| count + 1)
        })
        .min();

    memo.insert(joltage.clone(), result);
    result
}

fn get_joltatge_from_state(state: &Vec<usize>, machine: &Machine) -> Vec<usize> {
    let mut joltage = machine.joltage_goal.clone();
    joltage.fill(0);

    for (i, button) in machine.buttons.iter().enumerate() {
        for effeced in button {
            joltage[*effeced] += state[i];
        }
    }

    joltage
}
