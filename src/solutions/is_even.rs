fn main(){
    let number:i32=4;
    println!("{}",is_even(number))
}


fn is_even(number:i32)->bool{
    if number % 2 == 0 {
        return true
    }
    return false
}