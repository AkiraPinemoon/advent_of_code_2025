use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part1() {
    let mut file = File::open("input/09.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let positions = contents.lines().map(|l| {
        let (x, y) = l
            .split(',')
            .take(2)
            .map(|v| v.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (x, y)
    });

    let mut areas = positions
        .tuple_combinations()
        .map(|((ax, ay), (bx, by))| ((ax - bx).abs() + 1) * ((ay - by).abs() + 1))
        .sorted()
        .rev();

    let max_area = areas.next().unwrap();

    println!("the biggest possible area is {max_area} tiles");
}

pub fn part2() {
    let mut file = File::open("input/09_test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let red_positions = contents.lines().map(|l| {
        let (x, y) = l
            .split(',')
            .take(2)
            .map(|v| v.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (x, y)
    });

    let border_lines = red_positions
        .clone()
        .collect_vec()
        .into_iter()
        .circular_tuple_windows()
        .map(|((ax, ay), (bx, by))| ((ax, ay), (bx, by)))
        .collect_vec();

    let mut areas = red_positions
        .tuple_combinations()
        .filter(|(corner_a, corner_b)| {
            let rect_lines = vec![
                ((corner_a.0, corner_a.1), (corner_b.0, corner_a.1)),
                ((corner_b.0, corner_a.1), (corner_b.0, corner_b.1)),
                ((corner_b.0, corner_b.1), (corner_a.0, corner_b.1)),
                ((corner_a.0, corner_b.1), (corner_a.0, corner_a.1)),
            ];
            !rect_lines
                .into_iter()
                .cartesian_product(border_lines.iter())
                .any(|(line_a, &line_b)| do_lines_cross(line_a, line_b))
        })
        .sorted_by_cached_key(|((ax, ay), (bx, by))| ((ax - bx).abs() + 1) * ((ay - by).abs() + 1))
        .rev();

    let max_area = areas.next().unwrap();

    let area_size =
        ((max_area.0.0 - max_area.1.0).abs() + 1) * ((max_area.0.1 - max_area.1.1).abs() + 1);

    println!("the biggest possible area is {area_size} tiles with the points {max_area:?}");
}

pub fn do_lines_cross(line1: ((i64, i64), (i64, i64)), line2: ((i64, i64), (i64, i64))) -> bool {
    let line1_vertical = line1.0.0 == line1.1.0;
    let line2_vertical = line2.0.0 == line2.1.0;

    match (line1_vertical, line2_vertical) {
        (true, true) => return false,
        (false, false) => return false,
        _ => {
            let (vert_line, horiz_line) = if line1_vertical {
                (line1, line2)
            } else {
                (line2, line1)
            };

            let treshhold_x = vert_line.0.0;
            let threshhold_y = horiz_line.0.1;

            let x_min = horiz_line.0.0.min(horiz_line.1.0);
            let x_max = horiz_line.0.0.max(horiz_line.1.0);

            let y_min = vert_line.0.1.min(vert_line.1.1);
            let y_max = vert_line.0.1.max(vert_line.1.1);

            return (x_min < treshhold_x && treshhold_x < x_max)
                && (y_min < threshhold_y && threshhold_y < y_max);
        }
    }
}
