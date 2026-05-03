fn main(){
    println!("* Function that returns lenght of strings");
    let string=String::from("Ismail");
    let len=get_string_length(string);
    println!("Solution : Length of string is {}",len);
}

fn get_string_length(str :String)->usize{
    str.chars().count()
}   