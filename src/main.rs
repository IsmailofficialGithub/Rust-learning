struct Rect {
    width:u32,
    height:u32
}

impl Rect{
    fn area(&self) -> u32{
        self.width*self.height
    }
    fn parameter (&self,num:u32)-> u32{
        2 * (self.width*self.height)
    }
    fn debug ()->u32{
        return 3;
    }
}

fn main(){
    let rect1=Rect{
        width:30,
        height:40
    };
    let area=rect1.area();
    let parameter=rect1.parameter(32);
    println!("{},{}",rect1.width,rect1.height);
    println!("{}",area);
    println!("{}",parameter);
    println!("Debug functions is calling directly , Debug= {} ",Rect::debug());
}