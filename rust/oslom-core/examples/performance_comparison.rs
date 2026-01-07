use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};
use std::time::Instant;
use std::env;

fn create_test_network(num_communities: usize, community_size: usize) -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    let mut node_id = 0;
    
    // Create communities with deterministic structure for consistent benchmarking
    for community in 0..num_communities {
        let start_node = node_id;
        
        // Create dense connections within community (70% connectivity)
        for i in 0..community_size {
            for j in (i + 1)..community_size {
                // Deterministic "random" for reproducibility
                if (i * 7 + j * 11 + community * 13) % 100 < 70 {
                    let node1 = start_node + i;
                    let node2 = start_node + j;
                    let weight = 0.8 + ((i + j) % 5) as f64 * 0.1;
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        // Add inter-community connections (3% probability)
        if community > 0 {
            let prev_community_start = start_node - community_size;
            for i in 0..community_size {
                if (i * 17 + community * 19) % 100 < 3 {
                    let node1 = start_node + i;
                    let node2 = prev_community_start + (i % community_size);
                    let weight = 0.1 + ((i + community) % 3) as f64 * 0.05;
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        node_id += community_size;
    }
    
    builder.build()
}

fn run_benchmark(num_threads: Option<usize>) -> f64 {
    // Create a moderately large network for testing
    let num_communities = 25;
    let community_size = 120;
    let network = create_test_network(num_communities, community_size);
    
    let config = OslomConfig {
        r: 5,
        hr: 8,
        num_threads,
        verbose: false,
        ..Default::default()
    };
    
    let thread_label = match num_threads {
        Some(1) => "single-threaded".to_string(),
        Some(n) => format!("{}-threaded", n),
        None => format!("multi-threaded ({})", rayon::current_num_threads()),
    };
    
    println!("Running {} benchmark on network with {} nodes, {} edges...", 
             thread_label, network.node_count(), network.edge_count());
    
    let start = Instant::now();
    
    match run_oslom(&network, &config) {
        Ok(result) => {
            let duration = start.elapsed();
            let duration_secs = duration.as_secs_f64();
            
            println!("  Completed in {:.3}s", duration_secs);
            println!("  Found {} modules (expected {})", result.statistics.num_modules, num_communities);
            println!("  Modularity: {:.4}", result.statistics.modularity);
            println!("  Nodes per second: {:.0}", network.node_count() as f64 / duration_secs);
            
            duration_secs
        }
        Err(e) => {
            println!("  Failed: {:?}", e);
            f64::INFINITY
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // Run specific thread configuration
        match args[1].as_str() {
            "single" => {
                let duration = run_benchmark(Some(1));
                println!("RESULT: {:.3}", duration);
            }
            "multi" => {
                let duration = run_benchmark(None);
                println!("RESULT: {:.3}", duration);
            }
            _ => {
                println!("Usage: {} [single|multi]", args[0]);
            }
        }
    } else {
        // Run comparison (this will only work for the first thread pool configuration)
        println!("OSLOM Performance Comparison");
        println!("===========================");
        println!("Available CPU cores: {}", 
                 std::thread::available_parallelism()
                     .map(|n| n.get())
                     .unwrap_or(1));
        println!();
        
        // Test single-threaded first
        let single_time = run_benchmark(Some(1));
        println!();
        
        // Test multi-threaded (this will fail due to thread pool already being set)
        let multi_time = run_benchmark(None);
        
        if single_time < f64::INFINITY && multi_time < f64::INFINITY {
            let speedup = single_time / multi_time;
            println!("\nSpeedup: {:.2}x ({:.1}% improvement)", 
                     speedup, (speedup - 1.0) * 100.0);
        }
        
        println!("\nNote: To get accurate comparison, run:");
        println!("  cargo run --example performance_comparison --release single");
        println!("  cargo run --example performance_comparison --release multi");
    }
}