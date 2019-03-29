#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
mod roles;

use roles::{Admin, User};

// FIXME: Implement the following routes:
//
//   * (admin_index) GET /
//
//     Responds only if an `Admin` user is logged in. Returns the text:
//
//        "Hello, admin <id>!"
//
//     where <id> is the admin's user ID.
//
//   * (user_index) GET /
//
//     Responds only if a regular `User` user is logged in. Returns the text:
//
//        "Hello, user <id>!"
//
//     where <id> is the user's user ID. NOTE: The `admin_index` route takes
//     precedence over this route.
//
//   * (index) GET /
//
//     Redirects to `login_page`. NOTE: The `admin_index` and `user_index`
//     routes take precedence over this route.
//
//   * (login_page) GET /login
//
//     Responds with the contents of `static/login.html`.
//
//   * (login_submit) POST /login
//
//     Accepts a form with the following fields:
//
//       * `username` - an arbitrary string
//       * `password` - an arbitrary string
//
//     If the username is "admin" and the password is "password", logs in a user
//     with an id of `0` and redirects to `index`. If the username is "bob" and
//     the password is "123456", logs in a user with an id of `1` and redirects
//     to `index`. If the username and password are anything else, redirects to
//     `login_page`.
//
//   * (logout) GET /logout
//
//     Logs the current user out, if any, then redirects to `index`.
//
//     NOTE: This should really be a `POST`; we use `GET` purely for
//     convenience. In reality, we'd use Rocket's _method rewriting_ mechanism
//     to issue a `POST`.


fn main() {
    rocket::ignite()
        .mount("/", routes![admin_index, user_index, index])
        .mount("/", routes![login_page, login_submit, logout])
        .launch();
}
