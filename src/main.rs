fn main(){
    println!("Mutable structs in rust");
    let mut car=MutStructCar{
        brand:String::from("Toyota"),
        year_of_manufactured:2001,
        color:String::from("Black"),
        engine:String::from("V8")
    };
    println!("{}",car.brand);
    car.brand=String::from("BMW");
    println!("{}",car.brand);
    println!("{}",car.year_of_manufactured);
    println!("{}",car.color);
    println!("{}",car.engine);

}

struct MutStructCar {
    brand:String,
    year_of_manufactured : u16,
    color:String,
    engine:String
}