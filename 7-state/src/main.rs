#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

struct Token(i64);

// FIXME: Implement the following routes:
//
//   * (token) GET /token
//
//     Returns the value of the `token` configuration parameter. The value
//     should be stored in managed state on launch. If there is no such
//     configuration value or the value isn't a valid `i64`, launch should be
//     aborted.

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![token])
}

fn main() {
    rocket().launch();
}
