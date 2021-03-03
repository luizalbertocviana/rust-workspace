use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use graph::{Digraph, Edge, GraphImpl, WeightedGraph};

pub struct Instance {
    graph: WeightedGraph<u64>,
    dependencies: Digraph,
    dep_lb: HashMap<Edge, usize>,
    dep_ub: HashMap<Edge, usize>,
}

impl Instance {
    pub fn new(
        g: WeightedGraph<u64>,
        d: Digraph,
        l: HashMap<Edge, usize>,
        u: HashMap<Edge, usize>,
    ) -> Self {
        Self {
            graph: g,
            dependencies: d,
            dep_lb: l,
            dep_ub: u,
        }
    }

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

            let mut parser = || -> usize {words.next().unwrap().parse().unwrap()};

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

fn main() {
    println!("Hello, world!");
}
