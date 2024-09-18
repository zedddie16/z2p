use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {

    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//spawn app
fn spawn_app() -> String{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // Launch the server as a background task
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}