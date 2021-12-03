fn solve_first(input: &str) -> u32 {
    // Grabs the len of the "binary" number
    let end = input.lines().next().unwrap().len() - 1;
    let bin_numbers: Vec<u32> = input.lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();
    let (gamma, ep) = (0..=end)
        .rev()
        .map(|i| {
            let bit = 1 << i;
            let (zero, one) = bin_numbers.iter().fold((0, 0), |(zero, one), num| {
                if (num & bit) == bit {
                    (zero, one + 1)
                } else {
                    (zero + 1, one)
                }
            });
            if one > zero {
                (bit, 0)
            } else {
                (0, bit)
            }
        }).fold((0, 0), |(gamma, ep), (g, e)| (gamma | g , ep | e));
    gamma * ep
}

// The bit criteria depends on which type of rating value you want to find:

//     To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. 
//     If 0 and 1 are equally common, keep values with a 1 in the position being considered.
//     To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. 
//     If 0 and 1 are equally common, keep values with a 0 in the position being considered.


fn solve_second(input: &str) -> u32 {
    // Grabs the len of the "binary" number
    let end = input.lines().next().unwrap().len() - 1;
    let bin_numbers: Vec<u32> = input.lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();
    
    let mut oxy_rate = bin_numbers.clone();
    let mut co2_rate = bin_numbers.clone();
    for bit in (0..=end).rev().map(|i| 1 << i) {
        let count = | bin_nums: &[u32] | {
            bin_nums.iter().fold((0, 0), |(zero, one), num| {
                if (num & bit) == bit {
                    (zero, one + 1)
                } else {
                    (zero + 1, one)
                }
            })
        };

        let vec_reduction = | bin_nums: &mut Vec<u32>, wanted_num | {
            let mut i = 0;
            while i < bin_nums.len() {
                if bin_nums[i] & bit == wanted_num {
                    i += 1;
                } else {
                    bin_nums.swap_remove(i);
                }
            }
        };
        if oxy_rate.len() > 1 {
            let (zero, one) = count(&oxy_rate);
            vec_reduction(&mut oxy_rate, if one >= zero { bit } else {0});
        }
        if co2_rate.len() > 1 {
            let (zero, one) = count(&co2_rate);
            vec_reduction(&mut co2_rate, if one < zero { bit } else {0});
        }
    }
    oxy_rate[0] * co2_rate[0]
}

fn main() {

    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    let solution1 = solve_first(example);
    println!("{}", solution1);
    let solution1 = solve_first(input);
    println!("{}", solution1);
    
    let solution2 = solve_second(example);
    println!("{}", solution2);
    let solution2 = solve_second(input);
    println!("{}", solution2);
}
