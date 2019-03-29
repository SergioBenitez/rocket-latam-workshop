use super::*;
use rocket::{local::Client, http::Status};

#[test]
fn hello_world() {
    let rocket = rocket::ignite().mount("/", routes![index]);
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

// FIXME: Write a test that ensures the following:
//  * A HEAD request to `/` responds successfully.
//  * The response has an empty body.
