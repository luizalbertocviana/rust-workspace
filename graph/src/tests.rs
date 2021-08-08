use super::*;

// tests for the Digraph type
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
    assert_eq!(it.next(), Some((1, 2)));
    assert_eq!(it.next(), Some((3, 5)));
    assert_eq!(it.next(), None);
}
// tests for the Graph type
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
// tests for the Subgraph type
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
// tests for the WeightedGraph type
#[test]
fn weighted_graph_test() {
    let mut wg: WeightedGraph<usize> = WeightedGraph::new(10);

    assert_eq!(wg.num_verts(), 10);
    assert_eq!(wg.num_edges(), 0);

    wg.add_edge(0, 1).unwrap();
    assert_eq!(wg.has_edge(0, 1), true);
    assert_eq!(wg.has_edge(1, 0), true);
    assert_eq!(wg.get_edge_weight(0, 1), Some(&Default::default()));
    assert_eq!(wg.get_edge_weight(1, 0), Some(&Default::default()));

    wg.set_edge_weight(1, 0, 5);
    assert_eq!(wg.get_edge_weight(0, 1), Some(&5));
    assert_eq!(wg.get_edge_weight(1, 0), Some(&5));

    wg.remove_edge(0, 1).unwrap();
    assert_eq!(wg.get_edge_weight(0, 1), None);
    assert_eq!(wg.get_edge_weight(1, 0), None);
    assert_eq!(wg.remove_edge(1, 0).is_err(), true);
}
// tests for the functions of properties module
#[test]
fn properties_test() {
    let g = Graph::complete(3);

    assert_eq!(properties::is_acyclic(&g), false);

    let mut sg = Subgraph::from_graph(&g);

    sg.remove_edge(0, 1).unwrap();

    assert_eq!(properties::is_acyclic(&sg), true);

    sg.remove_edge(0, 2).unwrap();
    assert_eq!(properties::is_acyclic(&sg), true);
    assert_eq!(properties::is_connected(&sg), false);
    assert_eq!(properties::is_spanning_tree(&sg), false);

    sg.add_edge(0, 1).unwrap();
    assert_eq!(properties::is_connected(&sg), true);
    assert_eq!(properties::is_spanning_tree(&sg), true);
}
#[test]
fn num_neighbors_test() {
    let mut d = Digraph::new(4);

    assert_eq!(neighbors::num_in_neighbors(&d, 0), 0);

    d.add_edge(1, 0).unwrap();
    assert_eq!(neighbors::num_in_neighbors(&d, 0), 1);

    d.add_edge(2, 0).unwrap();
    d.add_edge(3, 0).unwrap();
    assert_eq!(neighbors::num_in_neighbors(&d, 0), 3);

    let mut in_it = neighbors::in_neighbors(&d, 0);

    assert_eq!(Some(1), in_it.next());
    assert_eq!(Some(2), in_it.next());
    assert_eq!(Some(3), in_it.next());
    assert_eq!(None, in_it.next());

    assert_eq!(neighbors::num_out_neighbors(&d, 0), 0);
    assert_eq!(neighbors::num_out_neighbors(&d, 1), 1);
    assert_eq!(neighbors::num_out_neighbors(&d, 2), 1);
    assert_eq!(neighbors::num_out_neighbors(&d, 3), 1);
}
#[test]
fn algorithms_test() {
    let weighted_edges = vec![
        (0, 1, 1),
        (0, 2, 9),
        (0, 5, 14),
        (1, 2, 10),
        (1, 3, 15),
        (2, 3, 11),
        (2, 5, 2),
        (3, 4, 6),
        (4, 5, 9),
    ];

    let wg = WeightedGraph::from_weighted_edges(6, weighted_edges);

    let mst_edges = algorithms::kruskal(&wg);

    assert_eq!(mst_edges.len(), 5);
    assert_eq!(
        mst_edges
            .iter()
            .map(|(u, v)| wg.get_edge_weight(*u, *v).unwrap())
            .sum::<usize>(),
        27
    );

    let solution = vec![(0, 1), (0, 2), (2, 5), (3, 4), (4, 5)];

    for edge in solution {
        assert_eq!(mst_edges.contains(&edge), true);
    }
}
