#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;

#[cfg(test)] mod tests;

use rocket_contrib::databases::{database, rusqlite};

#[derive(Debug, Serialize)]
pub struct Task {
    id: i32,
    description: String,
    completed: bool
}

fn fetch_tasks(conn: &rusqlite::Connection) -> Option<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")
        .expect("valid SQL");

    let iter = stmt.query_map(&[], |row| {
        Task {
            id: row.get(0),
            description: row.get(1),
            completed: row.get(2),
        }
    }).ok()?;

    iter.collect::<Result<_, _>>().ok()
}

// FIXME: Implement the following routes:
//
//   * (tasks) GET /tasks
//
//     Uses `fetch_tasks` to return a JSON vector of all tasks in the
//     `tasks.sqlite` database.

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![tasks])
}

fn main() {
    rocket().launch();
}
