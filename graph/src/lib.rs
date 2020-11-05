mod digraph;
mod graph;
mod subgraph;
mod traits;

pub use crate::digraph::Digraph;
pub use crate::graph::Graph;
pub use crate::subgraph::Subgraph;
pub use crate::traits::GraphImpl;

type Result<'a> = std::result::Result<(), &'a str>;
type Edge = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digraph_test() {
        let mut d = Digraph::new(6);

        assert_eq!(d.num_verts(), 6);
        assert_eq!(d.num_edges(), 0);

        assert_eq!(d.has_edge(3, 5), false);
        d.add_edge(3, 5).unwrap();
        assert_eq!(d.has_edge(3, 5), true);
        assert_eq!(d.num_edges(), 1);

        assert_eq!(d.has_edge(5, 3), false);
        d.add_edge(5, 3).unwrap();
        assert_eq!(d.has_edge(5, 3), true);
        assert_eq!(d.num_edges(), 2);

        d.add_edge(1, 2).unwrap();
        assert_eq!(d.num_edges(), 3);

        d.remove_edge(5, 3).unwrap();
        assert_eq!(d.num_edges(), 2);
        assert_eq!(d.has_edge(5, 3), false);
        assert!(d.remove_edge(5, 3).is_err());

        let mut it = d.edges();
        assert_eq!(it.next(), Some((1,2)));
        assert_eq!(it.next(), Some((3,5)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn graph_test() {
        let mut g = Graph::new(8);

        assert_eq!(g.num_verts(), 8);
        assert_eq!(g.num_edges(), 0);

        assert_eq!(g.has_edge(3, 5), false);
        g.add_edge(3, 5).unwrap();
        assert_eq!(g.has_edge(3, 5), true);
        assert_eq!(g.num_edges(), 1);
        assert_eq!(g.add_edge(5, 3).is_err(), true);
        assert_eq!(g.has_edge(5, 3), true);
        assert_eq!(g.num_edges(), 1);

        g.add_edge(1, 2).unwrap();
        assert_eq!(g.num_edges(), 2);

        g.remove_edge(5, 3).unwrap();
        assert_eq!(g.num_edges(), 1);
        assert_eq!(g.has_edge(5, 3), false);
        assert!(g.remove_edge(5, 3).is_err());

        g.add_edge(2, 3).unwrap();
        g.add_edge(1, 0).unwrap();

        let mut it = g.edges();

        assert_eq!(it.next(), Some((0, 1)));
        assert_eq!(it.next(), Some((1, 2)));
        assert_eq!(it.next(), Some((2, 3)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn subgraph_test() {
        let g = Graph::complete(5);

        assert_eq!(g.num_verts(), 5);
        assert_eq!(g.num_edges(), 10);

        let mut sg = Subgraph::from_graph(&g);

        assert_eq!(sg.num_verts(), 5);
        assert_eq!(sg.num_edges(), 10);

        assert_eq!(g.has_edge(1, 2), true);
        assert_eq!(sg.has_edge(1, 2), true);

        sg.remove_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), true);
        assert_eq!(sg.has_edge(1, 2), false);
        assert_eq!(sg.remove_edge(1, 2).is_err(), true);

        sg.add_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), true);
        assert_eq!(sg.has_edge(1, 2), true);
        assert_eq!(sg.add_edge(1, 2).is_err(), true);

        let mut it = sg.edges();

        assert_eq!(it.next(), Some((0, 1)));
        assert_eq!(it.next(), Some((0, 2)));
        assert_eq!(it.next(), Some((0, 3)));
        assert_eq!(it.next(), Some((0, 4)));
        assert_eq!(it.next(), Some((1, 2)));
        assert_eq!(it.next(), Some((1, 3)));
        assert_eq!(it.next(), Some((1, 4)));
        assert_eq!(it.next(), Some((2, 3)));
        assert_eq!(it.next(), Some((2, 4)));
        assert_eq!(it.next(), Some((3, 4)));
        assert_eq!(it.next(), None);

        let g = Graph::new(10);

        assert_eq!(g.num_verts(), 10);
        assert_eq!(g.num_edges(), 0);

        let mut sg = Subgraph::from_graph(&g);

        assert_eq!(sg.num_verts(), 10);
        assert_eq!(sg.num_edges(), 0);

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), false);

        sg.add_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), true);
        assert_eq!(sg.add_edge(1, 2).is_err(), true);

        sg.remove_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), false);
        assert_eq!(sg.remove_edge(1, 2).is_err(), true);

        let mut sg2 = Subgraph::from_subgraph(&sg);

        assert_eq!(sg2.remove_edge(1, 2).is_err(), true);

        sg2.add_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), false);
        assert_eq!(sg2.has_edge(1, 2), true);

        let mut it = sg2.edges();

        assert_eq!(it.next(), Some((1, 2)));
        assert_eq!(it.next(), None);
    }
}
