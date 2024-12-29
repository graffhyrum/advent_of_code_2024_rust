use crate::problems::Problem;

pub struct DayEleven;

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        // convert input to integer array
        let stones = parse_input(input);
        // blink 25 times
        let final_stones = (0..25).fold(stones, |stones, _| blink(stones));
        final_stones.iter().len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

type Stones = Vec<Stone>;
type Stone = u64;

fn parse_input(input: &str) -> Stones {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blink(stones: Stones) -> Stones {
    stones.iter().flat_map(|stone| process(*stone)).collect()
}

fn process(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().chars().count() % 2 == 0 {
        let count = stone.to_string().chars().count();
        let left: Stone = stone
            .to_string()
            .chars()
            .take(count / 2)
            .collect::<String>()
            .parse()
            .unwrap();
        let right: Stone = stone
            .to_string()
            .chars()
            .skip(count / 2)
            .collect::<String>()
            .parse()
            .unwrap();
        vec![left, right]
    } else {
        let new_stone = stone * 2024;
        vec![new_stone]
    }
}
