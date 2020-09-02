use std::string::String;

struct Node {
    data: String,
    id: u8,
    relation: Vec<Node>,
}

impl Node {
    fn new(id: u8, data: String) -> Node {
        Node {
            id: id,
            data: data,
            relation: Vec::new(),
        }
    }

    fn add_relation(&mut self, child: Node) {
        self.relation.push(child);
    }
}

fn main() {
    let mut node = Node::new(8, String::from("parent"));
    let child = Node::new(2, String::from("child"));
    node.add_relation(child);
    println!("Hello, world!");
    println!("{},{} -> {}", node.data, node.id, node.relation[0].data);
}
