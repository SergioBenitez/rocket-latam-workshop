use super::*;
use rocket::{local::Client, http::{Status, ContentType}};

fn client() -> Client {
    let routes = routes![add_json_task, add_msgpack_task];
    Client::new(rocket::ignite().mount("/", routes)).unwrap()
}

#[test]
fn test_json_task() {
    let client = client();

    let mut res = client.post("/task")
        .header(ContentType::JSON)
        .body(r#"{ "name": "Fruit Loops", "category": "Cereal" }"#)
        .dispatch();

    let expected = Task {
        name: "Fruit Loops".into(),
        category: "Cereal".into()
    };

    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let res = client.post("/task")
        .header(ContentType::JSON)
        .body(r#"{ name = "oops", category = 123 }"#)
        .dispatch();

    assert_eq!(res.status(), Status::BadRequest);

    let res = client.post("/task")
        .header(ContentType::JSON)
        .body(r#"{ "name": "Car Wash", "category": -1 }"#)
        .dispatch();

    assert_eq!(res.status(), Status::UnprocessableEntity);
}

#[test]
fn test_msgpack_task() {
    let client = client();

    let mut res = client.post("/task")
        .header(ContentType::MsgPack)
        .body(vec![0x82, 0xa4, 0x6e, 0x61, 0x6d, 0x65, 0xab, 0x46, 0x72, 0x75,
              0x69, 0x74, 0x20, 0x4c, 0x6f, 0x6f, 0x70, 0x73, 0xa8, 0x63, 0x61,
              0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0xa6, 0x43, 0x65, 0x72, 0x65,
              0x61, 0x6c])
        .dispatch();

    let expected = Task {
        name: "Fruit Loops".into(),
        category: "Cereal".into()
    };

    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some(format!("{:?}", expected)));

    let res = client.post("/task")
        .header(ContentType::MsgPack)
        .body(vec![0x82, 0xa4, 0x6e, 0x61, 0x6d, 0x65, 0xa3, 0x46, 0x6f, 0x6f,
              0xa8, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0xff])
        .dispatch();

    assert_eq!(res.status(), Status::BadRequest);
}

#[test]
fn test_unknown_format() {
    let client = client();

    let res = client.post("/task")
        .header(ContentType::Plain)
        .body("*knock knock*")
        .dispatch();

    assert_eq!(res.status(), Status::NotFound);
}
