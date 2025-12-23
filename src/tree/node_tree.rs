use crate::map::compress::CompressionMap;
use crate::utils::*;
use crate::errors::ZipError;
use crate::tree::node;
use node::{Node, Leaf};

#[derive(Debug)]
pub struct Tree {
    root: Option<Node>,
    nodes_left: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None, nodes_left: vec![] }
    }

    pub fn set_root(&mut self, node: &Node) {
        self.root=Some(node.clone());
    }

    pub fn search(&self, vector: &mut Vec<bool>) -> Option<u8> {
        if let Some(root) = &self.root {
            return root.search(vector);
        }
        None
    }

    pub fn from_file(filename: impl AsRef<str>) -> Result<Self, ZipError> {
        let mut tree = Self::new();
        for (byte, byte_count) in count_bytes(filename.as_ref())?.iter() {
            tree.push_node(byte, byte_count);
        }
        tree.construct();
        Ok(tree)
    }

    pub fn push_node(&mut self, byte: &u8, byte_count: &u64) {
        self.nodes_left.push(Node::LeafNode(
            Leaf { byte: byte.clone(), byte_count: byte_count.clone() } ));
    }

    pub fn pop_lowest(&mut self) -> Option<Node> {
        let mut lowest_weigh = match self.nodes_left.first() {
            Some(elem) => elem.weigh(),
            None => return None
        };
        let mut id = 0;
        for (i, item) in self.nodes_left.iter().enumerate() {
            let weigh = item.weigh();
            if weigh<lowest_weigh {
                lowest_weigh = weigh;
                id=i;
            }
        }
        Some(self.nodes_left.remove(id))
    }

    fn construct_once(&mut self) -> bool {
        let a = match self.pop_lowest() {
            Some(n) => n,
            None => return false
        };
        let b = match self.pop_lowest() {
            Some(n) => n,
            None => {
                self.root = Some(a);
                return false
            }
        };
        let new_node = Node::BranchNode(Box::new(a), Box::new(b));
        self.nodes_left.push(new_node);
        true
    }

    pub fn construct(&mut self) {
        while self.construct_once() {}
    }
}

impl From<&Tree> for CompressionMap {
    fn from(value: &Tree) -> Self {
        match &value.root {
            Some(root) => {
                let mut map = CompressionMap::new();
                map.map(root, vec![]);
                map
            },
            None => CompressionMap::new()
        }
    }
}

