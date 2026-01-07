use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};
use std::time::Instant;
use std::env;

fn create_test_network(num_communities: usize, community_size: usize) -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    let mut node_id = 0;
    
    for community in 0..num_communities {
        let start_node = node_id;
        
        // Create dense connections within community (75% connectivity)
        for i in 0..community_size {
            for j in (i + 1)..community_size {
                if (i * 7 + j * 11 + community * 13) % 100 < 75 {
                    let node1 = start_node + i;
                    let node2 = start_node + j;
                    let weight = 0.8 + ((i + j) % 5) as f64 * 0.1;
                    builder.add_edge(node1, node2, weight);
                }
            }
        }
        
        // Add inter-community connections (2% probability)
        if community > 0 {
            let prev_community_start = start_node - community_size;
            for i in 0..community_size {
                if (i * 17 + community * 19) % 100 < 2 {
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

fn run_benchmark(network_size: &str, num_threads: Option<usize>) -> f64 {
    let (num_communities, community_size) = match network_size {
        "small" => (15, 80),    // 1,200 nodes
        "medium" => (25, 120),  // 3,000 nodes  
        "large" => (35, 160),   // 5,600 nodes
        "huge" => (50, 200),    // 10,000 nodes
        _ => (25, 120),         // default
    };
    
    let network = create_test_network(num_communities, community_size);
    
    let config = OslomConfig {
        r: 3,  // Reduced for faster testing
        hr: 5,
        num_threads,
        verbose: false,
        ..Default::default()
    };
    
    let start = Instant::now();
    
    match run_oslom(&network, &config) {
        Ok(result) => {
            let duration = start.elapsed();
            let duration_secs = duration.as_secs_f64();
            
            println!("Network: {} nodes, {} edges", network.node_count(), network.edge_count());
            println!("Time: {:.3}s", duration_secs);
            println!("Found: {} modules (expected {})", result.statistics.num_modules, num_communities);
            println!("Modularity: {:.4}", result.statistics.modularity);
            println!("Throughput: {:.0} nodes/sec", network.node_count() as f64 / duration_secs);
            
            duration_secs
        }
        Err(e) => {
            println!("Failed: {:?}", e);
            f64::INFINITY
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage: {} <network_size> <single|multi>", args[0]);
        println!("Network sizes: small, medium, large, huge");
        return;
    }
    
    let network_size = &args[1];
    let thread_mode = &args[2];
    
    let num_threads = match thread_mode.as_str() {
        "single" => Some(1),
        "multi" => None,
        _ => {
            println!("Thread mode must be 'single' or 'multi'");
            return;
        }
    };
    
    let duration = run_benchmark(network_size, num_threads);
    println!("RESULT: {:.3}", duration);
}