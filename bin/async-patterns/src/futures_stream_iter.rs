use futures::StreamExt;
use rand::Rng;
use tokio::time::{Duration, sleep};

async fn mock_api_call(id: i32) -> i32 {
    let delay = rand::thread_rng().gen_range(100..=5000);
    println!("Starting API call for id: {} (delay: {}ms)", id, delay);
    sleep(Duration::from_millis(delay)).await;
    println!("Completed API call for id: {}", id);
    id
}

pub async fn futures_stream_iter() {
    let prompts = vec![1, 2, 3, 4, 5];

    let stream: Vec<_> = futures::stream::iter(prompts)
        .map(|prompt| mock_api_call(prompt))
        .buffer_unordered(2)
        .collect()
        .await;

    for response in stream {
        println!("Response: {}", response);
    }
}
