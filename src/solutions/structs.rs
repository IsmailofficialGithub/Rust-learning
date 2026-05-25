
             //syntax

// struct User {
//     name : String,
//     age : u8,
//     email:String
// }

// fn main(){
// println!("Hello , This is about structs in rust");

// let new_user= User{
//         name:String::from("Ismail Abbasi"),
//         age:19,
//         email:String::from("ismail.official295@gmail.com")
// };
// println!("{}",new_user.name);
// println!("{}",new_user.age);
// println!("{}",new_user.email);

// }

            //Mutable structs

// fn main(){
//     println!("Mutable structs in rust");
//     let mut car=MutStructCar{
//         brand:String::from("Toyota"),
//         year_of_manufactured:2001,
//         color:String::from("Black"),
//         engine:String::from("V8")
//     };
//     println!("{}",car.brand);
//     car.brand=String::from("BMW");
//     println!("{}",car.brand);
//     println!("{}",car.year_of_manufactured);
//     println!("{}",car.color);
//     println!("{}",car.engine);

// }

// struct MutStructCar {
//     brand:String,
//     year_of_manufactured : u16,
//     color:String,
//     engine:String
// }



            //create a new struct using values from an old struct.
// struct User {
//     name: String,
//     age: u8,
//     email: String,
//     active: bool,
// }
// fn main() {
//     let user1 = User {
//         name: String::from("Ismail"),
//         age: 22,
//         email: String::from("old@gmail.com"),
//         active: true,
//     };

//     let user2=User{
//         name:String::from("Rohan"),
//         ..user1
//     };

//     println!("{}",user1.name);
//     println!("{}",user2.name);

// }