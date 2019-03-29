use rocket::outcome::{Outcome, IntoOutcome};
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User {
    pub id: usize,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        unimplemented!("FIXME: Implement me!")
    }
}

#[derive(Debug)]
pub struct Admin(pub User);

// FIXME: Implement `FromRequest` for `Admin`, which authenticates a user as an
// administrator if and only if the user's ID is 0. Otherwise, it forwards.
