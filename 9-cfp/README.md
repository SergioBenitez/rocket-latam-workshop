# Call for Papers Demo

## Learning Objectives

  1. Authenticating users with GitHub OAuth
  2. Using authentication guards to restrict database access
  3. Author a complete application using Rocket

## Tasks

  1. Review the `auth.rs` module. Pay particular attention to the routes,
     redirects, and cookies that are provided.
  2. Review `talk.rs` and `user.rs`. Notice which methods take `Admin` or `User`
     parameters and why they do so.
  3. Fix all `FIXME`s in `src/main.rs`.
  4. Implement the missing templates.

## Hints

  * Use the `User` and `Admin` request guards to verify the user's authorization
    level for routes. The methods in `Talk` and `User` enforce that you
    authorize users appropriately. We call this form of type-based security
    _request-guard transperency_.

  * Note that `set_status` is a `PUT` endpoint. To call it, you'll need to
    create a form that takes advantage of Rocket's method reinterpretation
    feature.
  
## Relevant Documentation

  * [Reinterpreting Methods](https://rocket.rs/v0.4/guide/requests/#reinterpreting)
