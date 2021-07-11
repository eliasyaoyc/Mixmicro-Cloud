use crate::node::Node;
use dashmap::DashMap;
use std::fmt::{Debug, Formatter};

pub struct Cluster {
    nodes: DashMap<&'static str, Node>,
}

impl Debug for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("cluster")
            .field("nodes", &self.nodes)
            .finish()
    }
}

impl Cluster {
    pub fn new() -> Self {
        Self { nodes: Default::default() }
    }
}