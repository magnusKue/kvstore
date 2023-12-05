use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    
    let key = arguments.next().expect("no key found");
    let value = arguments.next().expect("no value found");
    
    println!("The key is '{}' and the value is '{}'!", key, value);
    
    let contents = format!("{}\t{}\n", key, value);    
    
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() panicked");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read file

        // let contents = match std::fs::read_to_string("kv.db") {
            // Ok(c) => c,
            // Err(e) => {
                // return Err(e);
            // }
        // };
        // same thing as:
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
        Ok(Database {
            map: map,
        })
    }
}