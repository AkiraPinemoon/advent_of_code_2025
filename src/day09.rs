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
    let mut file = File::open("input/09.txt").unwrap();
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
            let rect_min = (corner_a.0.min(corner_b.0), corner_a.1.min(corner_b.1));
            let rect_max = (corner_a.0.max(corner_b.0), corner_a.1.max(corner_b.1));
            !border_lines
                .iter()
                .any(|&line| does_line_contain_points_in_rect(rect_min, rect_max, line))
        })
        .sorted_by_cached_key(|((ax, ay), (bx, by))| ((ax - bx).abs() + 1) * ((ay - by).abs() + 1))
        .rev();

    let max_area = areas.next().unwrap();

    let area_size =
        ((max_area.0.0 - max_area.1.0).abs() + 1) * ((max_area.0.1 - max_area.1.1).abs() + 1);

    println!("the biggest possible area is {area_size} tiles with the points {max_area:?}");
}

pub fn does_line_contain_points_in_rect(
    rect_min: (i64, i64),
    rect_max: (i64, i64),
    line: ((i64, i64), (i64, i64)),
) -> bool {
    let line_vertical = line.0.0 == line.1.0;
    let (line_min, line_max) = match line_vertical {
        true => (line.0.1.min(line.1.1), line.0.1.max(line.1.1)),
        false => (line.0.0.min(line.1.0), line.0.0.max(line.1.0)),
    };

    for dir in line_min..=line_max {
        let point = if line_vertical {
            (line.0.0, dir)
        } else {
            (dir, line.0.1)
        };

        if point.0 > rect_min.0
            && point.0 < rect_max.0
            && point.1 > rect_min.1
            && point.1 < rect_max.1
        {
            return true;
        }
    }

    return false;
}
