//Hashmap in rust
use std::collections::HashMap;

fn main() {
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("hashmap"), 19);
    users.insert(String::from("Data"), 10);

    let first_user = users.get("hashmap");
    match first_user {
        Some(value) => println!("{:?}", value),
        None => println!("No HASHMAP found"),
    }
}
