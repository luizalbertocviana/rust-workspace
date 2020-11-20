// we are going to use these in a constructor
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
// we use UpperTriangularMatrix to represent our graphs
use matrix::UpperTriangularMatrix;
// Graph (and related types) implements these two traits
use crate::traits::{EdgeIterable, GraphImpl};
// utility function for adjusting endpoints of an edge
use crate::utils::adjust_endpoints;
// type aliases we use in these implementations
use crate::{Result, Edge};
/// represents an undirected graph
pub struct Graph {
    // edges are represented in the upper triangle of an upper
    // triangular matrix
    data: UpperTriangularMatrix<bool>,
    // number of vertices and undirected edges
    num_verts: usize,
    num_edges: usize,
}
// constructors
impl Graph {
    /// returns a Graph with num_verts vertices and no edges
    pub fn new(num_verts: usize) -> Self {
        let data = UpperTriangularMatrix::new(num_verts);
        let num_edges = 0;

        Self {
            data,
            num_verts,
            num_edges,
        }
    }
    /// returns a Graph with num_verts vertices and one edge between
    /// each pair of vertices
    pub fn complete(num_verts: usize) -> Self {
        let mut g = Self::new(num_verts);

        for u in 0..num_verts - 1 {
            for v in u + 1..num_verts {
                g.add_edge(u, v).unwrap();
            }
        }

        g
    }
    /// creates a new Graph from file named filename. The first line
    /// is expected to contain the number of vertices, and each
    /// subsequent line is expected to contain the endpoints of an
    /// edge
    pub fn from_file(filename: &str) -> Self {
        // opens file named filename
        let file = File::open(Path::new(filename)).expect(
            format!(
                "Graph::from_file: error while attempting to open {}",
                filename
            )
            .as_str(),
        );
        // gets a buffer for file
        let file_buffer = BufReader::new(file);
        // gets an iterator through the lines of file
        let mut file_lines = file_buffer.lines().map(|result_line| {
            result_line
                .expect("Graph::from_file: something went wrong while reading lines of a file")
        });
        // gets the first line of file
        let first_line = file_lines
            .next()
            .expect(format!("Graph::from_file: too few lines in {}", filename).as_str());
        // converts first_line into the number of vertices of the graph
        let num_verts: usize = first_line
            .parse()
            .expect("Graph::from_file: expected a nonnegative integer as the number of vertices");
        // creates a graph object with num_verts vertices
        let mut graph = Self::new(num_verts);
        // for each subsequent line of file
        for line in file_lines {
            // splits line into words
            let words = line.split_whitespace();
            // transforms each word of words into usize
            let mut endpoints = words.map(|w| {
                w.parse::<usize>()
                    .expect("Graph::from_file: expected a nonnegative integer as endpoint")
            });
            // this function gets an element from endpoints, panicking
            // when there are too few elements in endpoints
            let mut get_endpoint = || {
                endpoints.next().expect(
                    format!(
                        "Graph::from_file: too few endpoints in some line of {}",
                        filename
                    )
                    .as_str(),
                )
            };
            // gets the edge endpoints
            let u = get_endpoint();
            let v = get_endpoint();
            // adds edge with the taken endpoints to graph
            graph.add_edge(u, v).unwrap();
        }
        // returns the built graph
        graph
    }
}
// GraphImpl implementation
impl<'a> GraphImpl<'a> for Graph {
    type EdgeIterator = EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn has_edge(&self, i: usize, j: usize) -> bool {
        let mut edge = (i, j);
        adjust_endpoints(&mut edge.0, &mut edge.1);

        *self.data.const_at(edge.0, edge.1)
    }

    fn edges(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        let mut edge = (i, j);
        adjust_endpoints(&mut edge.0, &mut edge.1);

        if self.has_edge(edge.0, edge.1) {
            Err("Graph: attempting to add an existent edge")
        } else {
            *self.data.at(edge.0, edge.1) = true;
            self.num_edges += 1;

            Ok(())
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        let mut edge = (i, j);
        adjust_endpoints(&mut edge.0, &mut edge.1);

        if self.has_edge(edge.0, edge.1) {
            *self.data.at(edge.0, edge.1) = false;
            self.num_edges -= 1;

            Ok(())
        } else {
            Err("Graph: attempting to remove a nonexistent edge")
        }
    }
}
/// controls iteration through the edges of a Graph
pub struct EdgeIterator<'a> {
    // reference to the Graph whose edges are being iterated through
    parent: &'a Graph,

    current_pair: (usize, usize),
}
// constructors
impl<'a> EdgeIterator<'a> {
    // returns a new EdgeIterator
    fn new(parent: &'a Graph) -> Self {
        let current_pair = (0, 0);

        Self {
            parent,
            current_pair,
        }
    }
}
// EdgeIterable implementation
impl<'a> EdgeIterable<'a> for EdgeIterator<'a> {
    type Parent = Graph;

    fn parent(&self) -> &Graph {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        self.current_pair
    }

    fn next_pair(&mut self) {
        self.current_pair.1 += 1;

        if self.current_pair.1 == self.parent.num_verts() {
            self.current_pair.0 += 1;
            self.current_pair.1 = self.current_pair.0;
        }
    }
}
// Iterator implementation for EdgeIterator
impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        self.next_edge()
    }
}
