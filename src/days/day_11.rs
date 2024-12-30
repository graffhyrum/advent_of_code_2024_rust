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
        println!("Total time: {:?}", start.elapsed());
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
    let output = if depth == 1 {
        if right.is_none() {
            1
        } else {
            2
        }
    } else {
        let mut output = count_blinks(left.unwrap(), depth - 1, cache);
        if let Some(right) = right {
            output += count_blinks(right, depth - 1, cache);
        }
        output
    };
    cache.insert((stone, depth), output);
    output
}

fn process_stone(stone: Stone) -> (Option<Stone>, Option<Stone>) {
    if stone == 0 {
        (Some(1), None)
    } else if count_digits(stone) % 2 == 0 {
        let (left, right) = split_integer_into_halves(stone);
        (Some(left), Some(right))
    } else {
        (Some(stone * 2024), None)
    }
}

fn split_integer_into_halves(num: u64) -> (u64, u64) {
    let half_digits = count_digits(num) / 2;
    let divisor = 10u64.pow(half_digits);
    (num / divisor, num % divisor)
}

fn count_digits(num: u64) -> u32 {
    if num == 0 {
        1
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
