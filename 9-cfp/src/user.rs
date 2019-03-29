use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use schema::users;
use DbConn;

/// User data retrieved from the database
#[derive(Clone, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub github_id: String,
    pub email: String,
    pub name: String,
    pub is_admin: bool,
}

/// New user data to be inserted into the database
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub github_id: &'a str,
    pub email: &'a str,
    pub name: &'a str,
    pub is_admin: bool,
}

impl User {
    /// Get a user, or create it if it does not exist.
    ///
    /// Existence is determined by the `github_login`.
    /// `email` and `name` are only set for "new" users.
    pub fn get_or_create(
        conn: &DbConn,
        github_login: &str,
        name: &str,
        is_admin: bool,
    ) -> Result<i32, Error> {
        let existing_user = users::table
            .filter(users::github_id.eq(github_login))
            .first(&conn.0)
            .optional()?;

        if let Some(User { id, .. }) = existing_user {
            return Ok(id);
        }

        diesel::insert_into(users::table)
            .values(NewUser {
                github_id: github_login,
                email: "",
                name,
                is_admin,
            })
            .execute(&conn.0)?;

        users::table
            .filter(users::github_id.eq(github_login))
            .select(users::id)
            .first(&conn.0)
    }

    /// Get a given user by `id`. Returns an error when the user does not exist
    pub fn get(conn: &DbConn, id: i32) -> Result<User, Error> {
        users::table.filter(users::id.eq(id)).first(&conn.0)
    }

    /// Return the number of existing users.
    pub fn count(conn: &DbConn) -> Result<i64, Error> {
        users::table.count().get_result(&conn.0)
    }
}
