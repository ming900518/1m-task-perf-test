use std::{sync::Arc, collections::BTreeMap};
use futures::future::join_all;
use parking_lot::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let target = Arc::new(Mutex::new(BTreeMap::new()));
    let mut tasks = Vec::new();
    for num in 0..1_000_000 {
        let target = target.clone();
        tasks.push(tokio::spawn(async move { target.lock().insert(num, num) }));
    }
    join_all(tasks).await;
    let locked = target.lock();
    println!("{} items pushed into target.", locked.len());
}
