# Responders

## Learning Objectives

  1. `Responder` Trait
  2. `Redirect` responder
  3. Typed URIs via `uri!`
  4. `json!` macro and `JsonValue`
  5. Templates via `rocket_contrib::templates::Template`
  6. Fairing Attachment
  7. Custom `Responder` and `#[derive(Responder)]`

## Tasks
  
  1. Depend on `rocket_contrib` with the appropriate features enabled.
  2. Read and understand the templates in `templates/`.
  3. Fix all `FIXME`s so that `cargo test` passes.
  4. Modify a template while the application is running, observe auto-reload.

## Hints

  * Use `Redirect` and `uri!` to implement `index`.

  * Don't forget to attach `Template::fairing()`!

  * To implement `html_tasks`, create a structure with a `tasks` field that
    implements `Serialize`. Pass an instance of that structure to
    `Template::render`.

  * To implement `one_task`, create an `enum` with three variants and derive
    `Responder` on the enum. Use the attribute `#[response(status = N)]`
    attribute on one of the variants.

  * Use `Kind` when implementing `one_task`.

  * Our solution adds a total of 44 lines.

## Relevant Documentation

  * [Responses](https://rocket.rs/v0.4/guide/responses/)
  * [`Redirect`](https://api.rocket.rs/v0.4/rocket/response/struct.Redirect.html)
  * [`uri!`](https://api.rocket.rs/v0.4/rocket_codegen/macro.uri.html)
  * [`json!`](https://api.rocket.rs/v0.4/rocket_contrib/macro.json.html)
  * [`JsonValue`](https://api.rocket.rs/v0.4/rocket_contrib/json/struct.JsonValue.html)
  * [`Template`](https://api.rocket.rs/v0.4/rocket_contrib/templates/struct.Template.html)
  * [Attaching Fairings](https://rocket.rs/v0.4/guide/fairings/#attaching)
  * [`Responder` Derive](https://api.rocket.rs/v0.4/rocket_codegen/derive.Responder.html)
  * [Tera](https://tera.netlify.com/docs/templates/#templates)
