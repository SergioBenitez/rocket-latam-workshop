# Forms

## Learning Objectives

  1. `Form` type
  2. `FromForm` trait and derive
  3. Field renaming
  4. Strict forms
  5. Lenient forms
  6. Custom field validation

## Tasks
  
  1. Fix all `FIXME`s so that `cargo test` passes.

## Hints

  1. Use form field renaming (`#[form(field = "name")]`) in `post_strict_task`.
  2. Use the `FromFormValue` derive to implement `post_strict_task`.

## Relevant Documentation

  * [Forms](https://rocket.rs/v0.4/guide/requests/#forms)
  * [`Form`](https://api.rocket.rs/v0.4/rocket/request/struct.Form.html)
  * [`FromForm`](https://api.rocket.rs/v0.4/rocket/request/trait.FromForm.html)
  * [`FromForm` Derive](https://api.rocket.rs/v0.4/rocket_codegen/derive.FromForm.html)
  * [`FromFormValue`](https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html)
  * [`FromFormValue` Derive](https://api.rocket.rs/v0.4/rocket_codegen/derive.FromFormValue.html)
