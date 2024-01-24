use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::key::Key;

pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

pub struct NodeNetwork {
    pub node: Node,
    pub local_storage: Arc<Mutex<HashMap<Key, String>>>,
}

impl Node {
    pub fn new(ip: String, port: u16) -> Node {
        let input = format!("{}:{}", ip, port);
        let id = Key::new(input);

        Node { ip, port, id }
    }
}
