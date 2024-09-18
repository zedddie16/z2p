#[tokio::test]
async fn health_check_works() {
    //spawning app
    spawn_app();
    //spawning client
    let client = reqwest::Client::new();
    //test request
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");
    //validation
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//spawn app
fn spawn_app() {
    // Launch the server as a background task
    let server = zero2prod::run().expect("Failed to bind address");
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);
}