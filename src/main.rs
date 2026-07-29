use tokio::time::{sleep,Duration};
use tokio::spawn;

#[tokio::main]
async fn main(){
    println!("Hello, world!");
    spawn(async {
        println!("Hello, world! (in a new thread)");
        sleep(Duration::from_millis(5000)).await;
        println!("Five seconds later (in a new thread)");
    });
    sleep(Duration::from_millis(5000)).await;
    println!("Five seconds later (in the main thread)");
}