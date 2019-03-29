#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

// FIXME: Implement the following routes:
//
//   * (post_task) POST /task
//
//     Accepts a web form in the body with _exactly_ two fields:
//
//         `name` - any arbitrary string
//         `category` - any arbitrary string
//
//      Responds with a `Debug` string of the corresponding Rust structure.
//      Requests with unknown or missing form fields are rejected with a 422
//      error.
//
//   * (post_lenient_task) POST /lenient_task
//
//     Accepts a web form in the body with at least two fields:
//
//         `name` - any arbitrary string
//         `category` - any arbitrary string
//
//     The route is _lenient_; any additional fields in the form submission are
//     simply discarded. Requests with missing form fields are rejected with a
//     422 error. Responds with a `Debug` string of the corresponding Rust
//     structure.
//
//   * (post_strict_task) POST /strict_task
//
//     Accepts a web form in the body with exactly two fields:
//
//         `name` - a non-empty string of length <= 128 bytes
//         `type` - one of "leisure", "business", or "critical"
//
//     Requests with invalid form values are rejected with a 422 error. Responds
//     with a `Debug` string of the corresponding Rust structure.

#[derive(Debug)]
struct Task {
    name: String,
    category: String,
}

#[derive(Debug)]
struct Name(String);

#[derive(Debug)]
enum Kind {
    // FIXME: Add variants.
}

#[derive(Debug)]
struct StrictTask {
    name: Name,
    kind: Kind,
}

fn main() {
    rocket::ignite()
        .mount("/", routes![post_task, post_lenient_task, post_strict_task])
        .launch();
}
