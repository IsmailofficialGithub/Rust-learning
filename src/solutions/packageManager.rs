use chrono::{Local,Utc};

fn main (){
    let now =Utc::now();
    println!("Current time is {}",now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted time is {}",formatted);

    let get_local_time = Local::now();
    println!("Local time is {}",get_local_time);
}

