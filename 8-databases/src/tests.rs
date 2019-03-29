use super::*;
use rocket::{local::Client, http::{Status, ContentType}};

#[test]
fn test_tasks() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client.get("/tasks").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::JSON));

    let body = res.body_string().unwrap();
    assert!(body.contains("Start RustConf Workshop"));
    assert!(body.contains("Finish RustConf Workshop"));
    assert!(body.contains("Release Rocket v0.5"));
}
