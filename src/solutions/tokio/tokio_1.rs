use tokio::time::{sleep,Duration,spawn};

#[tokio::main]
async fn main(){
    println!("Hello, world!");
    sleep(Duration::from_millis(5000)).await;
    spawn(async {
        println!("Hello, world! (in a new thread)");
        sleep(Duration::from_millis(5000)).await;
        println!("Five seconds later (in a new thread)");
    });
    println!("Five seconds later (in the main thread)");
}