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

    fn remove_relation(&mut self, child_id: u8) {
        for (x, child) in self.relation.iter().enumerate() {
            match child {
                Node {
                    data: _,
                    id: i,
                    relation: _,
                } if *i == child_id => {
                    println!(
                        "removing node {}:{} at position {}",
                        child.data, child_id, x
                    );
                    self.relation.remove(x);
                    return;
                }
                _ => (),
            }
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
    let child2 = Node::new(23, String::from("sibling"));
    node.add_relation(child2);
    println!("{},{} -> {}", node.data, node.id, node.relation[0].data);
    node.remove_relation(2);
    println!("{},{} -> {}", node.data, node.id, node.relation[0].data);
}
