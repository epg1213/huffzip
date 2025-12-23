use std::collections::HashMap;
use crate::tree::{node_tree::Tree, node::*};
use serde::{Serialize, Deserialize};
use crate::map::nodemap::NodeMap;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DecompressionMap {
    pub data: HashMap<Vec<bool>, u8>
}
impl From<&DecompressionMap> for NodeMap {
    fn from(value: &DecompressionMap) -> Self {
        let mut new_map = NodeMap::new();
        for (k, v) in value.data.iter() {
            new_map.data.insert(k.clone(), Node::LeafNode(Leaf { byte: v.clone(), byte_count: 0 }));
        }
        new_map
    }
}

impl DecompressionMap {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl From<&DecompressionMap> for Tree {
    fn from(value: &DecompressionMap) -> Self {
        let mut nmap = NodeMap::from(value);
        while let Some(new_map) = nmap.reduce_depth() {
            nmap=new_map;
        }
        let mut res = Tree::new();
        match nmap.data.iter().next() {
            Some((_, v)) => { res.set_root(v) },
            None => {}
        }
        res
    }
}

