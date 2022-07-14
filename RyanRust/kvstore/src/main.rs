use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().unwrap();
    println!("The key is '{}', and the value is '{}'", key, value);
    //let contents = format!("{}\t{}\n", key, value);// file will look like this
                                                           // mykey\t myvalue \n mykey2 \t value2
    //std::fs::write("kv.db", contents).unwrap();
    // match write_result { //THIS INSTEAD OF UNWRAP()
    //     Ok(()) => {

    //     }
    //     Err(e) => {

    //     }
    // }
    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
    
    
}

struct Database {
    map: HashMap<String, String>, //<String, String> -> generics
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //read the kv.dv file
        //parse the string
        //populate our map
        /* 
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c, //same as Result::Ok
            Err(error) => {
            return Err(error); //same as Result::Err(error);
            }
        }
        */
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?; //exactly the same as the code above
        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t'); //HOW does this work?
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        
        Ok(Database { map: map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for pairs in self.map {
            let kvpair = format!("{}\t{}\n", pairs.0, pairs.1);
            contents.push_str(&kvpair);
        }
        std::fs::write("kv.db", contents)
    }
}