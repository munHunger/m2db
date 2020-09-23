use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::RandomState;
use std::process::exit;

fn add(a: &mut i32, b : i32) -> &i32 {
    *a = *a +  b;
    return a;
}

/** Creating an Hashmap insert that returns the Hashmap with the new key->value **/
fn my_insert<'a>(hashmap: &'a mut HashMap<&'static str, i32>, key: &'static str, value: i32) -> &'a HashMap<&'static str, i32>{
    //dereferencing hashmap and inserting new key->value pair
    (*hashmap).insert(key, value);
    return hashmap; //returning the hashmap with the new values
}

//building a wrapper for my hashmap
#[derive(Debug)]
struct DumbHashMap<'a> {
    hash_map: HashMap<&'a str, i32>
}

impl <'a> DumbHashMap<'a> {
    fn new() -> DumbHashMap<'a> {
        DumbHashMap {hash_map: HashMap::new() }
    }

    fn insert(&mut self, key: &'static str, value: i32) -> &mut DumbHashMap<'a> {
        self.hash_map.insert(key, value);
        return self;
    }
}
/*
#[derive(Debug)]
struct Indexer<'a> {
    indexes:  Vec<HashMap<&'a str, i32>>
}

impl<'a> Indexer<'a> {
    fn new() -> Indexer<'a> {
        Indexer {
            indexes: Vec::with_capacity(36)
        }
    }

    fn initialize(&mut self) -> Vec<HashMap<&'a str, i32, RandomState>> {
        for _ in 0..35{
            self.indexes.push(HashMap::new());
        }
        return self.indexes;
    }
}*/


pub fn lolakos(){
    println!("Hola muchacho!");
    let a = &mut 3;
    let _x = add(a, 4);
    println!("{}", a);
    let book_reviews = &mut HashMap::new();
    my_insert(book_reviews, "hello", 42);
    println!("{}", book_reviews.get("hello").expect("lolakos"));
    let mut x = DumbHashMap::new();
    x.insert("sarcazzo", 42);
    println!("{:?}", x);

    let mut test_vec = Vec::with_capacity(3);
    for n in 0..3 {
        test_vec.push(HashMap::new());
        println!("{:?}", my_insert(&mut test_vec[n], "lol", 23));//.get(n).expect("lolakos").insert("prova", 42));
    }
    for x in test_vec{
        println!("{:?}", x); //the :? in the print is the Debug specifier
    }

    //let ind = Indexer::new();
    //println!("{:?}", ind);


    /*[ my_insert(book_reviews, "lol", 31),
     my_insert(&mut HashMap::new(), "lol", 310),
     my_insert(&mut HashMap::new(), "lol", 3154)];*/
    /*for &sticazzi in &arr{
        println!("{}", sticazzi.get("lol").expect("E invece no"));
    }*/
}