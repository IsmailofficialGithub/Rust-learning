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
    let input=vec![(String::from("class"),20)];
    let my_hash_map=group_values_by_keys(input);
    println!("{:?}",my_hash_map);

}


// Function that take Vector of tuple and return HashMap for that Tuple values
//Dummy Vector of tuple => vec![(String::from"Str1",20),()]

fn group_values_by_keys(pairs:Vec<(String,i32)>)-> HashMap<String,i32>{
    let mut hm:HashMap<String,i32>=HashMap::new();
    for (index,value) in pairs{
        hm.insert(String::from(index),value);
    }
    return hm;
}
