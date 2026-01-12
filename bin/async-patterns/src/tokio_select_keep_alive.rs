use tokio::time::{Duration, sleep};

async fn long_running_io() -> String {
    println!("Starting long-running I/O operation...");
    sleep(Duration::from_secs(10)).await;
    "Input".to_string()
}

async fn keep_alive_api() {
    println!("Keep-alive ping sent");
}

pub async fn tokio_select_keep_alive() {
    let mut io_operation = Box::pin(long_running_io());

    // Keep sending keep-alive pings every 2 seconds until the I/O operation completes
    loop {
        tokio::select! {
            result = &mut io_operation => {
                println!("Operation finished: {}", result);
                break;
            }
            _ = sleep(Duration::from_secs(2)) => {
                keep_alive_api().await;
            }
        }
    }
}
