async fn worker(id: usize, receiver: flume::Receiver<()>) {
    loop {
        if receiver.try_recv().is_ok() {
            println!("Worker {} is shutting down", id);
            break;
        }

        println!("Worker {} is working...", id);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let (shutdown_tx, shutdown_rx) = flume::unbounded();
    let num_workers = 5;

    for id in 0..num_workers {
        let rx = shutdown_rx.clone();
        tokio::spawn(worker(id, rx));
    }

    println!("Program is running. Press Ctrl+C to stop workers.");

    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C caught, sending shutdown signal...");

    for _ in 0..num_workers {
        shutdown_tx.send(()).unwrap();
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("All workers have been shut down. Exiting program.");
}
