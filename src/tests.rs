#[tokio::test]
async fn test_create_todo_handler() {

    let body = serde_json::json!({
        "title": "My note",
        "content": "This is a note",
    });

    let body_str = serde_json::to_string(&body).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8000/api/todos")
        .timeout(tokio::time::Duration::from_secs(5))
        .header("content-type", "application/json")
        .body(body_str)
        .send()
        .await
        .unwrap();

    
    assert!(res.status().is_success());

    let res_str = res.text().await.unwrap();
    let res_json: serde_json::Value = serde_json::from_str(&res_str).unwrap();

    println!("Created note:{}", serde_json::to_string_pretty(&res_json).unwrap());

}