use super::*;
use rocket::{local::{Client, LocalResponse}, http::{Status, ContentType}};

fn client() -> Client {
    let routes = routes![post_task, post_lenient_task, post_strict_task];
    Client::new(rocket::ignite().mount("/", routes)).unwrap()
}

fn dispatch<'c>(client: &'c Client, uri: &'c str, form: &str) -> LocalResponse<'c> {
    client.post(uri)
        .header(ContentType::Form)
        .body(form)
        .dispatch()
}

#[test]
fn test_simple_form() {
    let client = client();

    let mut res = dispatch(&client, "/task", "name=Hello,%20world&category=intro");
    let expected = Task {
        name: "Hello, world".into(),
        category: "intro".into()
    };

    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let mut res = dispatch(&client, "/task", "name=&category=some");
    let expected = Task {
        name: "".into(),
        category: "some".into()
    };

    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let res = dispatch(&client, "/task", "name=hi&category=hey&whoah=boop");
    assert_eq!(res.status(), Status::UnprocessableEntity);

    let res = dispatch(&client, "/task", "name=hi");
    assert_eq!(res.status(), Status::UnprocessableEntity);
}

#[test]
fn test_lenient_form() {
    let client = client();

    let mut res = dispatch(&client, "/lenient_task",
                           "name=Hello,%20world&category=intro");

    let expected = Task {
        name: "Hello, world".into(),
        category: "intro".into()
    };

    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let mut res = dispatch(&client, "/lenient_task",
                           "name=hi&category=hey&whoah=boop");

    let expected = Task {
        name: "hi".into(),
        category: "hey".into()
    };

    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let res = dispatch(&client, "/lenient_task", "name=hi");
    assert_eq!(res.status(), Status::UnprocessableEntity);
}

#[test]
fn test_strict_form() {
    let client = client();

    // Bad `type`.
    let res = dispatch(&client, "/strict_task", "name=Hello,%20world&type=intro");
    assert_eq!(res.status(), Status::UnprocessableEntity);

    // Extra field, `whoah`.
    let res = dispatch(&client, "/strict_task", "name=hi&type=leisure&whoah=boop");
    assert_eq!(res.status(), Status::UnprocessableEntity);

    // Bad `name`.
    let res = dispatch(&client, "/strict_task", "name=&type=leisure");
    assert_eq!(res.status(), Status::UnprocessableEntity);

    let expected = StrictTask {
        name: Name("hi".into()),
        kind: Kind::Leisure,
    };

    let mut res = dispatch(&client, "/strict_task", "name=hi&type=leisure");
    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let mut res = dispatch(&client, "/strict_task", "name=hi&type=LEISURE");
    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let mut res = dispatch(&client, "/strict_task", "name=hi&type=lEIsuRE");
    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let mut res = dispatch(&client, "/strict_task", "name=woo%20wop&type=BUsiNESS");
    let expected = StrictTask {
        name: Name("woo wop".into()),
        kind: Kind::Business,
    };

    assert_eq!(res.status(), Status::Ok);;
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));
}
