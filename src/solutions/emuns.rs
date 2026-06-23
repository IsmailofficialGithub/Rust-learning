enum Direction {
    South,
    East,
    West,
    North
}

fn main(){
    let eDirection:Direction=South;
    my_direction(eDirection)
}

fn my_direction(direction:Direction){
    println!("My Direction is {}",direction)
}