use crate::btree::node::*;

mod btree;

fn main() {
    let node_1_max = 4 + 1 * 8 + 1 * 2 + 4 + BTREE_MAX_KEY_SIZE + BTREE_MAX_VAL_SIZE;
    assert!(node_1_max <= BTREE_PAGE_SIZE);

    let mut node = BNode::new(vec![0; BTREE_PAGE_SIZE]);
    node.set_header(BNODE_LEAF, 3);
    node.append_key_value(0, 0, "k1".as_bytes(), "wat".as_bytes());
    node.append_key_value(1, 0, "k2".as_bytes(), "b".as_bytes());
    node.append_key_value(2, 0, "k3".as_bytes(), "hello".as_bytes());
    
    
    println!("{}", String::from_utf8(node.get_key(0).to_vec()).unwrap().as_str());
    println!("{}", String::from_utf8(node.get_value(0).to_vec()).unwrap().as_str());
    println!("{}", String::from_utf8(node.get_key(1).to_vec()).unwrap().as_str());
    println!("{}", String::from_utf8(node.get_value(1).to_vec()).unwrap().as_str());
}