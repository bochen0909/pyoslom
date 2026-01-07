use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};
use std::time::Instant;

fn create_test_network() -> oslom_core::Network {
    let mut builder = NetworkBuilder::new(false);
    
    // Create 3 small communities for quick testing
    for community in 0..3 {
        let start = community * 20;
        
        // Dense connections within community
        for i in 0..20 {
            for j in (i + 1)..20 {
                if (i + j + community) % 3 == 0 {
                    builder.add_edge(start + i, start + j, 1.0);
                }
            }
        }
        
        // Weak inter-community connections
        if community > 0 {
            let prev_start = (community - 1) * 20;
            builder.add_edge(start, prev_start, 0.1);
            builder.add_edge(start + 1, prev_start + 1, 0.1);
        }
    }
    
    builder.build()
}

fn run_test(config: &OslomConfig, label: &str) {
    let network = create_test_network();
    
    println!("Running {} test...", label);
    println!("  Network: {} nodes, {} edges", network.node_count(), network.edge_count());
    
    let start = Instant::now();
    
    match run_oslom(&network, config) {
        Ok(result) => {
            let duration = start.elapsed();
            println!("  Completed in {:?}", duration);
            println!("  Found {} modules", result.statistics.num_modules);
            println!("  Modularity: {:.4}", result.statistics.modularity);
        }
        Err(e) => {
            println!("  Failed: {:?}", e);
        }
    }
    println!();
}

fn main() {
    println!("OSLOM Threading Configuration Demo");
    println!("==================================");
    println!("Available CPU cores: {}", 
             std::thread::available_parallelism()
                 .map(|n| n.get())
                 .unwrap_or(1));
    println!();
    
    // Test 1: Default configuration (single-threaded)
    let default_config = OslomConfig::default();
    run_test(&default_config, "Default (single-threaded)");
    
    // Test 2: Explicitly enable multi-threading with all cores
    let multi_config = OslomConfig {
        num_threads: None, // Use all available threads
        ..Default::default()
    };
    run_test(&multi_config, "Multi-threaded (all cores)");
    
    // Test 3: Multi-threading with specific thread count
    let custom_config = OslomConfig {
        num_threads: Some(2), // Use 2 threads
        ..Default::default()
    };
    run_test(&custom_config, "Multi-threaded (2 threads)");
    
    // Test 4: Multiple runs to show parallel benefit
    let multi_runs_config = OslomConfig {
        r: 5,  // 5 runs
        num_threads: None, // Enable parallel processing
        ..Default::default()
    };
    run_test(&multi_runs_config, "Multi-threaded with 5 runs");
    
    println!("Summary:");
    println!("- Default configuration uses single-threaded execution");
    println!("- Set num_threads: None to enable all available cores");
    println!("- Set num_threads: Some(n) to use n specific threads");
    println!("- Parallel processing is most beneficial with multiple runs");
}