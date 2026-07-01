use std::thread;
fn main (){
    let v=vec![1,2,3];
    let handle= thread::spawn(move|| {
        println!("Here from a Vector {:?}",v);
    });
    println!("{:?}",v);

    handle.join().unwrap();
}