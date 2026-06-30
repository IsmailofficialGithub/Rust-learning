
struct User<'a> {
    name:&'a str,
}
fn main(){
    let user;
    {    
        let name = String::from("Ismail");
            user=User{name:&name};
    }

        println!("{}",user.name);
     
}