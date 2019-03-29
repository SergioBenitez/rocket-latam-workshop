use super::*;
use rocket::{local::Client, http::{Status, ContentType}};

#[test]
fn everything() {
    let rocket = rocket::ignite()
        .mount("/", routes![admin_index, user_index, index])
        .mount("/", routes![login_page, login_submit, logout]);

    let client = Client::new(rocket).unwrap();

    // Redirect to login when logged out.
    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/login"));

    // Log in as regular user.
    let res = client.post("/login")
        .header(ContentType::Form)
        .body("username=bob&password=123456")
        .dispatch();

    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/"));

    // Should now be logged in.
    let mut res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.body_string().unwrap().contains("user"));

    // Logout, and check that it worked.
    client.get("/logout").dispatch();
    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/login"));

    // Log in as admin.
    let res = client.post("/login")
        .header(ContentType::Form)
        .body("username=admin&password=password")
        .dispatch();

    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/"));

    // Should now be logged in.
    let mut res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.body_string().unwrap().contains("admin"));

    // Logout, and check that it worked.
    client.get("/logout").dispatch();
    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/login"));

    // Check the login HTML.
    let mut res = client.get("/login").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::HTML));
    assert!(res.body_string().unwrap().contains("Please login to continue."));
}
