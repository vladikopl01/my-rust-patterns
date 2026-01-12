pub async fn tokio_select_first() {
    let x = async { 0 };
    let y = async { "hello".to_string() };

    // As soon as one future result is available
    let result = tokio::select! {
        res_a = x => format!("Result from a: {}", res_a),
        res_b = y => format!("Result from b: {}", res_b),
    };

    println!("{}", result);
}
