// &mut mutable borrowing
fn main(){
    let mut s1 = String::from("Ismail");
    some_function(&mut s1);
    println!("{}",s1);  
}

fn some_function (s2:&mut String){
    s2.push_str("Abbasi");
    println!("{}",s2);
    
}