use std::{fs::File, io::Read};

use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};
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
        .map(|machine| {
            let mut problem = ProblemVariables::new();

            let vars = machine
                .buttons
                .iter()
                .map(|_button| problem.add(variable().integer().min(0)))
                .collect_vec();

            let mut min_expresssion = Expression::from(0);
            for var in vars.iter() {
                min_expresssion += var
            }

            let mut model = problem.minimise(min_expresssion).using(default_solver);

            for counter in 0..machine.joltage_goal.len() {
                let mut expression = Expression::from(0);
                for (index, button) in machine.buttons.iter().enumerate() {
                    if button.contains(&counter) {
                        expression += vars[index]
                    }
                }
                model = model.with(expression.eq(machine.joltage_goal[counter] as u32))
            }

            let solution = model.solve().unwrap();

            let results = vars
                .iter()
                .map(|var| solution.value(*var).round() as usize)
                .collect_vec();

            let sum = results.iter().sum::<usize>();

            println!("solved in {:?} = {} presses", results, sum);

            sum
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
