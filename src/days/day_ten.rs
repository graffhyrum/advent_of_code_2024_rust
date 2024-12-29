use crate::problems::Problem;

/*
https://adventofcode.com/2024/day/10
 */
pub struct DayTen;

// Find hiking paths from topological map input
// Every 0 is a trailhead
// From any trailhead, moving only in cardinal directions,
// score the trailhead by the number of paths from it to 9s
impl Problem for DayTen {
    // get the sum of the number of 9s reachable from each 0
    fn part_one(&self, input: &str) -> String {
        let map = Map::new(input);
        map.score().to_string()
    }
    // get the sum of the number of distinct hiking trails from each trailhead
    fn part_two(&self, input: &str) -> String {
        let map = Map::new(input);
        map.rating().to_string()
    }
}

struct Map {
    trailheads: Vec<Trailhead>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let trailheads = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| {
                        if cell == 0 {
                            Some(Trailhead::new(
                                &Coordinate {
                                    x: x as i32,
                                    y: y as i32,
                                },
                                &map,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Trailhead>>()
            })
            .collect();
        Self { trailheads }
    }

    fn score(&self) -> u32 {
        self.trailheads
            .iter()
            .map(|trailhead| trailhead.score)
            .sum()
    }
    
    fn rating(&self) -> u32 {
        self.trailheads
            .iter()
            .map(|trailhead| trailhead.rating)
            .sum()
    }
}

struct Trailhead {
    score: u32,
    rating: u32,
}

impl Trailhead {
    fn new(coordinate: &Coordinate, map: &[Vec<u8>]) -> Self {
        let trails = get_trails(coordinate, map);
        let rating = get_rating(&coordinate, map);
        Self { score: trails.len() as u32, rating }
    }
}

fn get_trails(coordinate: &Coordinate, map: &[Vec<u8>]) -> Vec<Trail> {
    let mut trails = vec![];
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut stack = vec![(coordinate.clone(), vec![])];

    while let Some((current, trail)) = stack.pop() {
        // boundary check
        if current.y < 0 || current.y >= map.len() as i32 || current.x < 0 || current.x >= map[0].len() as i32 {
            continue;
        }

        if visited[current.y as usize][current.x as usize] {
            continue;
        }
        visited[current.y as usize][current.x as usize] = true;

        if map[current.y as usize][current.x as usize] == 9 {
            trails.push(trail);
            continue;
        }

        for (dx, dy, direction) in &[
            (0, -1, Direction::North),
            (0, 1, Direction::South),
            (1, 0, Direction::East),
            (-1, 0, Direction::West),
        ] {
            let new_x = current.x + dx;
            let new_y = current.y + dy;

            // boundary check for new coordinates
            if new_y < 0 || new_y >= map.len() as i32 || new_x < 0 || new_x >= map[0].len() as i32 {
                continue;
            }

            // check if the neighboring cell is 1 higher than the current cell
            if map[new_y as usize][new_x as usize] == map[current.y as usize][current.x as usize] + 1 {
            let mut new_trail = trail.clone();
            new_trail.push((current.clone(), direction.clone()));
            stack.push((
                Coordinate {
                        x: new_x,
                        y: new_y,
                },
                new_trail,
            ));
        }
    }
    }
    trails
}


// the rating is the number of distinct trails from the trailhead
fn get_rating(coordinate: &Coordinate, map: &[Vec<u8>]) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut stack = vec![coordinate.clone()];

    while let Some(current) = stack.pop() {
        // boundary check
        if current.y < 0 || current.y >= map.len() as i32 || current.x < 0 || current.x >= map[0].len() as i32 {
            continue;
        }

        if visited[current.y as usize][current.x as usize] {
            continue;
        }
        visited[current.y as usize][current.x as usize] = true;

        for (dx, dy) in &[
            (0, -1),
            (0, 1),
            (1, 0),
            (-1, 0),
        ] {
            let new_x = current.x + dx;
            let new_y = current.y + dy;

            // boundary check for new coordinates
            if new_y < 0 || new_y >= map.len() as i32 || new_x < 0 || new_x >= map[0].len() as i32 {
                continue;
            }

            // check if the neighboring cell is 1 higher than the current cell
            if map[new_y as usize][new_x as usize] == map[current.y as usize][current.x as usize] + 1 {
                stack.push(Coordinate {
                    x: new_x,
                    y: new_y,
                });
            }
        }
    }
    visited.iter().flatten().filter(|&&v| v).count() as u32
}

type Trail = Vec<Move>;
type Move = (Coordinate, Direction);
#[derive(Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}
#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
