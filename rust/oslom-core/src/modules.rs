use std::collections::{HashMap, HashSet};
use crate::graph::NodeId;
use crate::error::{OslomError, Result};

pub type ModuleId = usize;

#[derive(Debug, Clone)]
pub struct Module {
    pub id: ModuleId,
    pub nodes: Vec<NodeId>,
    pub score: f64,
}

impl Module {
    pub fn new(id: ModuleId, nodes: Vec<NodeId>, score: f64) -> Self {
        Self { id, nodes, score }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    pub fn intersection_size(&self, other: &Module) -> usize {
        let self_set: HashSet<_> = self.nodes.iter().copied().collect();
        other.nodes.iter().filter(|&&n| self_set.contains(&n)).count()
    }

    pub fn overlap_ratio(&self, other: &Module) -> f64 {
        let intersection = self.intersection_size(other);
        let min_size = self.size().min(other.size());
        if min_size == 0 {
            0.0
        } else {
            intersection as f64 / min_size as f64
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleCollection {
    modules: HashMap<ModuleId, Module>,
    memberships: HashMap<NodeId, HashSet<ModuleId>>,
    next_id: ModuleId,
}

impl ModuleCollection {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            memberships: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert_module(&mut self, nodes: Vec<NodeId>, score: f64) -> ModuleId {
        let id = self.next_id;
        self.next_id += 1;

        let module = Module::new(id, nodes.clone(), score);
        
        // Update memberships
        for &node in &nodes {
            self.memberships.entry(node).or_default().insert(id);
        }
        
        self.modules.insert(id, module);
        id
    }

    pub fn remove_module(&mut self, id: ModuleId) -> Option<Module> {
        if let Some(module) = self.modules.remove(&id) {
            // Update memberships
            for &node in &module.nodes {
                if let Some(membership) = self.memberships.get_mut(&node) {
                    membership.remove(&id);
                    if membership.is_empty() {
                        self.memberships.remove(&node);
                    }
                }
            }
            Some(module)
        } else {
            None
        }
    }

    pub fn get_module(&self, id: ModuleId) -> Option<&Module> {
        self.modules.get(&id)
    }

    pub fn modules(&self) -> impl Iterator<Item = &Module> {
        self.modules.values()
    }

    pub fn module_ids(&self) -> impl Iterator<Item = ModuleId> + '_ {
        self.modules.keys().copied()
    }

    pub fn size(&self) -> usize {
        self.modules.len()
    }

    pub fn node_memberships(&self, node: NodeId) -> Option<&HashSet<ModuleId>> {
        self.memberships.get(&node)
    }

    pub fn check_overlap(&self, m1: ModuleId, m2: ModuleId) -> Option<f64> {
        let module1 = self.modules.get(&m1)?;
        let module2 = self.modules.get(&m2)?;
        Some(module1.overlap_ratio(module2))
    }

    pub fn find_overlapping_pairs(&self, threshold: f64) -> Vec<(ModuleId, ModuleId, f64)> {
        let mut overlaps = Vec::new();
        let module_ids: Vec<_> = self.module_ids().collect();
        
        for i in 0..module_ids.len() {
            for j in (i + 1)..module_ids.len() {
                let id1 = module_ids[i];
                let id2 = module_ids[j];
                
                if let Some(overlap) = self.check_overlap(id1, id2) {
                    if overlap >= threshold {
                        overlaps.push((id1, id2, overlap));
                    }
                }
            }
        }
        
        overlaps
    }

    pub fn merge_modules(&mut self, module_ids: &[ModuleId]) -> Result<ModuleId> {
        if module_ids.is_empty() {
            return Err(OslomError::InvalidConfig("Cannot merge empty module list".to_string()));
        }

        let mut all_nodes = HashSet::new();
        let mut total_score = 0.0;
        let mut valid_modules = Vec::new();

        // Collect nodes and scores from all modules
        for &id in module_ids {
            if let Some(module) = self.modules.get(&id) {
                all_nodes.extend(module.nodes.iter().copied());
                total_score += module.score;
                valid_modules.push(id);
            }
        }

        if valid_modules.is_empty() {
            return Err(OslomError::InvalidConfig("No valid modules to merge".to_string()));
        }

        // Remove old modules
        for &id in &valid_modules {
            self.remove_module(id);
        }

        // Create new merged module
        let nodes: Vec<_> = all_nodes.into_iter().collect();
        let avg_score = total_score / valid_modules.len() as f64;
        let new_id = self.insert_module(nodes, avg_score);

        Ok(new_id)
    }

    pub fn coverage(&self) -> usize {
        self.memberships.len()
    }

    pub fn effective_groups(&self) -> usize {
        self.modules.values().filter(|m| !m.nodes.is_empty()).count()
    }

    pub fn homeless_nodes(&self, total_nodes: usize) -> Vec<NodeId> {
        let mut homeless = Vec::new();
        for node in 0..total_nodes {
            if !self.memberships.contains_key(&node) {
                homeless.push(node);
            }
        }
        homeless
    }
}

impl Default for ModuleCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_operations() {
        let mut collection = ModuleCollection::new();
        
        let id1 = collection.insert_module(vec![0, 1, 2], 0.8);
        let id2 = collection.insert_module(vec![2, 3, 4], 0.7);
        
        assert_eq!(collection.size(), 2);
        assert_eq!(collection.coverage(), 5);
        
        // Test overlap
        let overlap = collection.check_overlap(id1, id2).unwrap();
        assert!(overlap > 0.0); // Should have overlap due to node 2
        
        // Test merge
        let merged_id = collection.merge_modules(&[id1, id2]).unwrap();
        assert_eq!(collection.size(), 1);
        
        let merged_module = collection.get_module(merged_id).unwrap();
        assert_eq!(merged_module.size(), 5); // All unique nodes
    }

    #[test]
    fn test_overlapping_pairs() {
        let mut collection = ModuleCollection::new();
        
        collection.insert_module(vec![0, 1, 2], 0.8);
        collection.insert_module(vec![2, 3, 4], 0.7);
        collection.insert_module(vec![5, 6, 7], 0.6);
        
        let overlaps = collection.find_overlapping_pairs(0.1);
        assert_eq!(overlaps.len(), 1); // Only first two modules overlap
    }

    #[test]
    fn test_homeless_nodes() {
        let mut collection = ModuleCollection::new();
        collection.insert_module(vec![0, 2, 4], 0.8);
        
        let homeless = collection.homeless_nodes(6);
        assert_eq!(homeless, vec![1, 3, 5]);
    }
}