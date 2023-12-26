#[warn(dead_code)]
use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    
    let key = arguments.next().expect("no key found");
    let value = arguments.next().expect("no value found");
    
    println!("The key is '{}' and the value is '{}'!", key, value);

    let mut database = Database::new().expect("Database::new() panicked");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush();
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read file

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("faulty key!");
            let value = chunks.next().expect("faulty value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse string
        // populate map
        Ok(
            Database{
                map: map,
                flushed: false
            }
        )
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&mut self) {
        do_flush(self);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        do_flush(self);
    }

}

fn do_flush(db: &mut Database) {
    if !db.flushed {
        let mut contents = String::new();
        for (key, value) in &db.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        
        std::fs::write("kv.db", contents).expect("failed to write");
        
        db.flushed = true;
        println!("Data written to disk");
    }
}