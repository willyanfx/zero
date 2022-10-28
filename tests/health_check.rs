use std::net::TcpListener;
use zero::run;

// launch app in the bg
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // retrive the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let server = zero::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    //  return the app address
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();
    // perform HTTP requests against our app
    let client = reqwest::Client::new();
    // act
    // Use the returned application address
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
