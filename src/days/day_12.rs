use crate::problems::Problem;
use crate::util::Point;
use std::collections::HashSet;

pub struct DayTwelve;

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        get_regions(input)
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        get_regions(input)
            .iter()
            .map(|region| region.area * region.sides())
            .sum::<u32>()
            .to_string()
    }
}

fn get_regions(input: &str) -> Vec<Region> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &plant) in row.iter().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if !visited.contains(&point) {
                let mut region = Region::default();
                find_region(&grid, &mut visited, &mut region, point, plant);
                regions.push(region);
            }
        }
    }
    regions
}
fn find_region(
    grid: &[&[u8]],
    visited: &mut HashSet<Point>,
    region: &mut Region,
    point: Point,
    plant: u8,
) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        if let Some(&char) = row.get(point.x as usize) {
            if char == plant {
                if visited.insert(point) {
                region.area += 1;

                for neighbor in Point::von_neumann() {
                    if !find_region(grid, visited, region, point + neighbor, plant) {
                        region.perimeter += 1;
                        region.edges.insert(Edge(point, neighbor.into()));
                    }
                }
                }
                return true;
            }
        }
    }

    false
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge(Point, u8);

#[derive(Default)]
struct Region {
    area: u32,
    perimeter: u32,
    edges: HashSet<Edge>,
}

impl Region {
    fn sides(&self) -> u32 {
        let mut to_remove = HashSet::new();
        let mut sorted: Vec<_> = self.edges.iter().collect();
        sorted.sort();

        for edge in sorted {
            let sides = match edge.1 {
                b'^' | b'v' => [Point::left(), Point::right()],
                _ => [Point::up(), Point::down()],
            }
                .map(|point| Edge(edge.0 + point, edge.1));

            if sides
                .iter()
                .any(|fence| self.edges.contains(fence) && !to_remove.contains(fence))
            {
                to_remove.insert(*edge);
            }
        }

        (self.edges.len() - to_remove.len()) as u32
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

        assert_eq!(
            day.part_one(input),
            "1930"
        );
    }

    #[test]
    fn test_day_twelve_part_two() {
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

        assert_eq!(day.part_two(input), "1206");
    }
}
