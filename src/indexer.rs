use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::RandomState;


/*fn my_insert<'a>(mut hashmap: HashMap<&str, i32>, key: &'a str, value: i32) -> &HashMap<&str, i32, RandomState> {
    hashmap.insert(key, value);
    &hashmap;
}
 */

fn add(a: &mut i32, b : i32) -> &i32 {
    *a = *a +  b;
    return a;
}

fn my_insert<'a>(hashmap: &'a mut HashMap<&'static str, i32>, key: &'static str, value: i32)
    -> &'a HashMap<&'static str, i32>{
    (*hashmap).insert(key, value);
    return hashmap;
}

pub fn lolakos(){
    println!("Hola muchacho!");
    let a = &mut 3;
    let x = add(a, 4);
    println!("{}", a);
    let mut book_reviews = &mut HashMap::new();
    my_insert(book_reviews, "hello", 42);
    println!("{}", book_reviews.get("hello").expect("lolakos"));
    /*let arr: [&HashMap<&str, i32, RandomState>; 3] = [my_insert(book_reviews, "lol", 31),
        my_insert(HashMap::new(), "lolk", 310),
        my_insert(HashMap::new(), "lolo", 3154)];

    for &sticazzi in &arr{
        println!("{}", 42);
    }

     */
}