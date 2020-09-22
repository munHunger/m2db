use std::boxed::Box;
use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Result;
use std::io::SeekFrom;
use std::path::Path;
use std::str;
use std::string::String;

pub trait Writable {
    fn write(&self) -> Vec<u8>;
}

pub trait Readable {
    fn read() -> Self;
}

pub trait Document: Writable + Display {}

pub struct Stock {
    name: String,
    value: u8,
}

impl Document for Stock {}

impl Display for Stock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

impl Writable for Stock {
    fn write(&self) -> Vec<u8> {
        let name = self.name.as_bytes();
        return [
            &[self.value],
            &transform_u32_to_array_of_u8(name.len() as u32)[..],
            &name[..],
        ]
        .concat();
    }
}

impl Stock {
    fn read(file: &mut impl Read) -> Self {
        let mut value = [0; 1];
        file.read_exact(&mut value).expect("could not read value");
        let name_size = read_u32(file).expect("Could not read size of name");
        let mut name = vec![0; name_size as usize];
        file.read_exact(&mut name)
            .expect("Could not read name or wrong size");
        return Stock {
            name: String::from_utf8(name).expect("invalid UTF-8"),
            value: value[0],
        };
    }
}

fn read_u32(r: &mut impl Read) -> Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

pub struct Node {
    id: u8,
    name: String,
    data: Box<dyn Document>,
}
impl Node {
    pub fn new() -> Node {
        let stock = Box::new(Stock {
            name: "tesla".to_string(),
            value: 2,
        });
        return Node {
            id: 1,
            name: "portfolio".to_string(),
            data: stock,
        };
    }

    fn to_buffer(&mut self) -> Vec<u8> {
        let data = self.data.write();
        let name = self.name.as_bytes();
        return [
            &[self.id],
            &transform_u32_to_array_of_u8(name.len() as u32)[..],
            &name[..],
            &data[..],
        ]
        .concat();
    }

    fn read(r: &mut impl Read) -> Self {
        let mut id = [0; 1];
        r.read_exact(&mut id).expect("Cannot read id");
        let name_size = read_u32(r).expect("Could not read the size of the name");
        let mut name = vec![0; name_size as usize];
        r.read_exact(&mut name)
            .expect("Could not read name or wrong size");
        let stock = Box::new(Stock::read(r));
        return Node {
            id: id[0],
            name: String::from_utf8(name).expect("invalid UTF-8"),
            data: stock,
        };
    }
}

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

pub struct M2DB {
    data: File,
    data_fragments: File,
    relations: File,
    index: File,
}

impl M2DB {
    fn create_if_not_exists(path: &String) {
        if !Path::new(path.as_str()).exists() {
            File::create(path).expect(&format!("could not create file {}", path)[..]);
        }
    }

    fn open_file(path: String) -> File {
        M2DB::create_if_not_exists(&path);
        return OpenOptions::new()
            .read(true)
            .write(true)
            .open(path.as_str())
            .expect(&format!("could not open file {}", path)[..]);
    }

    pub fn open_database(path: &str) -> M2DB {
        let m2db = M2DB {
            data: M2DB::open_file(format!("{}/data.m2db", path)),
            data_fragments: M2DB::open_file(format!("{}/data.frag.m2db", path)),
            relations: M2DB::open_file(format!("{}/relations.m2db", path)),
            index: M2DB::open_file(format!("{}/index.m2db", path)),
        };
        return m2db;
    }

    pub fn delete_bytes(&mut self, start: u32, size: u32) {
        self.data_fragments
            .seek(SeekFrom::Start(0))
            .expect("could not seek to start of fragment file");
        self.data_fragments
            .write_all(
                &[
                    &transform_u32_to_array_of_u8(start)[..],
                    &transform_u32_to_array_of_u8(size)[..],
                ]
                .concat(),
            )
            .expect("Could not write fragment");
    }

    fn find_fragment_position(&mut self, size: u32) -> u32 {
        let frag_size = self
            .data_fragments
            .seek(SeekFrom::End(0))
            .expect("could not seek to end of frag file");
        if frag_size != 0 {
            self.data_fragments
                .seek(SeekFrom::Start(4))
                .expect("could not seek to start of fragment file");
            let s = read_u32(&mut self.data_fragments).expect("could not read size in fragment");
            println!("size of fragment {}, size of request {}", s, size);
            if s >= size {
                self.data_fragments
                    .seek(SeekFrom::Current(-4))
                    .expect("could not go back to position in fragment");
                return read_u32(&mut self.data_fragments)
                    .expect("could not read position in fragment");
            }
        }
        return self
            .data
            .seek(SeekFrom::End(0))
            .expect("could not read size of data file") as u32;
    }

    /// Adds a node and returns the byte offset to the node
    /// # Arguments
    /// * `node` the node to save
    /// # Examples
    /// ```
    /// use m2db::M2DB;
    /// use m2db::Node;
    /// let mut m2db = M2DB::open_database("./m2db");
    /// m2db.add_node(&mut Node::new());
    /// ```
    pub fn add_node(&mut self, node: &mut Node) -> u32 {
        let node_buffer = node.to_buffer();
        let pos = self.find_fragment_position(node_buffer.len() as u32);
        self.data
            .seek(SeekFrom::Start(pos as u64))
            .expect("could not seek to write position in data file");
        println!("writing {} bytes", node_buffer.len());
        self.data
            .write_all(&node_buffer)
            .expect("Could not write data to disk");
        return pos;
    }
}
