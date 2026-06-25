fn main() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(6);

    println!("Vector is {:?}", even_filter2(&mut vector));
    // println!("{:?}", even_filter(&vector));
}

//function that take vector and returns vector of even number only

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            new_vec.push(*i);
        }
    }
    return new_vec;
}

// approch 2

fn even_filter2(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
