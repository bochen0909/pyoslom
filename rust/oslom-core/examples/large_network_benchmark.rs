use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};
use std::time::Instant;

fn create_realistic_network(num_communities: usize, community_size: usize) -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    let mut node_id = 0;
    
    println!("  Generating {} communities of {} nodes each...", num_communities, community_size);
    
    // Create communities with realistic structure
    for community in 0..num_communities {
        let start_node = node_id;
        
        // Create dense connections within community (70% connectivity)
        for i in 0..community_size {
            for j in (i + 1)..community_size {
                // Add edge with 70% probability for strong community structure
                if (i * 7 + j * 11) % 100 < 70 { // Deterministic "random" for reproducibility
                    let node1 = start_node + i;
                    let node2 = start_node + j;
                    let weight = 0.8 + ((i + j) % 5) as f64 * 0.1; // Weight between 0.8-1.2
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        // Add inter-community connections (3% probability)
        if community > 0 {
            let prev_community_start = start_node - community_size;
            for i in 0..community_size {
                for j in 0..community_size {
                    if (i * 13 + j * 17 + community * 19) % 100 < 3 {
                        let node1 = start_node + i;
                        let node2 = prev_community_start + j;
                        let weight = 0.1 + ((i + j) % 3) as f64 * 0.05; // Weak inter-community edges
                        builder.add_edge(node1, node2, weight);
                    }
                }
            }
        }
        
        node_id += community_size;
        
        if (community + 1) % 10 == 0 {
            println!("    Generated {} communities...", community + 1);
        }
    }
    
    builder.build()
}

fn benchmark_network(num_communities: usize, community_size: usize, label: &str) {
    println!("\n{} Network Test:", label);
    println!("================");
    
    let network = create_realistic_network(num_communities, community_size);
    
    println!("Network statistics:");
    println!("  Nodes: {}", network.node_count());
    println!("  Edges: {}", network.edge_count());
    println!("  Average degree: {:.1}", 2.0 * network.edge_count() as f64 / network.node_count() as f64);
    println!("  Expected communities: {}", num_communities);
    println!();
    
    // Use multi-threaded configuration (since we can't change thread pool once set)
    let config = OslomConfig {
        r: 5,
        hr: 10,
        num_threads: None, // Use all available threads
        verbose: false,
        ..Default::default()
    };
    
    println!("Running OSLOM with {} threads...", rayon::current_num_threads());
    let start = Instant::now();
    
    match run_oslom(&network, &config) {
        Ok(result) => {
            let duration = start.elapsed();
            let duration_secs = duration.as_secs_f64();
            
            println!("Completed in {:.3}s", duration_secs);
            println!("Results:");
            println!("  Found {} modules at level 0", result.statistics.num_modules);
            println!("  Modularity: {:.4}", result.statistics.modularity);
            println!("  Coverage: {}/{} ({:.1}%)", 
                     result.statistics.coverage, 
                     result.statistics.total_nodes,
                     100.0 * result.statistics.coverage as f64 / result.statistics.total_nodes as f64);
            println!("  Homeless nodes: {}", result.statistics.homeless_nodes);
            println!("Performance:");
            println!("  Nodes per second: {:.0}", network.node_count() as f64 / duration_secs);
            println!("  Edges per second: {:.0}", network.edge_count() as f64 / duration_secs);
            
            // Check quality
            let expected_communities = num_communities;
            let found_communities = result.statistics.num_modules;
            let community_accuracy = if expected_communities > 0 {
                1.0 - (expected_communities as f64 - found_communities as f64).abs() / expected_communities as f64
            } else {
                0.0
            };
            println!("  Community detection accuracy: {:.1}%", community_accuracy * 100.0);
        }
        Err(e) => {
            println!("Failed: {:?}", e);
        }
    }
}

fn main() {
    println!("OSLOM Large Network Benchmark");
    println!("=============================");
    println!("Available CPU cores: {}", 
             std::thread::available_parallelism()
                 .map(|n| n.get())
                 .unwrap_or(1));
    
    // Test progressively larger networks
    let test_cases = vec![
        (20, 100, "Medium"),      // 2,000 nodes
        (30, 150, "Large"),       // 4,500 nodes
        (40, 200, "Very Large"),  // 8,000 nodes
        (50, 250, "Huge"),        // 12,500 nodes
    ];
    
    for (num_communities, community_size, label) in test_cases {
        benchmark_network(num_communities, community_size, label);
        
        // Add separator
        println!("\n{}", "=".repeat(60));
    }
    
    println!("\nBenchmark completed!");
    println!("Note: All tests use multi-threaded execution with {} threads.", 
             rayon::current_num_threads());
}