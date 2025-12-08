use std::{cmp::Reverse, collections::VecDeque};

use anyhow::{Result, anyhow};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nalgebra::{Point3, distance, point};
use ndarray::prelude::*;

use super::{AOCChallenge, AOCResult};

#[derive(Debug, Default)]
pub struct Challenge;

impl Challenge {
    fn connect(num_nodes: usize, adj_table: &Array2<bool>, start_point: usize) -> FxHashSet<usize> {
        let mut visited = FxHashSet::default();
        let mut to_visit: VecDeque<usize> = VecDeque::from([start_point]);

        while let Some(curr) = to_visit.pop_front() {
            visited.insert(curr);
            to_visit.extend(
                adj_table
                    .slice(s![curr, ..])
                    .indexed_iter()
                    .filter_map(|(ix, b)| (*b && !visited.contains(&ix)).then_some(ix)),
            );
        }

        visited
    }

    fn get_connected_components(
        num_nodes: usize,
        adj_table: &Array2<bool>,
    ) -> (Vec<FxHashSet<usize>>, FxHashMap<usize, usize>) {
        let mut to_visit = FxHashSet::from_iter(0..num_nodes);
        let mut component_map: FxHashMap<usize, usize> = Default::default();
        let mut components = Vec::new();

        while let Some(&start_point) = to_visit.iter().next() {
            let component = Self::connect(num_nodes, &adj_table, start_point);
            to_visit = to_visit.difference(&component).copied().collect();
            component_map.extend(component.iter().map(|i| (*i, components.len())));
            components.push(component);
        }

        (components, component_map)
    }

    fn update_connected_components_until_all_connected(
        num_nodes: usize,
        adj_table: &mut Array2<bool>,
        components: Vec<FxHashSet<usize>>,
        mut component_map: FxHashMap<usize, usize>,
        mut new_connections: impl Iterator<Item = (usize, usize)>,
    ) -> (usize, usize) {
        let mut components: FxHashMap<usize, FxHashSet<usize>> =
            components.into_iter().enumerate().collect();

        while let Some((a, b)) = new_connections.next() {
            adj_table[Ix2(a, b)] = true;
            adj_table[Ix2(b, a)] = true;

            let a_comp = component_map[&a];
            let b_comp = component_map[&b];

            if a_comp != b_comp {
                component_map
                    .iter_mut()
                    .filter(|(_, c_comp)| **c_comp == b_comp)
                    .for_each(|(_, c_comp)| *c_comp = a_comp);

                let [a_mut, b_mut] = components.get_disjoint_mut([&a_comp, &b_comp]);
                let a_mut = a_mut.unwrap();
                let b_mut = b_mut.unwrap();
                *a_mut = &*a_mut | &*b_mut;

                components.remove(&b_comp);
            }

            if components.len() == 1 {
                return (a, b);
            }
        }

        (0, 0)
    }
}

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let re = regex::Regex::new("(\\d+),(\\d+),(\\d+)")?;
        let junctions: Vec<Point3<f32>> = input
            .lines()
            .map(|line| {
                re.captures_iter(line)
                    .exactly_one()
                    .expect("invalid format")
                    .extract::<3>()
                    .1
            })
            .map(|[x, y, z]| -> Result<_> {
                Ok(point![
                    x.parse::<f32>()?,
                    y.parse::<f32>()?,
                    z.parse::<f32>()?,
                ])
            })
            .collect::<Result<Vec<_>>>()?;

        let pairs = junctions
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((ai, a), (bi, b))| ((ai, bi), distance(&a, &b)));

        let mut adj_table: Array2<bool> = Array2::default((junctions.len(), junctions.len()));

        let mut sorted_pairs =
            lazysort::SortedBy::sorted_by(pairs, |(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                .map(|((a, b), d)| (a, b));

        sorted_pairs.by_ref().take(1000).for_each(|(a, b)| {
            adj_table[Ix2(a, b)] = true;
            adj_table[Ix2(b, a)] = true;
        });

        let (components, component_map) =
            Self::get_connected_components(junctions.len(), &adj_table);

        let largest_components: usize = components
            .iter()
            .map(|component| component.len())
            .sorted_by_key(|len| Reverse(*len))
            .take(3)
            .product();

        let (last_a, last_b) = Self::update_connected_components_until_all_connected(
            junctions.len(),
            &mut adj_table,
            components,
            component_map,
            sorted_pairs,
        );

        let last_product = (junctions[last_a].x as u64) * (junctions[last_b].x as u64);

        Ok(AOCResult {
            part_1: largest_components.to_string(),
            part_2: last_product.to_string(),
        })
    }
}
