use std::collections::{HashMap, HashSet};

use branch_bound as bb;
use branch_bound::{SolutionCost, Variable, BBProblem};
use graph::{neighbors, properties};
use graph::{GraphImpl, Subgraph};
use instance::{Instance, WGraph};

#[derive(PartialEq, Eq, Hash)]
struct Edge {
    u: usize,
    v: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EdgeWeight {
    weight: usize,
}

struct Solution<'a> {
    edges: HashSet<Edge>,
    subgraph: Subgraph<'a, WGraph>,
    parent_problem: &'a Problem<'a>,
}

impl<'a> Solution<'a> {
    fn satisfies_dependencies(&self) -> bool {
        self.edges.iter().all(|edge| {
            let num_deps = num_deps(self, edge);
            let edge_tuple = (edge.u, edge.v);
            let lb = *self
                .parent_problem
                .instance
                .dep_lb()
                .get(&edge_tuple)
                .unwrap();
            let ub = *self
                .parent_problem
                .instance
                .dep_ub()
                .get(&edge_tuple)
                .unwrap();

            lb <= num_deps && num_deps <= ub
        })
    }
}

fn num_deps(sol: &Solution, edge: &Edge) -> usize {
    let e_idx = sol
        .parent_problem
        .edge_to_index
        .get(edge)
        .expect("instance does not contain such an edge");

    neighbors::in_neighbors(sol.parent_problem.instance.dependencies(), *e_idx)
        .filter(|dep_idx| {
            sol.edges
                .contains(&sol.parent_problem.index_to_edge[*dep_idx])
        })
        .count()
}

struct Problem<'a> {
    instance: &'a Instance,
    edge_to_index: HashMap<Edge, usize>,
    index_to_edge: Vec<Edge>,
}

impl<'a> Problem<'a> {
    fn new(instance: &'a Instance) -> Self {
        let mut edge_to_index = HashMap::new();
        let mut index_to_edge = Vec::new();

        let mut i = 0;

        for (u, v) in instance.graph().edges() {
            index_to_edge.push(Edge { u, v });
            edge_to_index.insert(Edge { u, v }, i);

            i += 1;
        }

        Problem {
            instance,
            edge_to_index,
            index_to_edge,
        }
    }
}

impl Variable for Edge {
    type ValueType = bool;
}

impl SolutionCost for EdgeWeight {}

impl<'a> bb::Solution for Solution<'a> {
    type Var = Edge;
    type SolCost = EdgeWeight;

    fn is_feasible(&self) -> bool {
        properties::is_spanning_tree(&self.subgraph) && self.satisfies_dependencies()
    }

    fn get_cost(&self) -> EdgeWeight {
        let wg = self.subgraph.parent();

        EdgeWeight {
            weight: self
                .edges
                .iter()
                .map(|edge| wg.get_edge_weight(edge.u, edge.v).unwrap())
                .sum(),
        }
    }

    fn get_value(&self, var: &Edge) -> bool {
        self.edges.contains(var)
    }
}

impl<'a> BBProblem for Problem<'a> {
    type Sol = Solution<'a>;

    type SubproblemIterator;

    fn solve_relaxation(&self) -> Self::Sol {
        todo!()
    }

    fn get_subproblems(&self) -> Self::SubproblemIterator {
        todo!()
    }
}
