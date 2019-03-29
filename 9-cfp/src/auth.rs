use std;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest;
use rocket::config::{self, Config};
use rocket::fairing::{AdHoc, Fairing};
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::request::{State, Form};
use rocket::response::Redirect;

use DbConn;
use user::User;

/// Generates a new random state string.
fn generate_state() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(20).collect()
}

/// Contains OAuth server configuration, including the client ID and secret.
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl OAuthConfig {
    pub fn from_config(config: &Config) -> config::Result<OAuthConfig> {
        let client_id = config.get_string("gh_client_id")?;
        let client_secret = config.get_string("gh_client_secret")?;
        Ok(OAuthConfig { client_id, client_secret })
    }
}

/// A struct containing the authentication callback query parameters.
#[derive(FromForm)]
struct AuthParams {
    code: String,
    state: String,
}

/// The OAuth token exchange response, containing the API access token.
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
}

/// User information to be retrieved from the GitHub API.
#[derive(Deserialize)]
struct UserInfo {
    login: String,
    #[serde(default)]
    name: String,
}

/// Generates a state string and redirects the user to the GitHub OAuth login
/// flow, which will later resume at the auth_callback.
#[get("/login")]
fn login(oauth_config: State<OAuthConfig>, mut cookies: Cookies) -> Redirect {
    let state = generate_state();
    cookies.add_private(
        Cookie::build("oauth_state", state.clone())
            .same_site(SameSite::Lax)
            .finish()
    );
    Redirect::to(format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=read%3Auser&state={}",
        oauth_config.client_id, state
    ))
}


/// The OAuth authentication callback. The GitHub OAuth flow will redirect here
/// after the user has logged into GitHub and granted the necessary permissions.
#[get("/callback/github?<params..>")]
fn auth_callback(
    conn: DbConn,
    params: Form<AuthParams>,
    mut cookies: Cookies,
    oauth_config: State<OAuthConfig>,
) -> Result<Redirect, Box<std::error::Error>> {
    // Verify that the given state is the same one in the cookie.
    match cookies.get_private("oauth_state") {
        Some(ref cookie) if cookie.value() == params.state => {
            cookies.remove(cookie.clone());
        },
        _ => {
            return Err("invalid state".into());
        }
    }

    // Set up a request to exchange for a token.
    let client = reqwest::Client::new();
    let uri = format!("https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}&state={}",
        oauth_config.client_id, oauth_config.client_secret, params.code, params.state);

    // Make the request to retrieve the token.
    let token_response: TokenResponse = client
        .get(&uri)
        .header(reqwest::header::Accept::json())
        .send()?
        .json()?;
    let access_token = token_response.access_token;

    // Use the token to retrieve the user's GitHub account information.
    let user_info: UserInfo = client
        .get("https://api.github.com/user")
        .header(reqwest::header::Authorization(format!("token {}", access_token)))
        .header(reqwest::header::Accept(vec![reqwest::header::qitem(
            "application/vnd.github.v3+json".parse().expect("mime type"),
        )]))
        .send()?
        .json()?;

    // If this is the first user to be created, make it an admin automatically.
    let is_admin = User::count(&conn)? < 1;

    // Get or create the user in the database.
    let id = User::get_or_create(&conn, &user_info.login, &user_info.name, is_admin)?;

    // Set a private cookie with the user ID, and redirect to the dashboard.
    cookies.add_private(
        Cookie::build("user", id.to_string())
            .same_site(SameSite::Lax)
            .finish()
    );
    Ok(Redirect::to("/"))
}

/// Returns a fairing that sets up OAuth for the application:
///
/// * Reads the OAuth configuration.
/// * Mounts the login route and the authentication callback.
pub fn fairing() -> impl Fairing {
    AdHoc::on_attach("GitHub OAuth", |rocket| {
        match OAuthConfig::from_config(rocket.config()) {
            Ok(config) => Ok(rocket
                .manage(config)
                .mount("/", routes![login, auth_callback])
            ),
            Err(_) => Err(rocket),
        }
    })
}
