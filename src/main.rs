#[warn(dead_code)]
use std::collections::HashMap;

fn main() {
    let mut database = Database::new().expect("!!ERROR!!: Database creation failed");
    
    let mut arguments = std::env::args().skip(1);
    
    let action = arguments.next().expect("!!ERROR!!: No action given").to_uppercase();

    if action=="W" {
        if std::env::args().len() > 4 { println!("!WARNING!: too many args given"); }

        let key = arguments.next().expect("!!ERROR!!:  No valid key given").to_uppercase();
        let value = arguments.next().expect("!!ERROR!!:  No valid value given");
        
        if database.map.contains_key(&key) {println!("!WARNING!: Key was overwritten")}
        database.insert(key.clone(), value.clone());
        
        println!("Set value '{}' for the key '{}'!", &value, &key);
    }

    else if action=="R" {
        if std::env::args().len() > 3 { println!("!WARNING!: too many args given"); }

        let key = arguments.next().expect("!!ERROR!!:  No valid key given").to_uppercase();
        println!("'{}' contains the value '{}'",&key, database.map.get(&key).expect("!!ERROR!!: Key not valid"));
    }

    else if action=="P" {
        if std::env::args().len() > 2 { println!("!WARNING!: too many args given"); }
        
        let mut comment = "no entries\n"; 
        if database.raw.len() != 0 { comment = ""}

        println!("\n=====================\n{}{}=====================\n", comment, &database.raw);
    }

    else if action=="D" { 
        if std::env::args().len() > 3 { println!("!WARNING!: too many args given"); }

        let key = arguments.next().expect("!!ERROR!!:  No valid key given").to_uppercase();
        database.map.remove(&key).expect("!!ERROR!!: Key not valid");
        println!("Successfully removed {} from database", key);
    }

    else if action=="E" {
        if std::env::args().len() > 1 { println!("!WARNING!: too many args given"); }
        database.clear();
        println!("Database cleared");
    }

    else {
        println!("!!ERROR!!: no valid option given")
    }

}

struct Database {
    map: HashMap<String, String>,
    raw: String,
    flushed: bool
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read file

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("!!ERROR!!: database corrupted - faulty key!");
            let value = chunks.next().expect("!!ERROR!!: database corrupted - faulty value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse string
        // populate map
        Ok(
            Database{
                map: map,
                raw: contents,
                flushed: false
            }
        )
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn clear(&mut self) {
        let _ = self.map.drain();
    }

    fn _flush(&mut self) {
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
        
        std::fs::write("kv.db", contents).expect("!!ERROR!!: failed to write");
        
        db.flushed = true;
        println!("Data written to disk");
    }
}