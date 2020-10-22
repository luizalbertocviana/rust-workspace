mod digraph;
mod graph;

use crate::digraph::Digraph;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn digraph_instantiation() {
        let mut d = Digraph::new(6);

        assert_eq!(d.num_verts(), 6);
        assert_eq!(d.num_edges(), 0);

        assert_eq!(d.has_edge(3, 5), false);
        d.add_edge(3, 5);
        assert_eq!(d.has_edge(3, 5), true);
        assert_eq!(d.num_edges(), 1);

        assert_eq!(d.has_edge(5, 3), false);
        d.add_edge(5, 3);
        assert_eq!(d.has_edge(5, 3), true);
        assert_eq!(d.num_edges(), 2);

        d.add_edge(1, 2);
        assert_eq!(d.num_edges(), 3);

        d.remove_edge(5, 3);
        assert_eq!(d.num_edges(), 2);
        assert_eq!(d.has_edge(5, 3), false);
    }
}
