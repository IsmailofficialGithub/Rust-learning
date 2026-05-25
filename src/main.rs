
//syntax
struct User {
    name : String,
    age : u8,
    email:String
}

fn main(){
println!("Hello , This is about structs in rust");

let new_user= User{
        name:String::from("Ismail Abbasi"),
        age:19,
        email:String::from("ismail.official295@gmail.com")
};
println!("{}",new_user.name);
println!("{}",new_user.age);
println!("{}",new_user.email);




}