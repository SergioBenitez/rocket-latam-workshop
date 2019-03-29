use std::io;

use diesel;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;
use rocket::FromFormValue;

use {Admin, DbConn};
use schema::talks;
use user::User;

/// Talk status: Pending, Approved, or Rejected.
/// Derives lots of traits so it can be used via serde (as a string)
/// and in diesel (as an integer)
#[repr(i32)]
#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression, Serialize, FromFormValue)]
#[sql_type = "Integer"]
pub enum TalkStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
}

impl From<i32> for TalkStatus {
    fn from(i: i32) -> TalkStatus {
        match i {
            0 => TalkStatus::Pending,
            1 => TalkStatus::Approved,
            2 => TalkStatus::Rejected,
            _ => panic!("Invalid TalkStatus value: {}", i),
        }
    }
}

impl<DB: Backend> FromSql<Integer, DB> for TalkStatus
where
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(i32::from_sql(bytes)?.into())
    }
}

impl<DB: Backend> ToSql<Integer, DB> for TalkStatus
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        i32::to_sql(&(*self as i32), out)
    }
}

/// Talk data retrieved from the database
#[derive(Queryable, Serialize)]
pub struct Talk {
    id: i32,
    presenter: i32,
    title: String,
    status: TalkStatus,
    description: String,
}

/// Talk data to be inserted into the database
#[derive(Insertable)]
#[table_name = "talks"]
struct NewTalk<'a> {
    pub presenter: i32,
    pub title: &'a str,
    pub status: TalkStatus,
    pub description: &'a str,
}

impl Talk {
    /// Retrieve all talks, regardless of presenter
    pub fn get_all(conn: &DbConn, _admin: &Admin) -> Result<Vec<Talk>, Error> {
        talks::table.load(&conn.0)
    }

    /// Retrieve all talks that `user` is presenting
    pub fn get_for_user(conn: &DbConn, user: &User) -> Result<Vec<Talk>, Error> {
        talks::table
            .filter(talks::presenter.eq(user.id))
            .load(&conn.0)
    }

    /// Retrieves a single talk by `id`. The user must either be the presenter
    /// of the given talk, or an admin.
    pub fn get_one(conn: &DbConn, user: &User, id: i32) -> Result<Option<Talk>, Error> {
        talks::table
            .filter(talks::id.eq(id).and(talks::presenter.eq(user.id).or(user.is_admin)))
            .get_result(&conn.0)
            .optional()
    }

    /// Inserts a new talk. The talk will be `Pending` initially and presented by `user`.
    pub fn new(conn: &DbConn, user: &User, title: &str, description: &str) -> Result<(), Error> {
        diesel::insert_into(talks::table)
            .values(NewTalk {
                presenter: user.id,
                title,
                status: TalkStatus::Pending,
                description,
            })
            .execute(&conn.0)?;
        Ok(())
    }

    /// Sets the status of the talk with the given `id` to `new_status`
    pub fn set_status(
        conn: &DbConn,
        _admin: &Admin,
        id: i32,
        new_status: TalkStatus
    ) -> Result<(), Error> {
        diesel::update(talks::table.filter(talks::id.eq(id)))
            .set(talks::status.eq(new_status as i32))
            .execute(&conn.0)?;

        Ok(())
    }
}
