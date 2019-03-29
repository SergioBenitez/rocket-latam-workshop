#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

// FIXME: Declare the following routes:
//
//   * `simple_hello`
//
//      GET '/<name>' => "Hello, <name>!"
//
//   * `good_aged_hello`
//
//      GET '/<name>/<age>' => "Hello, <age> year old <name>."
//
//      where 0 < age <= 122
//
//   * `bad_aged_hello`
//
//      GET '/<name>/<age>' => "'<age>' is a funky age, <name>."
//
//      where 0 < age <= usize::max_value()
//

fn main() {
    rocket::ignite()
        .mount("/", routes![simple_hello, good_aged_hello, bad_aged_hello])
        .launch();
}
