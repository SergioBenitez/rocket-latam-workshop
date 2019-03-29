# Hello, person!

## Learning Objectives

  1. Typed Parameters with existing `FromParam` implementations
  2. Forwarding and Ranking
  3. Custom parameter validation with `FromParam`

## Tasks
  
  1. Fix all `FIXME`s so that `cargo test` passes.

## Hints

  * You will need to implement `FromParam` for _one_ custom type of your choice.
    Your implementation should reuse existing `FromParam` implementations.

## Relevant Documentation

  * [Dynamic Segments](https://rocket.rs/v0.4/guide/requests/#dynamic-paths)
  * [Forwarding](https://rocket.rs/v0.4/guide/requests/#forwarding)
  * [`FromParam`](https://api.rocket.rs/v0.4/rocket/request/trait.FromParam.html)
