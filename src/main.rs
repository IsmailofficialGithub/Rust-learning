// cannot borrow any other variable if one is already mut referance
fn main(){
  let  mut s1=String::from("Ismail");

  let s2=&mut s1;

  let s3=&s1;

  println!("{} , {} , {}",s2,s2,s3);
}

