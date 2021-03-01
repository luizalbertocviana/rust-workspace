// we are going to use these in a constructor
use std::{fmt::Display, io::{BufRead, BufReader}};
use std::path::Path;
use std::{fs::File, io::Write};
// we use HashMap as a mapping from edges to weights
use std::collections::HashMap;
// we use Default to determine the weight of newly inserted edges
use std::default::Default;
// these trait are for ensuring that the weight tyoe can be parsed
// from a string
use std::fmt::Debug;
use std::str::FromStr;
// we use Graph as our internal graph representation
use crate::graph::Graph;
// our weighted graph representation implements GraphImpl
use crate::traits::GraphImpl;
// utility function for adjusting endpoints of an edge
use crate::utils::adjust_endpoints;
// we use these type aliases in some implementations here
use crate::{Edge, Result};
// our weighted graph representation, combining a graph representation
// eith a mapping from edges to weights
pub struct WeightedGraph<W: Default> {
    graph: Graph,
    weight_map: HashMap<Edge, W>,
}
// constructors
impl<W: Default> WeightedGraph<W> {
    /// returns a WeightedGraph with num_verts vertices and no edges
    pub fn new(num_verts: usize) -> Self {
        let graph = Graph::new(num_verts);
        let weight_map = HashMap::new();

        Self { graph, weight_map }
    }
    /// returns a WeightedGraph with num_verts vertices and edges
    pub fn from_weighted_edges(
        num_verts: usize,
        edges: impl IntoIterator<Item = (usize, usize, W)>,
    ) -> Self {
        let mut wg = Self::new(num_verts);

        for (u, v, w) in edges {
            wg.add_weighted_edge(u, v, w).unwrap();
        }

        wg
    }
}
// file facilities
impl<W: Default + Display> WeightedGraph<W> {
    /// returns a WeightedGraph built from the content of a file named
    /// filename. The first line is expected to contain the number of
    /// vertices, and each subsequent line is expected to contain the
    /// endpoints of an edge together with its weight
    pub fn from_file(filename: &str) -> Self
    where
        W: FromStr,
        <W as FromStr>::Err: Debug,
    {
        // opens file named filename
        let file = File::open(Path::new(filename)).expect(
            format!(
                "WeightedGraph::from_file: error while attempting to open {}",
                filename
            )
            .as_str(),
        );
        // gets a buffer for file
        let file_buffer = BufReader::new(file);
        // gets an iterator through the lines of file
        let mut file_lines = file_buffer.lines().map(|result_line| {
            result_line.expect(
                "WeightedGraph::from_file: something went wrong while reading lines of a file",
            )
        });
        // gets the first line of file
        let first_line = file_lines
            .next()
            .expect(format!("WeightedGraph::from_file: too few lines in {}", filename).as_str());
        // converts first_line into the number of vertices of the digraph
        let num_verts: usize = first_line.parse().expect(
            "WeightedGraph::from_file: expected a nonnegative integer as the number of vertices",
        );
        // this is to store some weighted edges
        let mut weighted_edges = Vec::new();
        // for each line of file
        for line in file_lines {
            // split line into words
            let mut words = line.split_whitespace();
            // take the first three words: the first two being
            // endpoints and the last one as the edge weight
            let opt_u = words.next();
            let opt_v = words.next();
            let opt_w = words.next();
            // parse each endpoint and weight
            let u: usize = opt_u
                .expect(
                    format!(
                        "WeightedGraph::from_file: too few words in some line of {}",
                        filename
                    )
                    .as_str(),
                )
                .parse()
                .expect("WeightedGraph::from_file: error while parsing a weighted edge");
            let v: usize = opt_v
                .expect(
                    format!(
                        "WeightedGraph::from_file: too few words in some line of {}",
                        filename
                    )
                    .as_str(),
                )
                .parse()
                .expect("WeightedGraph::from_file: expected nonnegative integer as endpoint");
            let w: W = opt_w
                .expect(
                    format!(
                        "WeightedGraph::from_file: too few words in some line of {}",
                        filename
                    )
                    .as_str(),
                )
                .parse()
                .expect("WeightedGraph::from_file: expected nonnegative integer as endpoint");

            weighted_edges.push((u, v, w));
        }

        Self::from_weighted_edges(num_verts, weighted_edges)
    }
    /// writes WeightedGraph to file named filename. The first line
    /// contains the number of vertices, and each subsequent line
    /// contains the endpoints of an edge together with its weight
    pub fn to_file(&self, filename: &str) {
        let mut file = File::create(filename)
            .expect(format!("WeightedGraph::to_file: could not create file {}", filename).as_str());

        let mut writer = |s: &str| {
            writeln!(file, "{}", s).expect(
                format!(
                    "WeightedGraph::to_file: error while writing to file {}",
                    filename
                )
                .as_str(),
            )
        };

        writer(&self.num_verts().to_string());

        for (u, v) in self.edges() {
            let w = self.get_edge_weight(u, v).unwrap();

            writer(format!("{} {} {}", u, v, w).as_str());
        }
    }
}
// GraphImpl implementation
impl<'a, W: Default> GraphImpl<'a> for WeightedGraph<W> {
    type EdgeIterator = crate::graph::EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.graph.num_verts()
    }

    fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph.has_edge(u, v)
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        self.graph.edges()
    }

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.graph.add_edge(i, j).is_ok() {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.insert((edge.0, edge.1), Default::default());

            Ok(())
        } else {
            Err("WeightedGraph: attempt to add existing edge")
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        if self.graph.remove_edge(i, j).is_ok() {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.remove(&(edge.0, edge.1));

            Ok(())
        } else {
            Err("WeightedGraph: attempt to remove nonexisting edge")
        }
    }
}

// weight related functions
impl<W: Default> WeightedGraph<W> {
    /// adds edge with endpoints i and j to WeightedGraph, also
    /// setting its weight to w. In case edge already exists, an error
    /// is returned
    pub fn add_weighted_edge(&mut self, i: usize, j: usize, w: W) -> Result {
        if self.add_edge(i, j).is_ok() {
            self.set_edge_weight(i, j, w);

            Ok(())
        } else {
            Err("WeightedGraph: attempt to add an existent edge")
        }
    }
    /// gets weight of edge (i, j) ((j, i), in case i > j). Returns
    /// None in case edge does not exist
    pub fn get_edge_weight(&self, i: usize, j: usize) -> Option<&W> {
        let mut edge = (i, j);
        adjust_endpoints(&mut edge.0, &mut edge.1);

        self.weight_map.get(&(edge.0, edge.1))
    }
    /// sets weight of edge (i, j) to w. Returns old weight in case
    /// edge exists, otherwise returns None
    pub fn set_edge_weight(&mut self, i: usize, j: usize, w: W) -> Option<W> {
        if self.graph.has_edge(i, j) {
            let mut edge = (i, j);
            adjust_endpoints(&mut edge.0, &mut edge.1);

            self.weight_map.insert((edge.0, edge.1), w)
        } else {
            None
        }
    }

    pub fn set_weighting(&mut self, weighting: HashMap<Edge, W>) {
        for ((u, v), w) in weighting {
            self.set_edge_weight(u, v, w);
        }
    }
}
