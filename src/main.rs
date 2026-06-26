fn main() {
    let name = String::from("Ismail Abbasi");

    let first= first_word(&name);
    println!("First Word of '{}' is {}",&name,&first);
}

fn first_word(str: &String) -> &str {
    let mut index: usize = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    return &str[0..index];
}
