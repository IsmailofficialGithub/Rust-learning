use std::thread;
fn main (){
    let handle= thread::spawn(||{
        for i in 0..5{
            println!("Hi from spawn Thread {}",i);
        }
    });

    for i in 0..500 {
        println!("Hi, from main THread {}",i);
    }
    handle.join().unwrap();
}


// Multithreading with vectors

use std::thread;
fn main (){
    let v=vec![1,2,3];
    let handle= thread::spawn(move|| {
        println!("Here from a Vector {:?}",v);
    });
    println!("{:?}",v);

    handle.join().unwrap();
}