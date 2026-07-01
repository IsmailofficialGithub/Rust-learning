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

// Message Passing from one thread to other

use std::{sync::mpsc, thread::{ spawn}};
fn main ( ){
let (tx,rx)= mpsc::channel();

    for i in 0..10 {
        let producer=tx.clone();
        spawn(move||{
            let mut sum:i64 =0;
            for j in i*10000000..(i+1*10000000)-1{
                sum=sum+j;
            }
        producer.send(sum).unwrap();
        });
    }
        drop(tx);
    let mut final_sum=0;
    for val in rx{
        println!("recv value from thread");
        final_sum= val+final_sum;
    }
    println!("{}",final_sum);
}