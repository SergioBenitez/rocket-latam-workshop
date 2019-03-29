# Segments

## Learning Objectives

  1. `FromSegments` Trait
  2. Custom `Segments` validator
  3. Collisions and Rankings
  4. In-band guard error catching

## Tasks
  
  1. Fix all `FIXME`s so that `cargo test` passes.

## Hints

  * `Result<T, T::Error>` implements `FromSegments` when `T` implements
    `FromSegments`
  * Use `path.display().to_string()` to convert a `PathBuf` to a `String`.
  * You may find the code snippet `segments.next().ok_or(n)?` helpful when
    implementing `FromSegments`.

## Relevant Documentation

  * [Multiple Segments](https://rocket.rs/v0.4/guide/requests/#multiple-segments)
  * [`FromSegments`](https://api.rocket.rs/v0.4/rocket/request/trait.FromSegments.html)
  * [Forwarding](https://rocket.rs/v0.4/guide/requests/#forwarding)
