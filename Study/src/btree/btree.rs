use crate::btree::node::{BNode, BNODE_INTERNAL, BNODE_LEAF, BTREE_PAGE_SIZE};

struct BTree {
    root: BNode,
}

impl BTree {
    
    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        
        // Check limit
        if key.len() + value.len() > BTREE_PAGE_SIZE {
            panic!("Key and value too long");
        }
        // Create root node if needed
        if self.root.is_empty() {
            self.root = BNode::new(vec![0; BTREE_PAGE_SIZE]);
            // later
        }
        
        // Insert key
        let node = self.tree_insert(self.root, key, value);
    }
    
    pub fn delete(&mut self, key: &[u8]) {
        todo!()
    }
    
    fn get(&self, index: u64) -> &BNode {
        todo!()
    }
    
    fn new(&self, node: &BNode) -> u64 {
        todo!()
    }

    fn del(&self, index: u64) {
        todo!()
    }
    
    fn tree_insert(&mut self, node: BNode, key: &[u8], value: &[u8]) -> BNode {
        // Extra size allows to exceed 1 page temporarily
        let mut new = BNode::new(vec![0; BTREE_PAGE_SIZE * 2]);
        // Where to insert the key?
        let index = node.node_lookup_less_than_or_equal(key);
        if node.b_type() == BNODE_LEAF {
            if key == node.get_key(index) {
                new.leaf_update(&node, index + 1, key, value);
            } else {
                new.leaf_insert(&node, index + 1, key, value);
            }
        } else { // BNODE_INTERNAL
            // Recursive insertion to the kid node
            let kid_pointer = node.get_pointer(index);
            let kid_node = self.tree_insert(self.get(kid_pointer).clone(), key, value);
            // After insert, split the result
            let (maybe_split_left, maybe_split_middle, maybe_split_right) = kid_node.node_split_three();
            self.del(kid_pointer);
        }
        new
    }
    
    fn node_replace_kid(
        &mut self, mut new: BNode,mut old: BNode, index: u16,
        maybe_left: Option<BNode>, maybe_middle: Option<BNode>, maybe_right: Option<BNode>) {
        
        let mut n = 0;
        
        if maybe_left.is_some() { n += 1 };
        if maybe_middle.is_some() { n += 1 };
        if maybe_right.is_some() { n += 1 };
        
        new.set_header(BNODE_INTERNAL, old.n_keys() + n - 1);
        new.append_range(&old, 0, 0, index);
        if let Some(left) = maybe_left {
            new.append_key_value(index, self.new(&left), left.get_key(0), 0u16.to_le_bytes().as_slice());
        }
        if let Some(middle) = maybe_middle {
            new.append_key_value(index + 1, self.new(&middle), middle.get_key(0), 0u16.to_le_bytes().as_slice());
        }
        if let Some(right) = maybe_right {
            new.append_key_value(index + 2, self.new(&right), right.get_key(0), 0u16.to_le_bytes().as_slice());
        }
        new.append_range(&old, index + n, index + 1, old.n_keys() - (index + 1));
    }
}