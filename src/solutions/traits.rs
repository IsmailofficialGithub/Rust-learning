trait Summary {
    fn summarise (&self)->String;
}

struct User{
    name:String,
    age:u32
}

impl Summary for User{
    fn summarise(&self)->String{
        return format!("Name of user is {} and age of user is {}",self.name,self.age);
    }
}

fn main (){
    let user =User{
        name:String::from("Ismail Abbasi"),
        age:19
    };

    println!(user.summarise());
}