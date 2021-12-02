use std::fs;

fn solve_first(input: &str) -> i64 {
    let (depth, width) = input
        .lines()
        .filter_map(|direction_map| {
            direction_map.split_once(' ')
            .map(|(direction, distance)| (direction, distance.parse::<i64>().unwrap()))
        })
        .fold((0, 0), |(depth, horizontal), (direction, distance)| match direction {
            "forward" => (depth, horizontal + distance),
            "down" => (depth + distance, horizontal),
            "up" => (depth - distance, horizontal),
            &_ => (depth, horizontal)
        });
    depth * width
}

fn solve_second(input: &str) -> i64 {
    let (depth, width, _) = input
        .lines()
        .filter_map(|direction_map| {
            direction_map.split_once(' ')
            .map(|(direction, distance)| (direction, distance.parse::<i64>().unwrap()))
        })
        .fold((0, 0, 0), |(depth, horizontal, aim), (direction, distance)| match direction {
            "forward" => (depth + aim * distance, horizontal + distance, aim),
            "down" => (depth, horizontal, aim + distance),
            "up" => (depth, horizontal, aim - distance),
            &_ => (depth, horizontal, aim)
        });
    depth * width
}

fn main() {

    let input = include_str!("input.txt");
    let solution1 = solve_first(input);
    let solution2 = solve_second(input);
    println!("{}", solution1);
    println!("{}", solution2);
}
