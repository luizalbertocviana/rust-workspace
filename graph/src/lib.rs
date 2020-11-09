// digraph module
mod digraph;
// graph module
mod graph;
// subgraph module
mod subgraph;
// traits module
mod traits;
// utility functions
mod utils;
/// properties module: contains functions regarding some graph properties
pub mod properties;
/// a digraph representation
pub use crate::digraph::Digraph;
/// a graph representation
pub use crate::graph::Graph;
/// a subgraph representation
pub use crate::subgraph::Subgraph;
/// this trait provides a common interface to our graph related types
pub use crate::traits::GraphImpl;
// type alias to represent the return of a function that can return an
// error
type Result<'a> = std::result::Result<(), &'a str>;
// type alias for an edge
type Edge = (usize, usize);

#[cfg(test)]
mod tests;
