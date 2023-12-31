extern crate kademlia_rs;
use kademlia_rs::{node::Node, key::Key};

fn main() {
    let key = Key::new("hello world".to_owned());
    println!("{:?}", key);
}
