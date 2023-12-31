use super::key::Key;

pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

impl Node {
    pub fn new(ip: String, port: u16) -> Node {
        let input = format!("{}:{}", ip, port);
        let id = Key::new(input);

        Node { ip, port, id }
    }
}
