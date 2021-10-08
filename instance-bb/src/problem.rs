use std::{collections::{HashMap, HashSet}, rc::Rc};

use branch_bound::BBProblem;
use graph::{Graph, GraphImpl, algorithms};
use instance::Instance;

use crate::{edge::{Edge, EdgeTuple}, solution::Solution, subproblem_iterator::SubproblemIterator};

pub struct BaseProblem {
    instance: Box<Instance>,
    edge_to_index: HashMap<Edge, usize>,
    index_to_edge: Vec<Edge>,
}

impl BaseProblem {
    fn new(instance: Instance) -> Self {
        let mut edge_to_index = HashMap::new();
        let mut index_to_edge = Vec::new();

        let mut i = 0;

        for (u, v) in instance.graph().edges() {
            index_to_edge.push(Edge { u, v });
            edge_to_index.insert(Edge { u, v }, i);

            i += 1;
        }

        BaseProblem {
            instance: Box::new(instance),
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
    base: Rc<BaseProblem>,

    added_edges: HashSet<Edge>,
    removed_edges: HashSet<Edge>,
}

#[derive(PartialEq)]
pub enum Derivation {
    AddingEdges,
    RemovingEdges,
    NoChanges,
}

impl Subproblem {
    fn new(
        base: Rc<BaseProblem>,
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

    pub fn base(&self) -> Rc<BaseProblem> {
        self.base.clone()
    }

    fn from_base_problem(base: Rc<BaseProblem>) -> Self {
        Self::new(base, HashSet::new(), HashSet::new())
    }

    fn from_subproblem(subproblem: &Subproblem) -> Self {
        let base = subproblem.base.clone();
        let added_edges = subproblem.added_edges.clone();
        let removed_edges = subproblem.removed_edges.clone();

        Self::new(base, added_edges, removed_edges)
    }

    pub fn from_problem<I: Iterator<Item = Edge>>(
        problem: &Problem,
        derivation: &Derivation,
        edges: I,
    ) -> Self {
        let mut base = match problem {
            Problem::Base(base_problem) => Self::from_base_problem(base_problem.clone()),
            Problem::Derived(subproblem) => Self::from_subproblem(subproblem),
        };

        match derivation {
            Derivation::AddingEdges => {
                for edge in edges {
                    base.added_edges.insert(edge);
                }
            }
            Derivation::RemovingEdges => {
                for edge in edges {
                    base.removed_edges.insert(edge);
                }
            }
            Derivation::NoChanges => (),
        }

        base
    }

    pub fn fixed_edge(&self, edge: &Edge) -> bool {
        self.added_edges.contains(edge) || self.removed_edges.contains(edge)
    }
}

pub enum Problem {
    Base(Rc<BaseProblem>),
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
                let forbidden_edges: HashSet<EdgeTuple> = edge_tuple_set(&subproblem.removed_edges());

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
