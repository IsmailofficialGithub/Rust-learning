fn main() {
    let longest_str;
    let s1 = String::from("longest");
    {
        let s2 = String::from("Small");
        longest_str = longest(&s1, &s2);
        println!("{}", longest_str);
    }
}

fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    };
}
