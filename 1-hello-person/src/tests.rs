use super::*;
use rocket::{local::Client, http::Status};

fn client() -> Client {
    let routes = routes![simple_hello, good_aged_hello, bad_aged_hello];
    Client::new(rocket::ignite().mount("/", routes)).unwrap()
}

#[test]
fn test_simple_hello() {
    let client = client();

    let mut res = client.get("/Michael").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("Hello, Michael!".into()));

    let mut res = client.get("/Michael%20Smith").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("Hello, Michael Smith!".into()));
}

#[test]
fn test_good_aged_hello() {
    let client = client();

    let mut res = client.get("/Mike/20").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("Hello, 20 year old Mike.".into()));
}

#[test]
fn test_bad_aged_hello() {
    let client = client();

    let mut res = client.get("/Mike/3940").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("'3940' is a funky age, Mike.".into()));
}

#[test]
fn test_real_bad_aged_hello() {
    let client = client();

    let res = client.get("/Mike/what!").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}
