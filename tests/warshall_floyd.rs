use petgraph::{graph::node_index, prelude::*};
use rand::prelude::*;
use std::collections::HashMap;
use warshall_floyd_bench::*;

fn run<F>(n: usize, p: f32, f: F)
where
    F: Fn(&UnGraph<(), ()>) -> HashMap<(NodeIndex, NodeIndex), f32>,
{
    for seed in 0..100 {
        let mut rng = StdRng::seed_from_u64(seed);
        let graph = generate_graph(n, p, &mut rng);
        let actual = f(&graph);
        let expected = warshall_floyd_petgraph(&graph);
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_warshall_floyd_vec1d_min() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_vec1d_min(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[u * n + v] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[u * n + v]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_vec1d_if() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_vec1d_if(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[u * n + v] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[u * n + v]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_vec2d_min() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_vec2d_min(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[u][v] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[u][v]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_vec2d_if() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_vec2d_if(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[u][v] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[u][v]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_ndarray2d() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_ndarray2d_min(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[[u, v]] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[[u, v]]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_ndarray2d_if() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_ndarray2d_if(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[[u, v]] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[[u, v]]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_nalgebra() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_nalgebra2d_min(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[(u, v)] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[(u, v)]);
                }
            }
        }
        actual
    });
}

#[test]
fn test_warshall_floyd_nalgebra_if() {
    run(100, 0.1, |graph| {
        let result = warshall_floyd_nalgebra2d_if(&graph);
        let mut actual = HashMap::new();
        let n = graph.node_count();
        for u in 0..n {
            for v in 0..n {
                if result[(u, v)] < std::f32::MAX {
                    actual.insert((node_index(u), node_index(v)), result[(u, v)]);
                }
            }
        }
        actual
    });
}
