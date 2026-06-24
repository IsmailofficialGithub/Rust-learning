//Moving in rust
// remember this code is never get compiled because it cannot meet requirnment of OwnerShip and its owner is moves to other variable but we still trying to print it .
fn create_string (){
    let s1 =String::from("this is s1");
    print_string(s1);
// Now owner of s1 is s2 that we passed it in Print_string function as parameter
    println!("{}",s1); //Wrong

}

fn print_string(s2:String){
    println!("{}",s2)
}


fn main(){
    
}