use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::channel;
use tokio::time::{sleep, Duration};

async fn prod(tx: Sender<i32>){
    for elem in 1..=10 {
        let _ = tx.send(elem).await;
        sleep(Duration::from_millis(1000)).await;
    }
    
}

async fn cons(mut rx: Receiver<i32>){
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}


#[tokio::main]
async fn main(){
    let (tx, rx) = channel::<i32>(5);
	let prod_handle = tokio::task::spawn(prod(tx));
    let cons_handle = tokio::task::spawn(cons(rx));
	let _ = tokio::join!(prod_handle, cons_handle);
}