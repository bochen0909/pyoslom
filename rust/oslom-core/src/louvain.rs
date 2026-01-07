use crate::error::{OslomError, Result};
use crate::graph::{Network, NodeId};
use crate::modules::{ModuleCollection, ModuleId};
use rand::prelude::*;
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct LouvainOptimizer<'a> {
    network: &'a Network,
    node_to_community: HashMap<NodeId, ModuleId>,
    community_weights: HashMap<ModuleId, f64>,
    total_weight: f64,
}

impl<'a> LouvainOptimizer<'a> {
    pub fn new(network: &'a Network) -> Self {
        let mut optimizer = Self {
            network,
            node_to_community: HashMap::new(),
            community_weights: HashMap::new(),
            total_weight: network.total_weight(),
        };
        
        // Initialize each node in its own community
        for (i, node) in network.nodes().enumerate() {
            optimizer.node_to_community.insert(node, i);
            optimizer.community_weights.insert(i, network.weighted_degree(node));
        }
        
        optimizer
    }

    pub fn optimize(&mut self) -> Result<ModuleCollection> {
        let mut improved = true;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 100;

        while improved && iteration < MAX_ITERATIONS {
            improved = self.optimize_pass()?;
            iteration += 1;
        }

        if iteration >= MAX_ITERATIONS {
            return Err(OslomError::ConvergenceFailed(
                "Louvain algorithm did not converge".to_string()
            ));
        }

        self.build_module_collection()
    }

    fn optimize_pass(&mut self) -> Result<bool> {
        let mut nodes: Vec<_> = self.network.nodes().collect();
        
        // Randomize order for better results
        nodes.shuffle(&mut thread_rng());

        // For now, always use sequential processing to respect single-threaded default
        // TODO: Add config parameter to control parallel processing
        self.optimize_pass_sequential(&nodes)
    }

    fn optimize_pass_sequential(&mut self, nodes: &[NodeId]) -> Result<bool> {
        let mut improved = false;

        for &node in nodes {
            let current_community = self.node_to_community[&node];
            let best_community = self.find_best_community(node)?;

            if best_community != current_community {
                self.move_node_to_community(node, best_community)?;
                improved = true;
            }
        }

        Ok(improved)
    }

    fn optimize_pass_parallel(&mut self, nodes: &[NodeId]) -> Result<bool> {
        let chunk_size = (nodes.len() / rayon::current_num_threads()).max(100);
        let node_chunks: Vec<_> = nodes.chunks(chunk_size).collect();
        
        // Shared state for parallel processing
        let node_to_community = Arc::new(Mutex::new(self.node_to_community.clone()));
        let community_weights = Arc::new(Mutex::new(self.community_weights.clone()));
        
        // Process chunks in parallel
        let moves: Vec<_> = node_chunks
            .par_iter()
            .map(|chunk| {
                let mut local_moves = Vec::new();
                
                for &node in chunk.iter() {
                    let current_community = {
                        let communities = node_to_community.lock().unwrap();
                        communities[&node]
                    };
                    
                    let best_community = self.find_best_community_parallel(
                        node, 
                        &node_to_community, 
                        &community_weights
                    )?;

                    if best_community != current_community {
                        local_moves.push((node, current_community, best_community));
                    }
                }
                
                Ok::<Vec<_>, OslomError>(local_moves)
            })
            .collect::<Result<Vec<_>>>()?;

        // Apply moves sequentially to avoid conflicts
        let mut improved = false;
        for chunk_moves in moves {
            for (node, _old_community, new_community) in chunk_moves {
                self.move_node_to_community(node, new_community)?;
                improved = true;
            }
        }

        Ok(improved)
    }

    fn find_best_community_parallel(
        &self,
        node: NodeId,
        node_to_community: &Arc<Mutex<HashMap<NodeId, ModuleId>>>,
        community_weights: &Arc<Mutex<HashMap<ModuleId, f64>>>,
    ) -> Result<ModuleId> {
        let current_community = {
            let communities = node_to_community.lock().unwrap();
            communities[&node]
        };
        
        let mut best_community = current_community;
        let mut best_gain = 0.0;

        // Consider neighboring communities
        let mut candidate_communities = std::collections::HashSet::new();
        candidate_communities.insert(current_community);

        if let Some(neighbors) = self.network.neighbors(node) {
            for &(neighbor, _) in neighbors {
                let neighbor_community = {
                    let communities = node_to_community.lock().unwrap();
                    communities.get(&neighbor).copied()
                };
                
                if let Some(community) = neighbor_community {
                    candidate_communities.insert(community);
                }
            }
        }

        for &community in &candidate_communities {
            let gain = self.calculate_modularity_gain_parallel(
                node, 
                community, 
                node_to_community, 
                community_weights
            )?;
            
            if gain > best_gain {
                best_gain = gain;
                best_community = community;
            }
        }

        Ok(best_community)
    }

    fn calculate_modularity_gain_parallel(
        &self,
        node: NodeId,
        target_community: ModuleId,
        node_to_community: &Arc<Mutex<HashMap<NodeId, ModuleId>>>,
        community_weights: &Arc<Mutex<HashMap<ModuleId, f64>>>,
    ) -> Result<f64> {
        let current_community = {
            let communities = node_to_community.lock().unwrap();
            communities[&node]
        };
        
        if current_community == target_community {
            return Ok(0.0);
        }

        let node_degree = self.network.weighted_degree(node);
        
        let (current_community_weight, target_community_weight) = {
            let weights = community_weights.lock().unwrap();
            let current_weight = weights[&current_community];
            let target_weight = weights.get(&target_community).copied().unwrap_or(0.0);
            (current_weight, target_weight)
        };

        // Calculate edges to current and target communities
        let edges_to_current = self.calculate_edges_to_community_parallel(
            node, 
            current_community, 
            node_to_community
        )?;
        let edges_to_target = self.calculate_edges_to_community_parallel(
            node, 
            target_community, 
            node_to_community
        )?;

        if self.total_weight == 0.0 {
            return Ok(0.0);
        }

        // Modularity gain calculation
        let gain = (edges_to_target - edges_to_current) / self.total_weight
            - (node_degree * (target_community_weight - current_community_weight + node_degree)) 
              / (2.0 * self.total_weight * self.total_weight);

        Ok(gain)
    }

    fn calculate_edges_to_community_parallel(
        &self,
        node: NodeId,
        community: ModuleId,
        node_to_community: &Arc<Mutex<HashMap<NodeId, ModuleId>>>,
    ) -> Result<f64> {
        let mut weight = 0.0;

        if let Some(neighbors) = self.network.neighbors(node) {
            let communities = node_to_community.lock().unwrap();
            
            for &(neighbor, edge_weight) in neighbors {
                if let Some(&neighbor_community) = communities.get(&neighbor) {
                    if neighbor_community == community && neighbor != node {
                        weight += edge_weight;
                    }
                }
            }
        }

        Ok(weight)
    }

    fn find_best_community(&self, node: NodeId) -> Result<ModuleId> {
        let current_community = self.node_to_community[&node];
        let mut best_community = current_community;
        let mut best_gain = 0.0;

        // Consider neighboring communities
        let mut candidate_communities = std::collections::HashSet::new();
        candidate_communities.insert(current_community);

        if let Some(neighbors) = self.network.neighbors(node) {
            for &(neighbor, _) in neighbors {
                if let Some(&community) = self.node_to_community.get(&neighbor) {
                    candidate_communities.insert(community);
                }
            }
        }

        for &community in &candidate_communities {
            let gain = self.calculate_modularity_gain(node, community)?;
            if gain > best_gain {
                best_gain = gain;
                best_community = community;
            }
        }

        Ok(best_community)
    }

    fn calculate_modularity_gain(&self, node: NodeId, target_community: ModuleId) -> Result<f64> {
        let current_community = self.node_to_community[&node];
        
        if current_community == target_community {
            return Ok(0.0);
        }

        let node_degree = self.network.weighted_degree(node);
        let current_community_weight = self.community_weights[&current_community];
        let target_community_weight = self.community_weights.get(&target_community).copied().unwrap_or(0.0);

        // Calculate edges to current and target communities
        let edges_to_current = self.calculate_edges_to_community(node, current_community)?;
        let edges_to_target = self.calculate_edges_to_community(node, target_community)?;

        if self.total_weight == 0.0 {
            return Ok(0.0);
        }

        // Modularity gain calculation
        let gain = (edges_to_target - edges_to_current) / self.total_weight
            - (node_degree * (target_community_weight - current_community_weight + node_degree)) 
              / (2.0 * self.total_weight * self.total_weight);

        Ok(gain)
    }

    fn calculate_edges_to_community(&self, node: NodeId, community: ModuleId) -> Result<f64> {
        let mut weight = 0.0;

        if let Some(neighbors) = self.network.neighbors(node) {
            for &(neighbor, edge_weight) in neighbors {
                if let Some(&neighbor_community) = self.node_to_community.get(&neighbor) {
                    if neighbor_community == community && neighbor != node {
                        weight += edge_weight;
                    }
                }
            }
        }

        Ok(weight)
    }

    fn move_node_to_community(&mut self, node: NodeId, new_community: ModuleId) -> Result<()> {
        let old_community = self.node_to_community[&node];
        let node_degree = self.network.weighted_degree(node);

        // Update community weights
        if let Some(weight) = self.community_weights.get_mut(&old_community) {
            *weight -= node_degree;
        }
        
        *self.community_weights.entry(new_community).or_insert(0.0) += node_degree;

        // Update node assignment
        self.node_to_community.insert(node, new_community);

        Ok(())
    }

    fn build_module_collection(&self) -> Result<ModuleCollection> {
        let mut collection = ModuleCollection::new();
        let mut community_nodes: HashMap<ModuleId, Vec<NodeId>> = HashMap::new();

        // Group nodes by community
        for (&node, &community) in &self.node_to_community {
            community_nodes.entry(community).or_default().push(node);
        }

        // Create modules for non-empty communities
        for (community_id, mut nodes) in community_nodes {
            if !nodes.is_empty() {
                nodes.sort_unstable();
                let modularity = self.calculate_community_modularity(community_id, &nodes)?;
                collection.insert_module(nodes, modularity);
            }
        }

        Ok(collection)
    }

    fn calculate_community_modularity(&self, community_id: ModuleId, nodes: &[NodeId]) -> Result<f64> {
        if self.total_weight == 0.0 {
            return Ok(0.0);
        }

        let mut internal_weight = 0.0;
        let mut total_degree = 0.0;

        for &node in nodes {
            total_degree += self.network.weighted_degree(node);
            
            if let Some(neighbors) = self.network.neighbors(node) {
                for &(neighbor, weight) in neighbors {
                    if let Some(&neighbor_community) = self.node_to_community.get(&neighbor) {
                        if neighbor_community == community_id {
                            internal_weight += weight;
                        }
                    }
                }
            }
        }

        // Avoid double counting for undirected graphs
        if !self.network.is_directed() {
            internal_weight /= 2.0;
        }

        let expected = (total_degree * total_degree) / (4.0 * self.total_weight);
        let modularity = (internal_weight / self.total_weight) - (expected / self.total_weight);

        Ok(modularity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NetworkBuilder;

    #[test]
    fn test_louvain_simple() {
        let mut builder = NetworkBuilder::new(false);
        // Create two triangles connected by a weak edge
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 1.0)
               .add_edge(2, 0, 1.0)
               .add_edge(3, 4, 1.0)
               .add_edge(4, 5, 1.0)
               .add_edge(5, 3, 1.0)
               .add_edge(2, 3, 0.1);

        let network = builder.build();
        let mut optimizer = LouvainOptimizer::new(&network);
        let result = optimizer.optimize().unwrap();

        // Should find at least 2 communities
        assert!(result.size() >= 2);
        assert!(result.coverage() == 6); // All nodes should be assigned
    }

    #[test]
    fn test_louvain_single_node() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 0, 1.0);

        let network = builder.build();
        let mut optimizer = LouvainOptimizer::new(&network);
        let result = optimizer.optimize().unwrap();

        assert_eq!(result.size(), 1);
        assert_eq!(result.coverage(), 1);
    }

    #[test]
    fn test_louvain_disconnected() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 1, 1.0)
               .add_edge(2, 3, 1.0); // Two disconnected edges

        let network = builder.build();
        let mut optimizer = LouvainOptimizer::new(&network);
        let result = optimizer.optimize().unwrap();

        // Should find separate communities for disconnected components
        assert!(result.size() >= 2);
        assert_eq!(result.coverage(), 4);
    }
}