use tokio::time::{sleep, Duration};
pub async fn task(num: u32) {
	for i in 1..=5 {
    	println!("Task{}.{}      	|", num, i);
    	sleep(Duration::from_secs(5)).await;
	}
}

#[tokio::main]
async fn main() {
	let jh1 = tokio::task::spawn(task(1));
	let jh2 = tokio::task::spawn(task(2));
	let _ = tokio::join!(jh1, jh2);
	// or use jh1.await; jh2.await;
}