use std::{
    // for mapping edges to dependency lower and upper bounds
    collections::HashMap,
    // for reading/writing instances from/to files
    fs::File,
    io::{BufRead, BufReader, Write},
};
// graph facilities
use graph::{Digraph, Edge, GraphImpl, WeightedGraph};
// some type aliases
pub type WGraph = WeightedGraph<u64>;
pub type DependencyBound = HashMap<Edge, usize>;
/// instance representation (G, D, l, u), where G = (V, E, w) is a
/// weighted graph, D = (E, A) represents the dependency relations
/// among edges, and l and u set lower and upper bounds, respectively,
/// to the number of dependencies an edge must satisfy
pub struct Instance {
    graph: WGraph,
    dependencies: Digraph,
    dep_lb: DependencyBound,
    dep_ub: DependencyBound,
}
// constructors and file operations
impl Instance {
    /// creates a new instance from a weighted graph, a digraph and
    /// two mappings from edges to dependency bounds
    pub fn new(
        g: WGraph,
        d: Digraph,
        l: DependencyBound,
        u: DependencyBound,
    ) -> Self {
        Self {
            graph: g,
            dependencies: d,
            dep_lb: l,
            dep_ub: u,
        }
    }
    /// writes instance to a set of files: graph_file will encode the
    /// weighted graph, dep_file will encode the dependency relation
    /// digraph, and bounds_file will store linewise each edge's
    /// endpoints and dependency bounds
    pub fn to_files(&self, graph_file: &str, dep_file: &str, bounds_file: &str) {
        self.graph.to_file(graph_file);
        self.dependencies.to_file(dep_file);

        let mut file = File::create(bounds_file)
            .expect(format!("Instance::to_file: could not create file {}", bounds_file).as_str());

        let mut writer = |s: &str| {
            writeln!(file, "{}", s).expect(
                format!(
                    "Instance::to_file: error while writing to file {}",
                    bounds_file
                )
                .as_str(),
            )
        };

        for e in self.graph.edges() {
            let (i, j) = e;
            let l = self.dep_lb.get(&e).unwrap();
            let u = self.dep_ub.get(&e).unwrap();

            writer(format!("{} {} {} {}", i, j, l, u).as_str());
        }
    }
    /// creates an instance object from a set of files encoding it:
    /// graph_file encodes the weighted graph, dep_file encodes the
    /// dependency relation digraph, and bounds_file stores linewise
    /// each edge's endpoints and dependency bounds
    pub fn from_files(graph_file: &str, dep_file: &str, bounds_file: &str) -> Self {
        let g = WeightedGraph::from_file(graph_file);
        let d = Digraph::from_file(dep_file);

        let mut dep_lb = HashMap::new();
        let mut dep_ub = HashMap::new();

        let file = File::open(bounds_file)
            .expect(format!("Instance::from_files: could not open file {}", bounds_file).as_str());

        for line_result in BufReader::new(file).lines() {
            let line = line_result.expect(
                format!(
                    "Instance::from_files: error while reading file {}",
                    bounds_file
                )
                .as_str(),
            );

            let mut words = line.split(' ');

            let mut parser = || -> usize { words.next().unwrap().parse().unwrap() };

            let i = parser();
            let j = parser();
            let l = parser();
            let u = parser();

            let edge = (i, j);

            dep_lb.insert(edge, l);
            dep_ub.insert(edge, u);
        }

        Self {
            graph: g,
            dependencies: d,
            dep_lb,
            dep_ub,
        }
    }
}
