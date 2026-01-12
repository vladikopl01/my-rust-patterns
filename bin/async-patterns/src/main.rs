#![allow(dead_code)]

mod futures_stream_iter;
mod tokio_select_first;
mod tokio_select_keep_alive;

#[tokio::main]
async fn main() {
    futures_stream_iter::futures_stream_iter().await;
    tokio_select_first::tokio_select_first().await;
    tokio_select_keep_alive::tokio_select_keep_alive().await;
}
