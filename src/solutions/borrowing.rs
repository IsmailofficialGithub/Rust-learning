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


// cannot borrow any other variable if one is already mut referance
fn not_two_mut(){
  let  mut s1=String::from("Ismail");

  let s2=&mut s1;

  let s3=&s1;

  println!("{} , {} , {}",s2,s2,s3);
}

