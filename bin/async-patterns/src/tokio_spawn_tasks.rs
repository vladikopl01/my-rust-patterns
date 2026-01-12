pub async fn tokio_spawn_tasks() {
    let x = tokio::spawn(async { 0 });
    let y = tokio::spawn(async { "hello".to_string() });

    // Wait for both tasks to complete
    let (x_res, y_res) = tokio::join!(x, y);

    println!("Results: {:?}, {:?}", x_res.unwrap(), y_res.unwrap());
}
