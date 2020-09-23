use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn my_insert<'a>(hashmap: &'a mut HashMap<&'static str, i32>, key: &'static str, value: i32) -> &'a HashMap<&'static str, i32>{
    //dereferencing hashmap and inserting new key->value pair
    (*hashmap).insert(key, value);
    return hashmap; //returning the hashmap with the new values
}

fn initialize<'a>(mut vector: Vec<HashMap<&'a str, i32>>) -> Vec<HashMap<&'a str, i32, RandomState>> {
    //initializing 36 spots: 26 english letters + 10 numbers
    for n in 0..36 {
        vector.push(HashMap::new());
    }
    return vector;
}


pub fn add<'a>(mut vector: Vec<HashMap<&'a str, i32>>, name: &str) -> bool {
    let mut ch : char = name.chars().next().expect("?"); //extracting first character
    ch = ch.to_ascii_lowercase();
    let i  = ch.to_digit(10);
    println!("{:?}\t{:?}", ch, i);
    return 1 == 2;
}

pub(crate) fn main(){
    let mut v = Vec::new();
    v = initialize(v);
    println!("{:#?}", v);
    add(v, "Tesla");
}