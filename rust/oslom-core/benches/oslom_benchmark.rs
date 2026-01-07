use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};

fn create_test_network(nodes: usize, edges_per_node: usize) -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    
    // Create a network with community structure
    let community_size = nodes / 4;
    
    // Add intra-community edges (strong)
    for community in 0..4 {
        let start = community * community_size;
        let end = ((community + 1) * community_size).min(nodes);
        
        for i in start..end {
            for j in (i + 1)..end {
                if (j - i) <= edges_per_node {
                    builder.add_edge(i, j, 1.0);
                }
            }
        }
    }
    
    // Add inter-community edges (weak)
    for i in 0..nodes {
        for j in (i + community_size)..(i + community_size + 2) {
            if j < nodes && j / community_size != i / community_size {
                builder.add_edge(i, j, 0.1);
            }
        }
    }
    
    builder.build()
}

fn benchmark_small_network(c: &mut Criterion) {
    let network = create_test_network(100, 3);
    let config = OslomConfig::default();
    
    c.bench_function("oslom_small_100_nodes", |b| {
        b.iter(|| {
            run_oslom(black_box(&network), black_box(&config)).unwrap()
        })
    });
}

fn benchmark_medium_network(c: &mut Criterion) {
    let network = create_test_network(500, 4);
    let config = OslomConfig {
        r: 5, // Reduce runs for benchmarking
        hr: 10,
        ..Default::default()
    };
    
    c.bench_function("oslom_medium_500_nodes", |b| {
        b.iter(|| {
            run_oslom(black_box(&network), black_box(&config)).unwrap()
        })
    });
}

fn benchmark_large_network(c: &mut Criterion) {
    let network = create_test_network(1000, 5);
    let config = OslomConfig {
        r: 3, // Reduce runs for benchmarking
        hr: 5,
        ..Default::default()
    };
    
    c.bench_function("oslom_large_1000_nodes", |b| {
        b.iter(|| {
            run_oslom(black_box(&network), black_box(&config)).unwrap()
        })
    });
}

fn benchmark_network_creation(c: &mut Criterion) {
    c.bench_function("network_creation_1000_nodes", |b| {
        b.iter(|| {
            create_test_network(black_box(1000), black_box(5))
        })
    });
}

criterion_group!(
    benches,
    benchmark_small_network,
    benchmark_medium_network,
    benchmark_large_network,
    benchmark_network_creation
);
criterion_main!(benches);