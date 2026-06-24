

use std::fs::read_to_string;
fn main(){
    let result:Result<String, std::io::Error> =read_to_string("a.txt");                            

    match result{
        Ok(data)=> println!("The content of the file is {}",data),
        Err(error)=> println!("Error while reading the file: {:?}",error),
    }
}