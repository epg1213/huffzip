#[derive(Debug, PartialEq, Clone)]
pub struct Leaf {
    pub byte: u8,
    pub byte_count: u64
}

impl Leaf {
    pub fn new(byte: u8, byte_count: u64) -> Self {
        Self { byte: byte, byte_count: byte_count }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    LeafNode(Leaf),
    BranchNode(Box<Self>, Box<Self>)
}

impl Node {
    pub fn weigh(&self) -> u64 {
        return match self {
            Node::LeafNode(leaf) => { leaf.byte_count.clone() },
            Node::BranchNode(n1, n2) => n1.weigh() + n2.weigh()
        };
    }

    pub fn search(&self, vector: &mut Vec<bool>) -> Option<u8> {
        match self {
            Node::LeafNode(leaf) => return Some(leaf.byte),
            Node::BranchNode(n1, n2) => {
                if vector.len() == 0 {
                    return None;
                }
                if vector.remove(0) {
                    return n2.search(vector);
                }
                return n1.search(vector);
            }
        };
    }
}

