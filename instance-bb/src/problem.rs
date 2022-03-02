use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use branch_bound::BBProblem;
use graph::{algorithms, Graph, GraphImpl};
use instance::Instance;

use crate::{
    edge::{Edge, EdgeTuple},
    solution::Solution,
    subproblem_iterator::SubproblemIterator,
};

pub struct BaseProblem {
    instance: Arc<Instance>,
    edge_to_index: HashMap<Edge, usize>,
    index_to_edge: Vec<Edge>,
}

impl BaseProblem {
    pub fn new(instance: Arc<Instance>) -> Self {
        let mut edge_to_index = HashMap::new();
        let mut index_to_edge = Vec::new();

        let mut i = 0;

        for (u, v) in instance.graph().edges() {
            index_to_edge.push(Edge { u, v });
            edge_to_index.insert(Edge { u, v }, i);

            i += 1;
        }

        Self {
            instance: instance.clone(),
            edge_to_index,
            index_to_edge,
        }
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn edge_to_index(&self, edge: &Edge) -> Option<&usize> {
        self.edge_to_index.get(edge)
    }

    pub fn index_to_edge(&self, index: usize) -> &Edge {
        &self.index_to_edge[index]
    }
}

pub struct Subproblem {
    base: Arc<BaseProblem>,

    added_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,
}

impl Subproblem {
    fn new(
        base: Arc<BaseProblem>,
        added_edges: HashSet<Edge>,
        removed_edges: HashSet<Edge>,
    ) -> Self {
        Self {
            base,
            added_edges,
            removed_edges,
        }
    }

    pub fn added_edges(&self) -> &HashSet<Edge> {
        &self.added_edges
    }

    pub fn removed_edges(&self) -> &HashSet<Edge> {
        &self.removed_edges
    }

    pub fn base(&self) -> Arc<BaseProblem> {
        self.base.clone()
    }

    fn from_base_problem(base: Arc<BaseProblem>) -> Self {
        Self::new(base, HashSet::new(), HashSet::new())
    }

    fn from_subproblem(subproblem: &Subproblem) -> Self {
        let base = subproblem.base.clone();
        let added_edges = subproblem.added_edges.clone();
        let removed_edges = subproblem.removed_edges.clone();

        Self::new(base, added_edges, removed_edges)
    }

    pub fn from_problem<I1: Iterator<Item = Edge>, I2: Iterator<Item = Edge>>(
        problem: &Problem,
        added_edges: I1,
        removed_edges: I2,
    ) -> Self {
        let mut base = match problem {
            Problem::Base(base_problem) => Self::from_base_problem(base_problem.clone()),
            Problem::Derived(subproblem) => Self::from_subproblem(subproblem),
        };

        for edge in added_edges {
            base.added_edges.insert(edge);
        }

        for edge in removed_edges {
            base.removed_edges.insert(edge);
        }

        base
    }

    pub fn fixed_edge(&self, edge: &Edge) -> bool {
        self.added_edges.contains(edge) || self.removed_edges.contains(edge)
    }
}

pub enum Problem {
    Base(Arc<BaseProblem>),
    Derived(Subproblem),
}

impl BBProblem for Problem {
    type Sol = Solution;

    type SubproblemIterator = SubproblemIterator;

    fn solve_relaxation(&self) -> Self::Sol {
        let tuple_edges = match self {
            Problem::Base(base_problem) => algorithms::kruskal(base_problem.instance().graph()),
            Problem::Derived(subproblem) => {
                let edge_tuple_set = |edge_struct_set: &HashSet<Edge>| {
                    edge_struct_set
                        .iter()
                        .map(|edge| (edge.u, edge.v))
                        .collect()
                };

                let initial_edges: HashSet<EdgeTuple> = edge_tuple_set(&subproblem.added_edges());
                let forbidden_edges: HashSet<EdgeTuple> =
                    edge_tuple_set(&subproblem.removed_edges());

                algorithms::custom_kruskal(
                    subproblem.base().instance().graph(),
                    &initial_edges,
                    &forbidden_edges,
                )
            }
        };

        let edges = tuple_edges
            .iter()
            .map(|(u, v)| Edge { u: *u, v: *v })
            .collect();

        let subgraph = {
            let mut subgraph = Graph::new(match self {
                Problem::Base(base_problem) => base_problem.instance().graph().num_verts(),
                Problem::Derived(subproblem) => subproblem.base().instance().graph().num_verts(),
            });

            for (u, v) in tuple_edges {
                subgraph.add_edge(u, v).unwrap();
            }

            subgraph
        };

        let parent_problem = match self {
            Problem::Base(base_problem) => base_problem.clone(),
            Problem::Derived(subproblem) => subproblem.base().clone(),
        };

        Solution::new(edges, subgraph, parent_problem)
    }

    fn get_subproblems(&self, solution: &Self::Sol) -> Self::SubproblemIterator {
        SubproblemIterator::new(self, solution)
    }
}
