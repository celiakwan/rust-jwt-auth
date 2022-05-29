use crate::db;
use crate::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize::Output;
use diesel::types::{FromSql, IsNull, ToSql, VarChar};
use diesel::{deserialize, serialize, QueryDsl};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::{from_utf8, FromStr};
use strum_macros::{Display, EnumString};

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: Role,
    pub logged_in: bool,
}

#[derive(Deserialize, Serialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: Role,
}

impl User {
    pub fn create(new_user: NewUser) -> QueryResult<usize> {
        let encrypted_user = NewUser {
            password: hash(new_user.password, DEFAULT_COST).unwrap(),
            ..new_user
        };
        let conn = db::connection();
        Ok(diesel::insert_into(users::table)
            .values(&encrypted_user)
            .execute(&conn)?)
    }

    pub fn get_by_username_and_password(username: &str, password: &str) -> Option<User> {
        let conn = db::connection();
        let result = users::table
            .filter(users::username.eq(username))
            .get_result::<User>(&conn);
        match result {
            Ok(user) => match verify(&password, &user.password) {
                Ok(true) => Some(user),
                _ => None,
            },
            Err(_) => None,
        }
    }

    pub fn get_by_username(username: &str) -> Option<User> {
        let conn = db::connection();
        let result = users::table
            .filter(users::username.eq(username))
            .get_result::<User>(&conn);
        match result {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub fn update_logged_in(username: &str, logged_in: bool) -> QueryResult<usize> {
        let conn = db::connection();
        Ok(
            diesel::update(users::table.filter(users::username.eq(username)))
                .set(users::logged_in.eq(logged_in))
                .execute(&conn)?,
        )
    }
}

#[derive(
    Deserialize, Serialize, AsExpression, Display, EnumString, FromSqlRow, PartialEq, Eq, Debug,
)]
#[sql_type = "VarChar"]
pub enum Role {
    Buyer,
    Seller,
}

impl ToSql<VarChar, Pg> for Role {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<VarChar, Pg> for Role {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            Some(b) => match Role::from_str(from_utf8(b)?) {
                Ok(role) => Ok(role),
                Err(e) => Err(format!("Unrecognized variant: {}", e).into()),
            },
            None => Err("Unable to deserialize empty bytes".into()),
        }
    }
}
