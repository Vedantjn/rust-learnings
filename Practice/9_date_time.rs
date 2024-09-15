use chrono::Utc;

// cargo add chrono 
fn main() {
    let now = Utc::now();
    println!("current time is {}", now);
}
