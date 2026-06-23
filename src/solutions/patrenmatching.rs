enum Direction {
    South,
    East,
    West,
    North
}

fn main(){
    let e_direction=Direction::South;
    my_direction(e_direction);
}

fn my_direction(direction:Direction){
    match direction {
        Direction::South => println!("My Direction is {}","South"),
        Direction::East => println!("My Direction is {}","East"),
        Direction::West => println!("My Direction is {}","West"),
        Direction::North => println!("My Direction is {}","North")
    }
    println!("Code compile successfully")
}