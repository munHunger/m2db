use std::boxed::Box;
use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use std::str;
use std::string::String;

trait Writable {
    fn write(&self) -> Vec<u8>;
}

trait Readable {
    fn read() -> Self;
}

trait Document: Writable + Display {}

struct Stock {
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

struct Node {
    id: u8,
    name: String,
    data: Box<dyn Document>,
}
impl Node {
    fn new() -> Node {
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

struct M2DB {
    data: File,
    relations: File,
    index: File,
}

impl M2DB {
    fn open_database(path: &str) -> M2DB {
        if !Path::new(&format!("{}/data.m2db", path)).exists() {
            File::create(format!("{}/data.m2db", path));
        }
        if !Path::new(&format!("{}/index.m2db", path)).exists() {
            File::create(format!("{}/index.m2db", path));
        }
        if !Path::new(&format!("{}/relations.m2db", path)).exists() {
            File::create(format!("{}/relations.m2db", path));
        }

        let m2db = M2DB {
            data: OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("{}/data.m2db", path))
                .unwrap(),
            relations: OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("{}/relations.m2db", path))
                .unwrap(),
            index: OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("{}/index.m2db", path))
                .unwrap(),
        };
        return m2db;
    }
}

fn main() -> std::io::Result<()> {
    let mut f = File::create("./test.m2db")?;
    let buf = Node::new().to_buffer();
    f.write_all(&buf)?;

    f = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./test.m2db")?;
    let n = Node::read(&mut f);
    println!("id {} name {} data {}", n.id, n.name, n.data);

    Ok(())
}
