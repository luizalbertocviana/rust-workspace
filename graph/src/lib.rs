mod digraph;
mod graph;
mod subgraph;

pub use crate::digraph::Digraph;
pub use crate::graph::Graph;
pub use crate::subgraph::Subgraph;

type Result<'a> = std::result::Result<(), &'a str>;

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

        sg.remove_edge(1,2).unwrap();

        assert_eq!(g.has_edge(1, 2), true);
        assert_eq!(sg.has_edge(1, 2), false);
        assert_eq!(sg.remove_edge(1, 2).is_err(), true);

        sg.add_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), true);
        assert_eq!(sg.has_edge(1, 2), true);
        assert_eq!(sg.add_edge(1, 2).is_err(), true);

        let g = Graph::new(10);

        assert_eq!(g.num_verts(), 10);
        assert_eq!(g.num_edges(), 0);

        let mut sg = Subgraph::from_graph(&g);

        assert_eq!(sg.num_verts(), 10);
        assert_eq!(sg.num_edges(), 0);

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), false);

        sg.add_edge(1,2).unwrap();

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), true);
        assert_eq!(sg.add_edge(1, 2).is_err(), true);

        sg.remove_edge(1, 2).unwrap();

        assert_eq!(g.has_edge(1, 2), false);
        assert_eq!(sg.has_edge(1, 2), false);
        assert_eq!(sg.remove_edge(1, 2).is_err(), true);
    }
}
