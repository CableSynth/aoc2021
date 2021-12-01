use std::fs;

fn solve_first(input: &str) -> u64 {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .fold((0, i32::MAX), |(sum, previous), number| {
            (if number > previous { sum + 1 } else { sum }, number)
        })
        .0
}

fn main() {
    let filename = "src/input.txt";
    let data = fs::read_to_string(filename).expect("error");
    let data_split_on_newline: Vec<&str> = data.split("\n").collect();
    let mut total = 0;
    let mut previous = data_split_on_newline[0].parse::<i64>().expect("Bad Value");
    for line in data_split_on_newline {
        let compline = line.parse::<i64>().expect("Bad Value");
        if compline > previous {
            total += 1;
        }
        previous = compline;
    }
    println!("{}", total);

    let data_split_on_newline: Vec<&str> = data.split("\n").collect();
    total = 0;
    previous = 0;
    for i in 0..(data_split_on_newline.len()) {
        let compline = data_split_on_newline[i].parse::<i64>().expect("Bad Value");
        let compline1 = data_split_on_newline[i + 1]
            .parse::<i64>()
            .expect("Bad Value");
        let compline2 = data_split_on_newline[i + 2]
            .parse::<i64>()
            .expect("Bad Value");
        let sum = compline + compline1 + compline2;
        // println!("{}", sum);
        if previous == 0 {
            previous = sum;
            continue;
        }
        if sum > previous {
            total += 1;
        }
        previous = sum;
        if i + 3 == data_split_on_newline.len() {
            break;
        }
    }
    println!("{}", total);

    let input = include_str!("input.txt");
    let solution1 = solve_first(input);
    println!("{}", solution1);
}
