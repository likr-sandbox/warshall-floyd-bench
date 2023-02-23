use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use warshall_floyd_bench::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut group = c.benchmark_group("Warshall-Floyd");
    for n in (100..=1000).step_by(100) {
        let p = 0.1;
        let graph = generate_graph(n, p, &mut rng);
        group.bench_with_input(BenchmarkId::new("petgraph", n), &graph, |bench, graph| {
            bench.iter(|| {
                let _ = warshall_floyd_petgraph(graph);
            });
        });
        group.bench_with_input(BenchmarkId::new("vec1d-min", n), &graph, |bench, graph| {
            bench.iter(|| {
                let _ = warshall_floyd_vec1d_min(graph);
            });
        });
        group.bench_with_input(BenchmarkId::new("vec1d-if", n), &graph, |bench, graph| {
            bench.iter(|| {
                let _ = warshall_floyd_vec1d_if(graph);
            });
        });
        group.bench_with_input(BenchmarkId::new("vec2d-min", n), &graph, |bench, graph| {
            bench.iter(|| {
                let _ = warshall_floyd_vec2d_min(graph);
            });
        });
        group.bench_with_input(BenchmarkId::new("vec2d-if", n), &graph, |bench, graph| {
            bench.iter(|| {
                let _ = warshall_floyd_vec2d_if(graph);
            });
        });
        group.bench_with_input(
            BenchmarkId::new("ndarray2d-min", n),
            &graph,
            |bench, graph| {
                bench.iter(|| {
                    let _ = warshall_floyd_ndarray2d_min(graph);
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("ndarray2d-if", n),
            &graph,
            |bench, graph| {
                bench.iter(|| {
                    let _ = warshall_floyd_ndarray2d_if(graph);
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("nalgebra2d-min", n),
            &graph,
            |bench, graph| {
                bench.iter(|| {
                    let _ = warshall_floyd_nalgebra2d_min(graph);
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("nalgebra2d-if", n),
            &graph,
            |bench, graph| {
                bench.iter(|| {
                    let _ = warshall_floyd_nalgebra2d_if(graph);
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
