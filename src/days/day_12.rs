use crate::problems::Problem;
use std::collections::{HashMap, HashSet};

pub struct DayTwelve;

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        self.solve(input)
    }

    fn part_two(&self, input: &str) -> String {
        "Not implemented".to_string()
    }
}

impl DayTwelve {
    fn solve(&self, input: &str) -> String {
        // build garden
        let garden = Garden::new(input);
        // score plots
        garden.score_plots().to_string()
    }
}

type Regions = HashMap<(char, Point), Vec<GardenCell>>;

#[derive(Debug)]
struct Garden {
    regions: Regions,
}

impl Garden {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut visited: HashSet<Point> = HashSet::new();
        let mut regions = HashMap::new();

        for (y, row) in map.iter().enumerate() {
            for (x, plant_type) in row.iter().enumerate() {
                let point = Point { x, y };
                let mut region_cells = vec![];
                // bfs to find all cells in the region,
                let mut queue = vec![point];
                while let Some(current) = queue.pop() {
                    if visited.contains(&current) {
                        continue;
                    }
                    visited.insert(current);
                    let mut fences = 0;
                    for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        if let Some(next) = check_bounds(
                            row.len(),
                            map.len(),
                            current.x as isize + dx,
                            current.y as isize + dy,
                        ) {
                            if map[next.y][next.x] == *plant_type {
                                queue.push(next);
                            } else {
                                fences += 1; // edge of region
                            }
                        } else {
                            fences += 1; // out of bounds
                        }
                    }
                    region_cells.push(GardenCell {
                        point: current,
                        fences,
                    });
                }
                regions.insert((*plant_type, point), region_cells);
            }
        }
        Garden { regions }
    }
    fn score_plots(&self) -> usize {
        let mut score = 0;
        for (_plant_type, plot) in self.regions.iter() {
            let mut area = 0;
            let mut circumference = 0;
            for cell in plot.iter() {
                area += 1;
                circumference += cell.fences as usize;
            }
            score += area * circumference;
        }
        score
    }
}

fn check_bounds(width: usize, height: usize, x: isize, y: isize) -> Option<Point> {
    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
        Some(Point {
            x: x as usize,
            y: y as usize,
        })
    } else {
        None
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct GardenCell {
    point: Point,
    fences: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_twelve_part_one() {
        let day = DayTwelve {};
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!(
            day.part_one(input),
            "1930",
            "Part one failed: {:?}",
            Garden::new(input)
        );
    }
}
