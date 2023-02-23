use petgraph::{graph::node_index, prelude::*};
use rand::prelude::*;
use std::collections::HashMap;

pub fn generate_graph<R: Rng>(n: usize, p: f32, rng: &mut R) -> UnGraph<(), ()> {
    let mut graph = Graph::new_undirected();
    for _ in 0..n {
        graph.add_node(());
    }
    for u in 0..n {
        for v in 0..u {
            if rng.gen::<f32>() < p {
                graph.add_edge(node_index(u), node_index(v), ());
            }
        }
    }
    graph
}

pub fn warshall_floyd_petgraph(graph: &UnGraph<(), ()>) -> HashMap<(NodeIndex, NodeIndex), f32> {
    petgraph::algo::floyd_warshall(graph, |_| 1.).unwrap()
}

pub fn warshall_floyd_vec1d_min(graph: &UnGraph<(), ()>) -> Vec<f32> {
    let n = graph.node_count();
    let mut d = vec![std::f32::MAX; n * n];
    for u in 0..n {
        d[u * n + u] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[u * n + v.index()] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i * n + j] = d[i * n + j].min(d[i * n + k] + d[k * n + j]);
            }
        }
    }
    d
}

pub fn warshall_floyd_vec1d_if(graph: &UnGraph<(), ()>) -> Vec<f32> {
    let n = graph.node_count();
    let mut d = vec![std::f32::MAX; n * n];
    for u in 0..n {
        d[u * n + u] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[u * n + v.index()] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let e = d[i * n + k] + d[k * n + j];
                if e < d[i * n + j] {
                    d[i * n + j] = e;
                }
            }
        }
    }
    d
}

pub fn warshall_floyd_vec2d_min(graph: &UnGraph<(), ()>) -> Vec<Vec<f32>> {
    let n = graph.node_count();
    let mut d = vec![vec![std::f32::MAX; n]; n];
    for u in 0..n {
        d[u][u] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[u][v.index()] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

pub fn warshall_floyd_vec2d_if(graph: &UnGraph<(), ()>) -> Vec<Vec<f32>> {
    let n = graph.node_count();
    let mut d = vec![vec![std::f32::MAX; n]; n];
    for u in 0..n {
        d[u][u] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[u][v.index()] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let e = d[i][k] + d[k][j];
                if e < d[i][j] {
                    d[i][j] = e;
                }
            }
        }
    }
    d
}

pub fn warshall_floyd_ndarray2d_min(graph: &UnGraph<(), ()>) -> ndarray::Array2<f32> {
    let n = graph.node_count();
    let mut d = ndarray::Array::from_elem((n, n), std::f32::MAX);
    for u in 0..n {
        d[[u, u]] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[[u, v.index()]] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[[i, j]] = d[[i, j]].min(d[[i, k]] + d[[k, j]]);
            }
        }
    }
    d
}

pub fn warshall_floyd_ndarray2d_if(graph: &UnGraph<(), ()>) -> ndarray::Array2<f32> {
    let n = graph.node_count();
    let mut d = ndarray::Array::from_elem((n, n), std::f32::MAX);
    for u in 0..n {
        d[[u, u]] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[[u, v.index()]] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let e = d[[i, k]] + d[[k, j]];
                if e < d[[i, j]] {
                    d[[i, j]] = e;
                }
            }
        }
    }
    d
}

pub fn warshall_floyd_nalgebra2d_min(graph: &UnGraph<(), ()>) -> nalgebra::DMatrix<f32> {
    let n = graph.node_count();
    let mut d = nalgebra::DMatrix::from_element(n, n, std::f32::MAX);
    for u in 0..n {
        d[(u, u)] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[(u, v.index())] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[(i, j)] = d[(i, j)].min(d[(i, k)] + d[(k, j)]);
            }
        }
    }
    d
}

pub fn warshall_floyd_nalgebra2d_if(graph: &UnGraph<(), ()>) -> nalgebra::DMatrix<f32> {
    let n = graph.node_count();
    let mut d = nalgebra::DMatrix::from_element(n, n, std::f32::MAX);
    for u in 0..n {
        d[(u, u)] = 0.;
        for v in graph.neighbors_undirected(node_index(u)) {
            d[(u, v.index())] = 1.;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let e = d[(i, k)] + d[(k, j)];
                if e < d[(i, j)] {
                    d[(i, j)] = e;
                }
            }
        }
    }
    d
}
