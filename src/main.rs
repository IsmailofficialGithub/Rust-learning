struct User {
    name: String,
    age: u8,
    email: String,
    active: bool,
}
fn main() {
    let user1 = User {
        name: String::from("Ismail"),
        age: 22,
        email: String::from("old@gmail.com"),
        active: true,
    };

    let user2=User{
        name:String::from("Rohan"),
        ..user1
    };

    println!("{}",user1.name);
    println!("{}",user2.name);

}