use crate::error::{OslomError, Result};
use crate::graph::{Network, NodeId};
use crate::modules::{ModuleCollection, ModuleId};
use crate::louvain::LouvainOptimizer;
use crate::statistics::StatisticalTester;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OslomConfig {
    /// Number of runs for the first hierarchical level
    pub r: usize,
    /// Number of runs for higher hierarchical levels  
    pub hr: usize,
    /// Statistical significance threshold
    pub threshold: f64,
    /// Coverage parameter for module unions
    pub cp: f64,
    /// Whether to find singleton nodes
    pub find_singletons: bool,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
    /// Verbose output
    pub verbose: bool,
    /// Maximum iterations for convergence
    pub max_iterations: usize,
    /// Convergence tolerance
    pub convergence_tolerance: f64,
    /// Number of threads for parallel processing (Some(1) = single-threaded by default, None = use all available)
    pub num_threads: Option<usize>,
}

impl Default for OslomConfig {
    fn default() -> Self {
        Self {
            r: 10,
            hr: 50,
            threshold: 0.1,
            cp: 0.5,
            find_singletons: false,
            random_seed: None,
            verbose: false,
            max_iterations: 100,
            convergence_tolerance: 1e-6,
            num_threads: Some(1), // Single-threaded by default
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClusteringStats {
    pub num_modules: usize,
    pub coverage: usize,
    pub modularity: f64,
    pub total_nodes: usize,
    pub homeless_nodes: usize,
}

#[derive(Debug, Clone)]
pub struct OslomResult {
    /// Hierarchical clustering results (level -> modules)
    pub hierarchical_modules: HashMap<usize, ModuleCollection>,
    /// Clustering statistics
    pub statistics: ClusteringStats,
    /// Configuration used
    pub config: OslomConfig,
}

impl OslomResult {
    pub fn num_levels(&self) -> usize {
        self.hierarchical_modules.len()
    }

    pub fn max_level(&self) -> usize {
        self.hierarchical_modules.keys().copied().max().unwrap_or(0)
    }

    pub fn get_level(&self, level: usize) -> Option<&ModuleCollection> {
        self.hierarchical_modules.get(&level)
    }

    /// Convert to Python-compatible format
    pub fn to_python_format(&self) -> HashMap<usize, HashMap<usize, Vec<NodeId>>> {
        let mut result = HashMap::new();
        
        for (&level, collection) in &self.hierarchical_modules {
            let mut level_modules = HashMap::new();
            for (idx, module) in collection.modules().enumerate() {
                level_modules.insert(idx, module.nodes.clone());
            }
            result.insert(level, level_modules);
        }
        
        result
    }
}

/// Main OSLOM algorithm implementation
pub fn run_oslom(network: &Network, config: &OslomConfig) -> Result<OslomResult> {
    if network.node_count() == 0 {
        return Err(OslomError::InsufficientData("Empty network".to_string()));
    }

    // Configure thread pool if specified (and not single-threaded)
    if let Some(num_threads) = config.num_threads {
        if num_threads > 1 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .map_err(|e| OslomError::InvalidConfig(format!("Failed to set thread count: {}", e)))?;
        }
    } else {
        // None means use all available threads
        // No need to configure - rayon will use default thread pool
    }

    let mut hierarchical_modules = HashMap::new();
    let mut current_network = network.clone();
    let mut level = 0;

    // Initialize random seed if provided
    if let Some(seed) = config.random_seed {
        use rand::SeedableRng;
        let _rng = rand::rngs::StdRng::seed_from_u64(seed);
    }

    loop {
        if config.verbose {
            println!("Processing hierarchical level {} with {} threads", 
                     level, rayon::current_num_threads());
        }

        // Run OSLOM for current level
        let modules = run_oslom_level(&current_network, config, level)?;
        
        if modules.size() == 0 {
            if config.verbose {
                println!("No modules found at level {}, stopping", level);
            }
            break;
        }

        if config.verbose {
            println!("Found {} modules at level {}", modules.size(), level);
        }

        hierarchical_modules.insert(level, modules.clone());

        // Check if we should continue to next level
        if modules.size() <= 1 || level > 10 {
            break;
        }

        // Create network for next level (aggregate current modules)
        current_network = create_aggregate_network(&current_network, &modules)?;
        level += 1;
    }

    // Calculate final statistics
    let base_level = hierarchical_modules.get(&0);
    let statistics = if let Some(base_modules) = base_level {
        ClusteringStats {
            num_modules: base_modules.size(),
            coverage: base_modules.coverage(),
            modularity: calculate_modularity(network, base_modules),
            total_nodes: network.node_count(),
            homeless_nodes: base_modules.homeless_nodes(network.node_count()).len(),
        }
    } else {
        ClusteringStats {
            num_modules: 0,
            coverage: 0,
            modularity: 0.0,
            total_nodes: network.node_count(),
            homeless_nodes: network.node_count(),
        }
    };

    Ok(OslomResult {
        hierarchical_modules,
        statistics,
        config: config.clone(),
    })
}

fn run_oslom_level(network: &Network, config: &OslomConfig, level: usize) -> Result<ModuleCollection> {
    let runs = if level == 0 { config.r } else { config.hr };
    
    // Check if we should use parallel processing
    let use_parallel = match config.num_threads {
        Some(1) => false,  // Explicitly single-threaded
        Some(0) => false,  // Invalid thread count, treat as single-threaded
        Some(_) => runs > 1,  // Multi-threaded if multiple runs
        None => runs > 1,  // Use all threads if multiple runs
    };
    
    if runs == 1 || !use_parallel {
        // Single run or single-threaded - no need for parallelization
        return run_single_oslom_iteration(network, config);
    }

    // Parallel execution of multiple runs
    let results: Result<Vec<_>> = (0..runs)
        .into_par_iter()
        .map(|run| {
            if config.verbose && runs > 1 {
                println!("  Run {}/{}", run + 1, runs);
            }
            run_single_oslom_iteration(network, config)
        })
        .collect();

    let all_results = results?;
    
    // Find the best result based on modularity
    let mut best_modules = ModuleCollection::new();
    let mut best_modularity = f64::NEG_INFINITY;

    for modules in all_results {
        let modularity = calculate_modularity(network, &modules);
        if modularity > best_modularity {
            best_modularity = modularity;
            best_modules = modules;
        }
    }

    // Handle singletons if requested
    if config.find_singletons {
        handle_singletons(network, &mut best_modules)?;
    }

    Ok(best_modules)
}

fn run_single_oslom_iteration(network: &Network, config: &OslomConfig) -> Result<ModuleCollection> {
    // Initialize with Louvain algorithm
    let mut optimizer = LouvainOptimizer::new(network);
    let initial_modules = optimizer.optimize()?;

    // Evaluate modules statistically
    let mut tester = StatisticalTester::new(network, config.threshold);
    let evaluated_modules = tester.evaluate_modules(&initial_modules)?;

    // Check for overlaps and merge if necessary
    let final_modules = process_overlaps(network, evaluated_modules, config)?;

    Ok(final_modules)
}

fn process_overlaps(
    network: &Network,
    mut modules: ModuleCollection,
    config: &OslomConfig,
) -> Result<ModuleCollection> {
    let overlap_threshold = 0.5; // Could be made configurable
    
    loop {
        let overlaps = modules.find_overlapping_pairs(overlap_threshold);
        if overlaps.is_empty() {
            break;
        }

        // Process the most significant overlap
        if let Some((id1, id2, _overlap)) = overlaps.first() {
            // Decide whether to merge based on statistical significance
            let should_merge = should_merge_modules(network, &modules, *id1, *id2, config)?;
            
            if should_merge {
                modules.merge_modules(&[*id1, *id2])?;
            } else {
                // Remove the weaker module
                let module1 = modules.get_module(*id1).unwrap();
                let module2 = modules.get_module(*id2).unwrap();
                
                if module1.score < module2.score {
                    modules.remove_module(*id1);
                } else {
                    modules.remove_module(*id2);
                }
            }
        }
    }

    Ok(modules)
}

fn should_merge_modules(
    network: &Network,
    modules: &ModuleCollection,
    id1: ModuleId,
    id2: ModuleId,
    config: &OslomConfig,
) -> Result<bool> {
    let module1 = modules.get_module(id1).ok_or_else(|| {
        OslomError::InvalidConfig("Module not found".to_string())
    })?;
    let module2 = modules.get_module(id2).ok_or_else(|| {
        OslomError::InvalidConfig("Module not found".to_string())
    })?;

    // Create union of modules
    let mut union_nodes = module1.nodes.clone();
    union_nodes.extend(&module2.nodes);
    union_nodes.sort_unstable();
    union_nodes.dedup();

    // Test statistical significance of union
    let mut tester = StatisticalTester::new(network, config.threshold);
    let union_score = tester.evaluate_module(&union_nodes)?;

    // Merge if union is better than individual modules
    Ok(union_score > module1.score.max(module2.score))
}

fn handle_singletons(network: &Network, modules: &mut ModuleCollection) -> Result<()> {
    let homeless = modules.homeless_nodes(network.node_count());
    
    for node in homeless {
        // Try to assign to neighboring modules
        if let Some(neighbors) = network.neighbors(node) {
            let mut best_module = None;
            let mut best_connections = 0;
            
            for &(neighbor, _weight) in neighbors {
                if let Some(memberships) = modules.node_memberships(neighbor) {
                    for &module_id in memberships {
                        // Count connections to this module
                        let module = modules.get_module(module_id).unwrap();
                        let connections = neighbors
                            .iter()
                            .filter(|(n, _)| module.contains(*n))
                            .count();
                        
                        if connections > best_connections {
                            best_connections = connections;
                            best_module = Some(module_id);
                        }
                    }
                }
            }
            
            // If no good module found, create singleton
            if best_module.is_none() {
                modules.insert_module(vec![node], 0.0);
            }
        } else {
            // Isolated node - create singleton
            modules.insert_module(vec![node], 0.0);
        }
    }
    
    Ok(())
}

fn create_aggregate_network(
    original: &Network,
    modules: &ModuleCollection,
) -> Result<Network> {
    // Create a new network where each module becomes a node
    let mut builder = crate::graph::NetworkBuilder::new(original.is_directed());
    
    // Map module IDs to new node IDs
    let module_ids: Vec<_> = modules.module_ids().collect();
    let id_map: HashMap<ModuleId, NodeId> = module_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();
    
    // Calculate inter-module edge weights
    for i in 0..module_ids.len() {
        for j in (i + 1)..module_ids.len() {
            let module1 = modules.get_module(module_ids[i]).unwrap();
            let module2 = modules.get_module(module_ids[j]).unwrap();
            
            let weight = calculate_inter_module_weight(original, module1, module2);
            if weight > 0.0 {
                let node1 = id_map[&module_ids[i]];
                let node2 = id_map[&module_ids[j]];
                builder.add_edge(node1, node2, weight);
            }
        }
    }
    
    Ok(builder.build())
}

fn calculate_inter_module_weight(
    network: &Network,
    module1: &crate::modules::Module,
    module2: &crate::modules::Module,
) -> f64 {
    let mut total_weight = 0.0;
    
    for &node1 in &module1.nodes {
        if let Some(neighbors) = network.neighbors(node1) {
            for &(node2, weight) in neighbors {
                if module2.contains(node2) {
                    total_weight += weight;
                }
            }
        }
    }
    
    total_weight
}

fn calculate_modularity(network: &Network, modules: &ModuleCollection) -> f64 {
    let total_weight = network.total_weight();
    if total_weight == 0.0 {
        return 0.0;
    }
    
    // Use sequential calculation by default
    // TODO: Add config parameter to enable parallel calculation
    let module_vec: Vec<_> = modules.modules().collect();
    calculate_modularity_sequential(network, &module_vec, total_weight)
}

fn calculate_modularity_sequential(network: &Network, modules: &[&crate::modules::Module], total_weight: f64) -> f64 {
    let mut modularity = 0.0;
    
    for module in modules {
        let mut internal_weight = 0.0;
        let mut total_degree = 0.0;
        
        // Calculate internal edges and total degree
        for &node in &module.nodes {
            total_degree += network.weighted_degree(node);
            
            if let Some(neighbors) = network.neighbors(node) {
                for &(neighbor, weight) in neighbors {
                    if module.contains(neighbor) {
                        internal_weight += weight;
                    }
                }
            }
        }
        
        // Avoid double counting for undirected graphs
        if !network.is_directed() {
            internal_weight /= 2.0;
        }
        
        let expected = (total_degree * total_degree) / (4.0 * total_weight);
        modularity += (internal_weight / total_weight) - expected / total_weight;
    }
    
    modularity
}

fn calculate_modularity_parallel(network: &Network, modules: &[&crate::modules::Module], total_weight: f64) -> f64 {
    modules
        .par_iter()
        .map(|module| {
            let mut internal_weight = 0.0;
            let mut total_degree = 0.0;
            
            // Calculate internal edges and total degree
            for &node in &module.nodes {
                total_degree += network.weighted_degree(node);
                
                if let Some(neighbors) = network.neighbors(node) {
                    for &(neighbor, weight) in neighbors {
                        if module.contains(neighbor) {
                            internal_weight += weight;
                        }
                    }
                }
            }
            
            // Avoid double counting for undirected graphs
            if !network.is_directed() {
                internal_weight /= 2.0;
            }
            
            let expected = (total_degree * total_degree) / (4.0 * total_weight);
            (internal_weight / total_weight) - expected / total_weight
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NetworkBuilder;

    #[test]
    fn test_basic_oslom() {
        let mut builder = NetworkBuilder::new(false);
        // Create a simple network with two communities
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 1.0)
               .add_edge(2, 0, 1.0)
               .add_edge(3, 4, 1.0)
               .add_edge(4, 5, 1.0)
               .add_edge(5, 3, 1.0)
               .add_edge(2, 3, 0.1); // Weak connection between communities
        
        let network = builder.build();
        let config = OslomConfig::default();
        
        let result = run_oslom(&network, &config).unwrap();
        
        assert!(result.num_levels() > 0);
        assert!(result.statistics.num_modules > 0);
        assert!(result.statistics.coverage > 0);
    }

    #[test]
    fn test_empty_network() {
        let network = NetworkBuilder::new(false).build();
        let config = OslomConfig::default();
        
        let result = run_oslom(&network, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_single_node() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 0, 1.0); // Self-loop
        
        let network = builder.build();
        let config = OslomConfig::default();
        
        let result = run_oslom(&network, &config).unwrap();
        assert_eq!(result.statistics.total_nodes, 1);
    }
}