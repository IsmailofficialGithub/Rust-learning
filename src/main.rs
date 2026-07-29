use tokio::time::{interval,Duration};

#[tokio::main]
async fn main(){
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("Tick");
    }
}