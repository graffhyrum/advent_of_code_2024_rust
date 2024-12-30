use crate::problems::Problem;
use std::collections::HashMap;
use std::time::Instant;

pub struct DayEleven;

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        self.solve(input, 25)
    }

    fn part_two(&self, input: &str) -> String {
        self.solve(input, 75)
    }
}

impl DayEleven {
    fn solve(&self, input: &str, blinks: usize) -> String {
        let start = Instant::now();
        let stones = parse_input(input);
        let result = blink_n_times(stones, blinks);
        let duration = start.elapsed();
        println!("Total time: {:?}", duration);
        result.to_string()
    }
}

fn blink_n_times(stones: Stones, n: usize) -> usize {
    let mut result = 0;
    let mut cache: Cache = HashMap::new();
    for stone in stones {
        result += count_blinks(stone, n, &mut cache);
    }
    result
}

fn count_blinks(stone: Stone, depth: usize, cache: &mut Cache) -> usize {
    if let Some(&value) = cache.get(&(stone, depth)) {
        return value;
    }
    let (left, right) = process_stone(stone);
    if depth == 1 {
        if right.is_none() {
            cache.insert((stone, depth), 1);
            1
        } else {
            cache.insert((stone, depth), 2);
            2
        }
    } else {
        let mut output = count_blinks(left.unwrap(), depth - 1, cache);
        if right.is_some() {
            output += count_blinks(right.unwrap(), depth - 1, cache);
        }
        cache.insert((stone, depth), output);
        output
    }
}

fn process_stone(stone: Stone) -> (Option<Stone>, Option<Stone>) {
    let result = if stone == 0 {
        (Some(1), None)
    } else if count_digits(stone) % 2 == 0 {
        let (left, right) = split_integer_into_halves(stone);
        (Some(left), Some(right))
    } else {
        let new_stone = stone * 2024;
        (Some(new_stone), None)
    };
    result
}

fn split_integer_into_halves(num: u64) -> (u64, u64) {
    if num == 0 {
        return (0, 0); // Special case for 0
    }

    let total_digits = count_digits(num);
    let half_digits = total_digits / 2;

    // Calculate the divisor for splitting the number
    let divisor = 10u64.pow(half_digits);

    let first_half = num / divisor;
    let second_half = num % divisor;

    (first_half, second_half)
}

fn count_digits(num: u64) -> u32 {
    if num == 0 {
        1 // Special case for 0, which has 1 digit
    } else {
        (num as f64).log10().floor() as u32 + 1
    }
}

fn parse_input(input: &str) -> Stones {
    input
        .split_whitespace()
        .map(|s| s.parse::<Stone>().expect("Failed to parse input"))
        .collect()
}

type Stones = Vec<Stone>;
type Stone = u64;
type Cache = HashMap<(Stone, usize), usize>;
