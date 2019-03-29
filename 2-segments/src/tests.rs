use super::*;
use rocket::{local::Client, http::Status};

fn client() -> Client {
    let routes = routes![inner, outer, echo];
    Client::new(rocket::ignite().mount("/", routes)).unwrap()
}

#[test]
fn test_outer() {
    let client = client();

    let mut res = client.get("/outer/foo/bar").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("bar".into()));

    let mut res = client.get("/outer/foo/bar/baz").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("bar".into()));

    let mut res = client.get("/outer/test/baz").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("baz".into()));

    let mut res = client.get("/outer/test/ploop").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("ploop".into()));

    let mut res = client.get("/outer/test").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("Expected >= 2 segments, found 1.".into()));
}

#[test]
fn test_inner() {
    let client = client();

    let mut res = client.get("/inner/1/2").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("2".into()));

    let mut res = client.get("/inner/0/1").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("1".into()));

    let mut res = client.get("/inner/foo/boo").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("boo".into()));

    let mut res = client.get("/inner/woo/wop/da/bam").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("wop".into()));
}

#[test]
fn test_echo() {
    let client = client();

    let mut res = client.get("/outer").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("outer".into()));

    let mut res = client.get("/inner").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("inner".into()));

    let mut res = client.get("/inner/foo").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("inner/foo".into()));

    let mut res = client.get("/inner/foo/").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some("inner/foo".into()));
}

#[test]
fn test_no_index() {
    let client = client();

    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}
