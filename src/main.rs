#[tokio::main (flavor="multi_thread" , worker_threads=10)]

async fn main() {
    // println!("Hello, world!");
    print_with_wait().await;
}

async fn print_with_wait (){

    std::thread::sleep(std::time::Duration::from_millis(5000));
    print!("Print with wait .... waiting 5 seconds");
}