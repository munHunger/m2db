mod m2db;

fn main() {
    let mut m2db = m2db::M2DB::open_database("./m2db");
    m2db.add_node(&mut m2db::Node::new());
    m2db.add_node(&mut m2db::Node::new());
    m2db.delete_bytes(3, 30);
    m2db.add_node(&mut m2db::Node::new());
}
