#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde;

#[cfg(test)] mod tests;

use rocket::request::{FromParam, FromFormValue};
use rocket::http::RawStr;

#[derive(FromFormValue)]
enum Kind { Html, Json, }

impl<'a> FromParam<'a> for Kind {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        Kind::from_form_value(param)
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    name: &'static str,
    severity: u8,
}

static TASKS: &[Task] = &[
    Task { id: 0, name: "Hi! This is the 0th task.", severity: 100 },
    Task { id: 1, name: "Beep boop. Robot here!", severity: 0 },
    Task { id: 2, name: "RustConf Rocket Workshop", severity: 127 },
    Task { id: 3, name: "Make `TASKS` Slice", severity: 50 },
];

// FIXME: Implement the following routes:
//
//   * (index) GET /
//
//     Redirects to the `html_tasks` route.
//
//   * (json_tasks) GET /tasks (Accept = application/json)
//
//     Renders the list of tasks in a JSON dictionary with a single key:
//     `tasks`. Example:
//
//     {
//         "tasks": [
//             { "id": 0, name: "Hi! This is...", "severity", 100 }
//         ]
//     }
//
//   * (html_tasks) GET /tasks (Accept = text/html)
//
//     Renders the list of tasks using the `tasks.html.tera` template.
//
//   * (one_task) GET /tasks/<id>/<kind>
//
//     Where:
//
//      * <id> - a unsigned integer in range [0, 256) (`u8`)
//      * <kind> - one of `html` or `json`
//
//     If `kind `is `html`, renders the `task.html.tera` template for the task
//     with id `id`. If `kind` is `json`, returns the JSON representation of the
//     task with id `id`. In either case, if `id` doesn't correspond to a known
//     task, returns a 404 page with the text "Unknown task: <id>". If `kind`
//     is neither `html` nor `json`, this route does not get called.
//
//  NOTE: If no 'Accept' is provided to `GET /tasks`, `html_tasks` should run.

fn main() {
    rocket::ignite()
        .mount("/", routes![index, html_tasks, json_tasks, one_task])
        .launch();
}
