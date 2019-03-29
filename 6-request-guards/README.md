# Data

## Learning Objectives

  1. Request Guards and the `FromRequest` Trait
  2. Cookies via the `Cookies` request guard
  3. `NamedFile` Responder

## Tasks
  
  1. Implement the `User` and `Admin` request guards in `src/roles.rs`.
  2. Fix all `FIXME`s so that `cargo test` passes.

## Hints

  * To log a user in, set a private cookie with their ID.

  * To authenticate a user or admin, check for the private cookie you set on
    log-in. If authentication fails, forward.

  * Use the `User` `FromRequest` implementation when implementing the `Admin`
    request guard.

  * Use the `NamedFile` responder to implement `login_page`. Do not `unwrap`.

  * To a log a user out, delete the associated private cookie.

## Relevant Documentation

  * [Request Guards](https://rocket.rs/v0.4/guide/requests/#request-guards)
  * [Cookies](https://rocket.rs/v0.4/guide/requests/#cookies)
  * [`FromRequest`](https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html)
  * [`Cookies`](https://api.rocket.rs/v0.4/rocket/http/enum.Cookies.html)
  * [`NamedFile`](https://api.rocket.rs/v0.4/rocket/response/struct.NamedFile.html)
