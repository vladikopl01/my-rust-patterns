mod futures_stream_iter;

#[tokio::main]
async fn main() {
    futures_stream_iter::futures_stream_iter().await;
}
