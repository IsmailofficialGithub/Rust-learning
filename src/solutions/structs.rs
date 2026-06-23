
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


            // tuple structs
// fn main (){
//     let black = Color(0,0,0);
//     println!("{},{},{}",black.0,black.1,black.2);

// }

// struct Color(i32,i32,i32);


//             impl Functions to struct
// struct Rect {
//     width:u32,
//     height:u32
// }

// impl Rect{
//     fn area(&self) -> u32{
//         self.width*self.height
//     }
//     fn parameter (&self,num:u32)-> u32{
//         2 * (self.width*self.height)
//     }
//     fn debug ()->u32{
//         return 3;
//     }
// }

// fn main(){
//     let rect1=Rect{
//         width:30,
//         height:40
//     };
//     let area=rect1.area();
//     let parameter=rect1.parameter(32);
//     println!("{},{}",rect1.width,rect1.height);
//     println!("{}",area);
//     println!("{}",parameter);
//     println!("Debug functions is calling directly , Debug= {} ",Rect::debug());
// }