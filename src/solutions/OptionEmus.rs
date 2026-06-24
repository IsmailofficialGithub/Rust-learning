fn main (){
    let index=find_First_a(String::from("Cargorun"));
        match index{
            Some(value)=>println!("The index of first 'a' is {}",value),
            None=>println!("There is no 'a' in the string"),
        }

}

fn find_First_a (string:String)-> Option<i32> {
    // string = Cargorun 

    for (index,char) in string.chars().enumerate()
    {
        if char =='a'
        {
            return Some (index as i32);
        }
    
    }
    None
}