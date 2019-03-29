use super::*;
use rocket::local::Client;

#[test]
fn token() {
    // Ensure the token is '123', which is what we have in `Rocket.toml`.
    let client = Client::new(rocket()).unwrap();
    let mut res = client.get("/token").dispatch();
    assert_eq!(res.body_string(), Some("123".into()));

    // Now set an environment variable that overrides that.
    ::std::env::set_var("ROCKET_TOKEN", "456");
    let client = Client::new(rocket()).unwrap();
    let mut res = client.get("/token").dispatch();
    assert_eq!(res.body_string(), Some("456".into()));

    // And now ensure that invalid tokens result in a launch failure.
    ::std::env::set_var("ROCKET_TOKEN", "whoops!");
    assert!(Client::new(rocket()).map_err(|e| eprintln!("{}", e)).is_err());
}
