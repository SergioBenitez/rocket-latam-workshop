use super::*;
use rocket::{local::Client, http::{Status, Accept, ContentType}};
use rocket_contrib::templates::Template;

fn client() -> Client {
    let routes = routes![index, html_tasks, json_tasks, one_task];
    let rocket = rocket::ignite().mount("/", routes).attach(Template::fairing());
    Client::new(rocket).unwrap()
}

#[test]
fn tests_index() {
    let client = client();

    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::SeeOther);
    assert_eq!(res.headers().get_one("Location"), Some("/tasks"))
}

#[test]
fn tests_json_tasks() {
    let client = client();

    let mut res = client.get("/tasks").header(Accept::JSON).dispatch();

    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::JSON));
    assert!(body.contains(r#""tasks"#));
    assert!(body.contains("Hi! This is the 0th task."));
    assert!(body.contains("Beep boop."));
    assert!(body.contains("RustConf"));
    assert!(body.contains("Slice"));
}

#[test]
fn tests_html_tasks() {
    let client = client();

    let mut res = client.get("/tasks").header(Accept::HTML).dispatch();

    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::HTML));
    assert!(body.contains("<html>"));
    assert!(body.contains("</html>"));
    assert!(body.contains("<b>(0) Hi! This is the 0th task."));
    assert!(body.contains("Slice [50]"));
}

#[test]
fn tests_default_to_html_tasks() {
    let client = client();
    let mut res = client.get("/tasks").dispatch();
    res.body_string().expect("missing body");
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::HTML));
}

#[test]
fn test_one_task() {
    let client = client();

    let mut res = client.get("/tasks/1/html").dispatch();
    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::HTML));
    assert!(body.contains("<html>"));
    assert!(body.contains("ID: 1"));

    let mut res = client.get("/tasks/2/json").dispatch();
    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::JSON));
    assert!(body.contains("RustConf"));

    let mut res = client.get("/tasks/20/html").dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert_eq!(res.content_type(), Some(ContentType::Plain));
    assert_eq!(res.body_string(), Some("Unknown task: 20".into()));

    let mut res = client.get("/tasks/20/json").dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert_eq!(res.content_type(), Some(ContentType::Plain));
    assert_eq!(res.body_string(), Some("Unknown task: 20".into()));

    let mut res = client.get("/tasks/42/json").dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert_eq!(res.content_type(), Some(ContentType::Plain));
    assert_eq!(res.body_string(), Some("Unknown task: 42".into()));

    let res = client.get("/tasks/1/oops").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}
