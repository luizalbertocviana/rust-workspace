// we are going to use these in a constructor
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fs::File, io::Write};
// we use Matrix as the internal representation of our digraphs
use matrix::Matrix;
// Digraph (and related types) implements these two traits
use crate::traits::{EdgeIterable, GraphImpl};
// type aliases we use in these implementations
use crate::{Edge, Result};
/// represents a directed graph
pub struct Digraph {
    // arcs are represented by an adjacency matrix
    data: Matrix<bool>,
    // number of vertices and edges (arcs)
    num_verts: usize,
    num_edges: usize,
}
// constructors
impl Digraph {
    /// creates a new Digraph with num_verts vertices and no arcs
    pub fn new(num_verts: usize) -> Self {
        let data = Matrix::square(num_verts);
        let num_edges = 0;

        Self {
            data,
            num_verts,
            num_edges,
        }
    }
    /// creates a new Digraph from file named filename. The first line
    /// is expected to contain the number of vertices, and each
    /// subsequent line is expected to contain the endpoints of an
    /// edge
    pub fn from_file(filename: &str) -> Self {
        // opens file named filename
        let file = File::open(Path::new(filename)).expect(
            format!(
                "Digraph::from_file: error while attempting to open {}",
                filename
            )
            .as_str(),
        );
        // gets a buffer for file
        let file_buffer = BufReader::new(file);
        // gets an iterator through the lines of file
        let mut file_lines = file_buffer.lines().map(|result_line| {
            result_line
                .expect("Digraph::from_file: something went wrong while reading lines of a file")
        });
        // gets the first line of file
        let first_line = file_lines
            .next()
            .expect(format!("Digraph::from_file: too few lines in {}", filename).as_str());
        // converts first_line into the number of vertices of the digraph
        let num_verts: usize = first_line
            .parse()
            .expect("Digraph::from_file: expected a nonnegative integer as the number of vertices");
        // creates a digraph object with num_verts vertices
        let mut digraph = Self::new(num_verts);
        // for each subsequent line of file
        for line in file_lines {
            // splits line into words
            let words = line.split_whitespace();
            // transforms each word of words into usize
            let mut endpoints = words.map(|w| {
                w.parse::<usize>()
                    .expect("Digraph::from_file: expected a nonnegative integer as endpoint")
            });
            // this function gets an element from endpoints, panicking
            // when there are too few elements in endpoints
            let mut get_endpoint = || {
                endpoints.next().expect(
                    format!(
                        "Digraph::from_file: too few endpoints in some line of {}",
                        filename
                    )
                    .as_str(),
                )
            };
            // gets the edge endpoints
            let u = get_endpoint();
            let v = get_endpoint();
            // adds edge with the taken endpoints to digraph
            digraph.add_edge(u, v).unwrap();
        }
        // returns the built digraph
        digraph
    }
    /// writes Digraph to file named filename. The first line contains
    /// the number of vertices, and each subsequent line contains the
    /// endpoints of an edge
    pub fn to_file(&self, filename: &str) {
        let mut file = File::create(filename)
            .expect(format!("Digraph::to_file: could not create file {}", filename).as_str());

        let mut writer = |s: &str| {
            writeln!(file, "{}", s).expect(
                format!("Digraph::to_file: error while writing to file {}", filename).as_str(),
            )
        };

        writer(&self.num_verts().to_string());

        for (u, v) in self.edges() {
            writer(format!("{} {}", u, v).as_str());
        }
    }
}
// GraphImpl implementation
impl<'a> GraphImpl<'a> for Digraph {
    type EdgeIterator = EdgeIterator<'a>;

    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn has_edge(&self, i: usize, j: usize) -> bool {
        *self.data.const_at(i, j)
    }

    fn edges(&'a self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn add_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            Err("Digraph: attempting to add existent edge")
        } else {
            *self.data.at(i, j) = true;
            self.num_edges += 1;

            Ok(())
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) -> Result {
        if self.has_edge(i, j) {
            *self.data.at(i, j) = false;
            self.num_edges -= 1;

            Ok(())
        } else {
            Err("Digraph: attempting to remove nonexistent edge")
        }
    }
}
/// controls iteration through the arcs of a Digraph
pub struct EdgeIterator<'a> {
    // reference to the Digraph whose arcs are being iterated through
    parent: &'a Digraph,
    // current arc being visited
    current_pair: (usize, usize),
}
// constructors
impl<'a> EdgeIterator<'a> {
    // returns a new EdgeIterator
    fn new(parent: &'a Digraph) -> Self {
        let current_pair = (0, 0);

        Self {
            parent,
            current_pair,
        }
    }
}
// EdgeIterable implementation
impl<'a> EdgeIterable<'a> for EdgeIterator<'a> {
    type Parent = Digraph;

    fn parent(&self) -> &Digraph {
        self.parent
    }

    fn current_pair(&self) -> (usize, usize) {
        self.current_pair
    }

    fn next_pair(&mut self) {
        self.current_pair.1 += 1;

        if self.current_pair.1 == self.parent.num_verts() {
            self.current_pair.1 = 0;
            self.current_pair.0 += 1;
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
