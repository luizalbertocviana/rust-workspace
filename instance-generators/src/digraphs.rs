// we are goiing to use this as a queue
use std::collections::VecDeque;

use rand::{prelude::SliceRandom, thread_rng};

use graph::*;

fn random_arborescence(n: usize, branching_factor: usize) -> Digraph {
    let mut arborescence = Digraph::new(n);

    let root = 0;
    //// we are shuffling vertices from 1 to n - 1 because 0 is always
    //// going to be used as root
    let mut random_vertices_ordering: Vec<usize> = (1..n).collect();
    random_vertices_ordering.shuffle(&mut thread_rng());

    let mut remaining_vertices = VecDeque::with_capacity(n);
    remaining_vertices.push_back(root);

    let mut i = 0;
    while !remaining_vertices.is_empty() && i < n - 1 {
        let u = remaining_vertices.pop_front().unwrap();

        for _ in 0..branching_factor {
            if i < n - 1 {
                let v = random_vertices_ordering[i];
                i += 1;

                arborescence.add_edge(u, v).unwrap();

                remaining_vertices.push_back(v);
            }
        }
    }

    arborescence
}
