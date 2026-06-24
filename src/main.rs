//Borrowing in rust

//& is used to Borrow a variable
fn create_string (){
    let s1 =String::from("this is s1");
    print_string(&s1);
    println!("{}",s1); 

}

fn print_string(s2:&String){
    println!("{}",s2)
}


fn main(){
    
}