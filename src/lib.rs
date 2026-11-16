//! SigmaOS-aks: Azure Kubernetes Service model.
//!
//! Real implementation: clusters, nodes, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cluster {
    pub id: String,
    pub name: String,
    pub region: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub cluster_id: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AksError {
    EmptyClusterName,
    DuplicateCluster(String),
}

impl fmt::Display for AksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyClusterName => write!(f, "cluster name must not be empty"),
            Self::DuplicateCluster(id) => write!(f, "duplicate cluster {:?}", id),
        }
    }
}

impl std::error::Error for AksError {}

pub struct Aks {
    pub name: String,
    clusters: BTreeMap<String, Cluster>,
    nodes: BTreeMap<String, Node>,
}

impl Aks {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), clusters: BTreeMap::new(), nodes: BTreeMap::new() }
    }

    pub fn add_cluster(&mut self, c: Cluster) -> Result<(), AksError> {
        if c.name.trim().is_empty() { return Err(AksError::EmptyClusterName); }
        let id = c.id.clone();
        if self.clusters.contains_key(&id) {
            return Err(AksError::DuplicateCluster(id.clone()));
        }
        self.clusters.insert(id, c);
        Ok(())
    }

    pub fn cluster_count(&self) -> usize { self.clusters.len() }

    pub fn get_cluster(&self, id: &str) -> Option<&Cluster> { self.clusters.get(id) }

    pub fn add_node(&mut self, n: Node) {
        self.nodes.insert(n.id.clone(), n);
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> { self.nodes.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Aks::new("Aks");
        assert_eq!(a.name, "Aks");
        assert_eq!(a.cluster_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Aks::new("Aks");
        let cluster = Cluster { id: "c1".into(), name: "prod".into(), region: "eastus".into() };
        a.add_cluster(cluster).unwrap();
        assert_eq!(a.cluster_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Aks::new("Aks");
        let cluster = Cluster { id: "x".into(), name: "".into(), region: "x".into() };
        assert!(a.add_cluster(cluster).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Aks::new("Aks");
        let cluster = Cluster { id: "x".into(), name: "X".into(), region: "x".into() };
        a.add_cluster(cluster).unwrap();
        let cluster2 = Cluster { id: "x".into(), name: "X".into(), region: "x".into() };
        assert!(a.add_cluster(cluster2).is_err());
    }
}
