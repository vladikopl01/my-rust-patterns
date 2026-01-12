#![allow(dead_code)]

mod futures_stream_iter;
mod lazylock_config_vars;
mod tokio_select_first;
mod tokio_select_keep_alive;
mod tokio_spawn_tasks;

#[tokio::main]
async fn main() {
    // futures_stream_iter::futures_stream_iter().await;
    // tokio_select_first::tokio_select_first().await;
    // tokio_select_keep_alive::tokio_select_keep_alive().await;
    // tokio_spawn_tasks::tokio_spawn_tasks().await;
}
