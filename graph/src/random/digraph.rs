use std::collections::VecDeque;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::{digraph::Digraph, GraphImpl};

pub fn random_arborescence(n: usize, branching_factor: usize) -> Digraph {
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

pub fn random_dag(n: usize, density: f64) -> Digraph {
    let mut dag = Digraph::new(n);

    let mut rng = thread_rng();

    let mut topological_sort: Vec<usize> = (0..n).collect();
    topological_sort.shuffle(&mut rng);

    for u in 0..(n - 1) {
        for v in (u + 1)..n {
            let chance: f64 = rng.gen();

            if chance <= density {
                dag.add_edge(u, v).unwrap();
            }
        }
    }

    dag
}
