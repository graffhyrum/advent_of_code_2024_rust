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

#[derive(Debug)]
struct Garden {
    plots: HashMap<(char, Point), Vec<GardenCell>>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut visited: HashSet<Point> = HashSet::new();
        let mut plots = HashMap::new();

        for (y, row) in map.iter().enumerate() {
            for (x, plant_type) in row.iter().enumerate() {
                if visited.contains(&Point { x, y }) {
                    continue;
                }
                let plot_key = (*plant_type, Point { x, y });
                let mut plot = Vec::new();

                let mut stack: Vec<Point> = vec![Point { x, y }];
                // check each direction
                while let Some(point) = stack.pop() {
                    if visited.contains(&point) {
                        continue;
                    }
                    visited.insert(point.clone());

                    let mut fences = 0;
                    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (x_offset, y_offset) in directions.iter() {
                        if let Some(neighbour) = check_bounds(
                            map[0].len(),
                            map.len(),
                            x as isize + x_offset,
                            y as isize + y_offset,
                        ) {
                            if map[neighbour.y][neighbour.x] == *plant_type {
                                stack.push(neighbour);
                            } else {
                                fences += 1;
                            }
                        } else {
                            fences += 1;
                        }
                    }
                    plot.push(GardenCell::new(point.x, point.y, fences));
                }
                if !plot.is_empty() {
                    plots.insert(plot_key, plot);
                }
            }
        }
        Garden { plots }
    }
    fn score_plots(&self) -> usize {
        let mut score = 0;
        for (_plant_type, plot) in self.plots.iter() {
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

// type VisitedKey = (char, Coordinate);

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct GardenCell {
    point: Point,
    fences: u8,
}

impl GardenCell {
    fn new(x: impl Into<usize>, y: impl Into<usize>, fences: u8) -> Self {
        GardenCell {
            point: Point {
                x: x.into(),
                y: y.into(),
            },
            fences,
        }
    }
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

        assert_eq!(day.part_one(input), "1930", "Part one failed: {:?}", Garden::new(input));
    }
}
