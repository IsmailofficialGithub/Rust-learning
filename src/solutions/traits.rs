trait Summary {
    fn summarise (&self)->String{
        return String::from("No Function is written in this struct");
    }
}

trait Fix {
    fn fix(&self)->String{
        return String::from("Hello from fix");
    }
}

struct User{
    name:String,
    age:u32
}

impl Summary for User{
    fn summarise(&self)->String{
        return format!("Name of user is {} and age of user is {}.",self.name,self.age);
    }
}
impl Fix for User{}

fn notify <T:Summary + Fix>(u :T){
    println!("{}",u.summarise());
}
impl Summary for String{}
fn main (){
    let user =User{
        name:String::from("Ismail Abbasi"),
        age:19
    };
    // println!("{}",user.summarise());
    notify(user);
}