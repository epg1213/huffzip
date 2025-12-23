use std::collections::HashMap;
use crate::tree::node::*;
use crate::map::decompress::DecompressionMap;

#[derive(Debug)]
pub struct CompressionMap {
    pub data: HashMap<u8, Vec<bool>>
}

impl CompressionMap {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn map(&mut self, node: &Node, path: Vec<bool>) {
        match node {
            Node::LeafNode(leaf) => {
                self.data.insert(leaf.byte, path);
            },
            Node::BranchNode(a, b) => {
                let mut path_a = path.clone();
                path_a.push(false);
                self.map(a, path_a);
                let mut path_b = path.clone();
                path_b.push(true);
                self.map(b, path_b);
            }
        }
    }

    pub fn get(&self, key: &u8) -> Option<Vec<bool>> {
        if let Some(value) = self.data.get(key) {
            return Some(value.clone());
        }
        None
    }
}

impl From<&CompressionMap> for DecompressionMap {
    fn from(value: &CompressionMap) -> DecompressionMap {
        let rev = value.data.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
        DecompressionMap { data: rev }
    }
}

