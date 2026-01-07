use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};
use std::time::Instant;
use rand::Rng;

fn create_large_test_network(num_communities: usize, community_size: usize) -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    let mut node_id = 0;
    
    println!("  Generating {} communities...", num_communities);
    
    // Create communities with realistic structure
    for community in 0..num_communities {
        let start_node = node_id;
        
        // Create dense connections within community (80% connectivity)
        for i in 0..community_size {
            for j in (i + 1)..community_size {
                // Add edge with 80% probability for strong community structure
                if rand::random::<f64>() < 0.8 {
                    let node1 = start_node + i;
                    let node2 = start_node + j;
                    let weight = 0.8 + rand::random::<f64>() * 0.4; // Weight between 0.8-1.2
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        // Add inter-community connections (5% probability)
        if community > 0 {
            let prev_community_start = start_node - community_size;
            for i in 0..community_size {
                for j in 0..community_size {
                    if rand::random::<f64>() < 0.05 {
                        let node1 = start_node + i;
                        let node2 = prev_community_start + j;
                        let weight = 0.1 + rand::random::<f64>() * 0.2; // Weak inter-community edges
                        builder.add_edge(node1, node2, weight);
                    }
                }
            }
        }
        
        // Connect to a few random other communities for realism
        for other_community in 0..community {
            if rand::random::<f64>() < 0.3 { // 30% chance to connect to another community
                let other_start = other_community * community_size;
                for _ in 0..3 { // Add 3 random inter-community edges
                    let node1 = start_node + (rand::random::<usize>() % community_size);
                    let node2 = other_start + (rand::random::<usize>() % community_size);
                    let weight = 0.05 + rand::random::<f64>() * 0.15;
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        node_id += community_size;
        
        if (community + 1) % 10 == 0 {
            println!("  Generated {} communities...", community + 1);
        }
    }
    
    builder.build()
}

fn benchmark_oslom(network: &oslom_core::Network, config: &OslomConfig, label: &str) -> (f64, f64) {
    println!("Running {} benchmark...", label);
    let start = Instant::now();
    
    match run_oslom(network, config) {
        Ok(result) => {
            let duration = start.elapsed();
            let duration_secs = duration.as_secs_f64();
            
            println!("{} completed in {:.3}s", label, duration_secs);
            println!("  Found {} modules at level 0", result.statistics.num_modules);
            println!("  Modularity: {:.4}", result.statistics.modularity);
            println!("  Coverage: {}/{}", result.statistics.coverage, result.statistics.total_nodes);
            println!("  Nodes per second: {:.0}", network.node_count() as f64 / duration_secs);
            
            (duration_secs, result.statistics.modularity)
        }
        Err(e) => {
            println!("{} failed: {:?}", label, e);
            (f64::INFINITY, 0.0)
        }
    }
}

fn run_benchmark_suite() {
    println!("OSLOM Parallel Processing Benchmark Suite");
    println!("=========================================");
    
    let test_cases = vec![
        (10, 100, "Medium"),   // 1,000 nodes
        (25, 200, "Large"),    // 5,000 nodes  
        (50, 300, "Very Large"), // 15,000 nodes
    ];
    
    for (num_communities, community_size, label) in test_cases {
        println!("\n{} Network Test:", label);
        println!("Creating network with {} communities of {} nodes each...", 
                 num_communities, community_size);
        
        let network = create_large_test_network(num_communities, community_size);
        
        println!("Network stats:");
        println!("  Nodes: {}", network.node_count());
        println!("  Edges: {}", network.edge_count());
        println!("  Expected communities: {}", num_communities);
        println!();
        
        // Test with single thread
        let config_single = OslomConfig {
            r: 3,  // Reduced runs for faster testing
            hr: 5,
            num_threads: Some(1),
            verbose: false,
            ..Default::default()
        };
        
        let (time_single, _) = benchmark_oslom(&network, &config_single, "Single-threaded");
        
        // Test with multiple threads
        let config_multi = OslomConfig {
            r: 3,
            hr: 5,
            num_threads: None,
            verbose: false,
            ..Default::default()
        };
        
        let (time_multi, _) = benchmark_oslom(&network, &config_multi, "Multi-threaded");
        
        // Calculate and display speedup
        if time_single < f64::INFINITY && time_multi < f64::INFINITY {
            let speedup = time_single / time_multi;
            println!("Speedup: {:.2}x ({:.1}% improvement)", speedup, (speedup - 1.0) * 100.0);
        }
        
        println!();
        
        println!("----------------------------------------");
    }
}

fn main() {
    println!("Available parallelism: {} threads", 
             std::thread::available_parallelism()
                 .map(|n| n.get())
                 .unwrap_or(1));
    println!();
    
    run_benchmark_suite();
    
    println!("\nNote: Thread pool configuration is global per process.");
    println!("Larger networks show more significant parallel speedups.");
}