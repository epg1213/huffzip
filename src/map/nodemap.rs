use std::collections::HashMap;
use crate::tree::node::*;

pub struct NodeMap {
    pub data: HashMap<Vec<bool>, Node>
}

impl NodeMap {

    pub fn new() -> Self {
        Self {data: HashMap::new()}
    }

    pub fn reduce_depth(&self) -> Option<NodeMap> {
        if self.data.len() < 2 {
            return None;
        }
        let mut new_map = NodeMap::new();
        for (path, node) in self.data.clone().iter() {
            let mut new_path = path.clone();
            if let Some(npath) = new_path.pop() {
                let mut other_path = new_path.clone();
                other_path.push(!npath);
                match self.data.get(&other_path) {
                    Some(other_node) => {
                        if npath {
                            new_map.data.insert(new_path, Node::BranchNode(Box::new(other_node.clone()), Box::new(node.clone())));
                        } else {
                            new_map.data.insert(new_path, Node::BranchNode(Box::new(node.clone()), Box::new(other_node.clone())));
                        }
                    },
                    None => {new_map.data.insert(path.clone(), node.clone());}
                };
            } else {
                new_map.data.insert(new_path, node.clone());// already empty path
            }
        }
        Some(new_map)
    }
}

