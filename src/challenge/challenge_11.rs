use anyhow::anyhow;
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use itertools::Itertools;
use petgraph::{
    acyclic::Acyclic,
    algo::{all_simple_paths, toposort},
    data::Build,
    prelude::*,
    visit::EdgeFiltered,
};

use super::{AOCChallenge, AOCResult};

struct Bin;
impl<A> FromIterator<A> for Bin {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self
    }
}

#[derive(Debug, Default)]
pub struct Challenge;

impl AOCChallenge for Challenge {
    fn run(self, input: &str) -> anyhow::Result<AOCResult> {
        let mut graph: Acyclic<StableDiGraph<&str, ()>> = Acyclic::new();

        let node_indices = input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .chain(std::iter::once(("out", "")))
            .map(|(node, _)| (node, graph.add_node(node)))
            .collect::<FxHashMap<_, _>>();

        let edges = input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .flat_map(|(input, outputs)| {
                outputs
                    .split_whitespace()
                    .map(|output| (node_indices[input], node_indices[output]))
            })
            .try_for_each(|(a, b)| graph.try_add_edge(a, b, ()).map(|_| ()))
            .map_err(|_| anyhow!("Tried to create an edge cycle"))?;

        // let you = node_indices["you"];
        let out = node_indices["out"];

        // let path_count =
        //     all_simple_paths::<Vec<_>, _, FxBuildHasher>(&graph, you, out, 0, None).count();
        let path_count = 0;

        let svr = node_indices["svr"];
        let fft = node_indices["fft"];
        let dac = node_indices["dac"];

        let sorted = toposort(&graph, None).unwrap();

        let [(first_pos, &first), (last_pos, &last)] = sorted
            .iter()
            .enumerate()
            .filter(|(_, node)| **node == fft || **node == dac)
            .next_array()
            .unwrap();

        let start_to_first = graph.filter_map(
            |ni, n| sorted[0..=first_pos].contains(&ni).then_some(n),
            |_, e| Some(e),
        );
        let paths_start_to_first =
            all_simple_paths::<Bin, _, FxBuildHasher>(&start_to_first, svr, first, 0, None).count();

        let first_to_last = graph.filter_map(
            |ni, n| sorted[first_pos..=last_pos].contains(&ni).then_some(n),
            |_, e| Some(e),
        );
        let paths_first_to_last =
            all_simple_paths::<Bin, _, FxBuildHasher>(&first_to_last, first, last, 0, None).count();

        let last_to_end = graph.filter_map(
            |ni, n| sorted[last_pos..].contains(&ni).then_some(n),
            |_, e| Some(e),
        );
        let paths_last_to_end =
            all_simple_paths::<Bin, _, FxBuildHasher>(&last_to_end, last, out, 0, None).count();

        let path_count_via = paths_start_to_first * paths_first_to_last * paths_last_to_end;

        Ok(AOCResult {
            part_1: path_count.to_string(),
            part_2: path_count_via.to_string(),
        })
    }
}
