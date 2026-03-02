// fn main(){

//     let string:String=String::from("Hello String");

//     let length: Option<char>=string.chars().nth(10);
//     match length{
//         Some (c)=>print!("{}",c),
//         None => print!("No character at this index")

//     }
//     print!("{:?}",length)

// }




// fn main(){
//     let sentence =String::from("Ismail is my first word in name");
//     let n:i8=100;
//     for i in 0..n {
//         println!("loop is in  loop {}",i)
//     }


//     let first_world= get_first_word(sentence);
//     print!("First word is :{}", first_world);



//     fn get_first_word(sentence:String)->String  {
//         let mut ans=String::from("");

//         for char in sentence.chars() {
//            ans.push_str(char.to_string().as_str());
//            if char == ' ' {
//             break ;
//            }
//         }
//         return ans;
        
//     }
// }

// fn main(){
//     let value= String::from ("my name is ismail");
//     print_value(value);

// }

// fn print_value(string:String)->String{
//     println!("The value in argument is : '{}'",string);
//     return string;
// }


// fn main(){
//     let number:i8=20;
//     for i in 1..number{

//         print!("{}",i);
//     }
// }


fn main(){
    let number:i8=12;
    if number <10{
        print!("Value {} is less than 10",number)
    }else{
        print!("Value {} is greater than 10",number)
    }
}