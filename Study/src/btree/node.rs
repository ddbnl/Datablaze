use std::cmp::Ordering;

pub const BTREE_PAGE_SIZE: usize = 4096;
pub const BTREE_MAX_KEY_SIZE: usize = 1000;
pub const BTREE_MAX_VAL_SIZE: usize = 3000;
pub const BNODE_LEAF : u16 = 0;
pub const BNODE_INTERNAL : u16 = 1;

pub struct Node {
    keys: Vec<u8>,
    vals: Vec<u8>,
    kids: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BNode {
    data: Vec<u8>,
}

impl BNode {

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set_header(&mut self, b_type: u16, n_keys: u16) {
        self.data[0..2].copy_from_slice(&b_type.to_le_bytes());
        self.data[2..4].copy_from_slice(&n_keys.to_le_bytes());
    }
    
    pub fn b_type(&self) -> u16 {
        decode_header_field(&self.data, 0, 2)
    }
    pub fn n_keys(&self) -> u16 {
        decode_header_field(&self.data, 2, 4)
    }

    pub fn get_pointer(&self, index: u16) -> u64 {
        assert!(index < self.n_keys());
        let position = (4 + 8 * index) as usize;
        u64::from_le_bytes(self.data[position..position + 8].try_into().unwrap())
    }
    
    pub fn set_pointer(&mut self, index: u16, pointer: u64) {
        assert!(index < self.n_keys());
        let position = (4 + 8 * index) as usize;
        self.data[position..position + 8].copy_from_slice(&pointer.to_le_bytes());
    }
    
    pub fn get_offset(&self, index: u16) -> u16 {
        if index == 0 {
            return 0
        }
        let position = (4 + 8 * self.n_keys() + 2 * (index - 1)) as usize;
        u16::from_le_bytes(self.data[position..position + 2].try_into().unwrap())
    }
    
    pub fn set_offset(&mut self, index: u16, offset: u16) {
        
        let position = (4 + 8 * self.n_keys() + 2 * (index - 1)) as usize;
        self.data[position..position + 2].copy_from_slice(&offset.to_le_bytes());
    }

    pub fn key_value_position(&self, index: u16) -> u16 {
        assert!(index < self.n_keys());
        4 + 8 * self.n_keys() + 2 * self.n_keys() + self.get_offset(index)
    }
    
    pub fn get_key(&self, index: u16) -> &[u8] {
        assert!(index < self.n_keys());
        let position = self.key_value_position(index) as usize;
        let key_length = u16::from_le_bytes(self.data[position..position + 2].try_into().unwrap()) as usize;
        &self.data[position + 4..position + 4 + key_length]
    }
    
    pub fn get_value(&self, index: u16) -> &[u8] {
        assert!(index < self.n_keys());
        let position = self.key_value_position(index) as usize;
        let key_length = u16::from_le_bytes(self.data[position..position + 2].try_into().unwrap()) as usize;
        let value_length = u16::from_le_bytes(self.data[position + 2..position + 2 + 2].try_into().unwrap()) as usize;
        &self.data[position + 4 + key_length..position + 4 + key_length + value_length]
    }
    
    pub fn append_key_value(&mut self, index: u16, pointer: u64, key: &[u8], value: &[u8]) {
        if self.b_type() == BNODE_INTERNAL {
            // Set pointer for internal
            self.set_pointer(index, pointer);
        } else {
            // Set KV for leaf
            let position = self.key_value_position(index) as usize;
            // Set KV length headers
            self.data[position..position + 2].copy_from_slice(&(key.len() as u16).to_le_bytes());
            self.data[position + 2..position + 2 + 2].copy_from_slice(&(value.len() as u16).to_le_bytes());
            // Set KV data
            self.data[position + 4..position + 4 + key.len()].copy_from_slice(key);
            self.data[position + 4 + key.len()..position + 4 + key.len() + value.len()].copy_from_slice(value);
            // Update offset for next key
            self.set_offset(index + 1, self.get_offset(index) + 4 + key.len() as u16 + value.len() as u16)
        }
    }
    
    pub fn size(&self) -> u16 {
        self.key_value_position(self.n_keys())
    }
    
    pub fn leaf_insert(&mut self, old: &BNode, index: u16, key: &[u8], value: &[u8]) {
        self.set_header(BNODE_LEAF, old.n_keys() + 1);
        self.append_range(&old, 0, 0, index); // copy keys before index
        self.append_key_value(index, 0, key, value); // new key
        self.append_range(&old, index + 1, index, old.n_keys() - index); // copy after index
    }
    
    pub fn append_range(&mut self, old: &BNode, dest_new: u16, src_old: u16, n: u16) {
        for i in 0..n {
            let (dst, src) = (dest_new+i, src_old+i);
            self.append_key_value(dst, old.get_pointer(src), old.get_key(src), old.get_value(src));
        }
    }
    
    pub fn leaf_update(&mut self, old: &BNode, index: u16, key: &[u8], value: &[u8]) {
        self.set_header(BNODE_LEAF, old.n_keys());
        self.append_range(&old, 0, 0, index);
        self.append_key_value(index, 0, key, value);
        self.append_range(&old, index + 1, index + 1, old.n_keys() - (index - 1));
    }
    
    pub fn node_lookup_less_than_or_equal(&self, key: &[u8]) -> u16 {
        let n_keys = self.n_keys();
        let mut i = 0;
        while i < n_keys {
            let to_compare_key = self.get_key(i);
            let cmp = key.cmp(to_compare_key);
            match cmp {
                Ordering::Equal => return i,
                Ordering::Greater => return i - 1,
                _ => {},
            }
            i += 1;
        }
        i - 1
    }
    
    pub fn node_split_two(&self) -> (BNode, BNode) {
        
        assert!(self.n_keys() >= 2);
        let mut n_left = self.n_keys() /2; // Initial guess
        
        // Try to fit left half
        let left_bytes = 4 + 8 * n_left + 2 * n_left + self.get_offset(n_left);
        if left_bytes > BTREE_PAGE_SIZE as u16 { 
            n_left -= left_bytes - BTREE_PAGE_SIZE as u16
        }
        assert!(n_left >= 1);
        
        // Try to fit right half
        let right_bytes = self.size() - left_bytes + 4;
        if right_bytes > BTREE_PAGE_SIZE as u16 {
            n_left += right_bytes - BTREE_PAGE_SIZE as u16
        }
        assert!(n_left <= self.n_keys());
        let n_right = self.n_keys() - n_left;
        
        // New nodes
        let mut left = BNode::new([0; BTREE_PAGE_SIZE * 2].to_vec());
        left.set_header(self.b_type(), n_left);
        left.append_range(self, 0, 0, n_left);
        
        let mut right = BNode::new([0; BTREE_PAGE_SIZE].to_vec());
        right.set_header(self.b_type(), n_right);
        right.append_range(self, 0, n_left, n_right);
        assert!(right.size() <= BTREE_PAGE_SIZE as u16);
        (left, right)
    }

    pub fn node_split_three(&self) -> (Option<BNode>, Option<BNode>, Option<BNode>) {
        
        if self.size() <= BTREE_PAGE_SIZE as u16 {
            return (None, None, Some(self.clone()))
        }
        let (left, right) = self.node_split_two();
        if left.size() <= BTREE_PAGE_SIZE as u16 {
            return (None, Some(left), Some(right))
        }
        let (left_left, middle) = left.node_split_two();
        assert!(left_left.size() <= BTREE_PAGE_SIZE as u16);
        (Some(left_left), Some(middle), Some(right))
    }
}

pub fn decode_header_field(data: &Vec<u8>, start: usize, end: usize) -> u16 {
    u16::from_le_bytes(data[start..end]
        .try_into()
        .expect("Failed to convert bytes to u16"))
}
