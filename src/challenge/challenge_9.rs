use std::{cmp::minmax, ops::Range};

use anyhow::{Ok, Result, anyhow};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nalgebra::{Point2, point};
use ndarray::{Array2, Axis, Ix2, azip, s};

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

fn rect_area(a: &Point2<i64>, b: &Point2<i64>) -> i64 {
    (i64::abs(a.x - b.x) + 1) * (i64::abs(a.y - b.y) + 1)
}

fn flood_fill(start: Ix2, map: &mut Array2<bool>) {
    let mut stack: Vec<Ix2> = vec![start];

    while let Some(point) = stack.pop() {
        map[point] = true;

        [
            (point[0].wrapping_sub(1), point[1]),
            (point[0] + 1, point[1]),
            (point[0], point[1].wrapping_sub(1)),
            (point[0], point[1] + 1),
        ]
        .into_iter()
        .filter(|(x, y)| *x < map.shape()[0] && *y < map.shape()[1])
        .map(|(x, y)| Ix2(x, y))
        .filter(|p| !map[*p])
        .for_each(|p| stack.push(p));
    }
}

struct OrdinalMap {
    ranges: Vec<Range<i64>>,
    coord_cache: FxHashMap<i64, usize>,
}

impl OrdinalMap {
    fn from_numbers(numbers: impl Iterator<Item = i64>) -> Self {
        let ranges = numbers
            .sorted()
            .dedup()
            .flat_map(|x| [x, x + 1])
            .chain(std::iter::once(i64::MAX))
            .scan(0, |prev, num| {
                let ret = (*prev)..num;
                *prev = num;
                Some(ret)
            })
            .collect::<Vec<_>>();

        let coord_cache = ranges
            .iter()
            .enumerate()
            .map(|(i, r)| (r.start, i))
            .collect::<FxHashMap<_, _>>();

        Self {
            ranges,
            coord_cache,
        }
    }

    fn length_of_range_inclusive(&self, a: usize, b: usize) -> i64 {
        let [min_idx, max_idx] = minmax(a, b);
        self.ranges[max_idx].end - self.ranges[min_idx].start
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let re = regex::Regex::new("(\\d+),(\\d+)")?;
        let points = input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .exactly_one()
                    .expect("invalid format")
                    .extract()
                    .1
            })
            .map(|[x, y]| -> Result<_> { Ok(point![x.parse::<i64>()?, y.parse::<i64>()?,]) })
            .collect::<Result<Vec<_>>>()?;

        let largest_rect = points
            .iter()
            .tuple_combinations()
            .map(|(a, b)| rect_area(a, b))
            .max()
            .ok_or(anyhow!("No points"))?;

        let x_ordinals = OrdinalMap::from_numbers(points.iter().map(|p| p.x));
        let y_ordinals = OrdinalMap::from_numbers(points.iter().map(|p| p.y));

        let new_points = points
            .iter()
            .map(|p| Ix2(x_ordinals.coord_cache[&p.x], y_ordinals.coord_cache[&p.y]))
            .collect::<Vec<_>>();

        println!("{new_points:?}");

        let mut grid = Array2::from_shape_simple_fn(
            (x_ordinals.ranges.len(), y_ordinals.ranges.len()),
            || false,
        );

        new_points
            .iter()
            .fold(new_points.last().unwrap(), |prev, this| {
                grid[*prev] = true;
                if prev[0] == this[0] {
                    let [a, b] = minmax(prev[1], this[1]);
                    (a..=b).for_each(|p| grid[Ix2(prev[0], p)] = true);
                } else if prev[1] == this[1] {
                    let [a, b] = minmax(prev[0], this[0]);
                    (a..=b).for_each(|p| grid[Ix2(p, prev[1])] = true);
                } else {
                    panic!("Can't do diagonals!");
                }
                grid[*this] = true;

                this
            });

        let grid_disp = grid
            .axis_iter(Axis(1))
            .map(|ax| {
                ax.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n");
        println!("{grid_disp}\n");

        let mut outside = grid.clone();
        flood_fill(Ix2(0, 0), &mut outside);

        azip!((a in &mut grid, b in &outside) *a = *a | (!b));

        let grid_disp = grid
            .axis_iter(Axis(1))
            .map(|ax| {
                ax.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n");
        println!("{grid_disp}\n");

        let (a, b) = new_points
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let [min_x, max_x] = minmax(a[0], b[0]);
                let [min_y, max_y] = minmax(a[1], b[1]);
                grid.slice(s![min_x..=max_x, min_y..=max_y])
                    .iter()
                    .all(|b| *b)
            })
            .max_by_key(|(a, b)| {
                x_ordinals.length_of_range_inclusive(a[0], b[0])
                    * y_ordinals.length_of_range_inclusive(a[1], b[1])
            })
            .ok_or(anyhow!("No points"))?;

        println!("{a:?}, {b:?}");
        let largest_rect_within = x_ordinals.length_of_range_inclusive(a[0], b[0])
            * y_ordinals.length_of_range_inclusive(a[1], b[1]);

        Ok(AOCResult {
            part_1: largest_rect.to_string(),
            part_2: largest_rect_within.to_string(),
        })
    }
}
