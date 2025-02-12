use mongodb::bson::oid::ObjectId;
use ring::digest;
use serde::{Deserialize, Serialize};

use crate::password::Password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub pass: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub pass: String,
}

impl From<(CreateUser, &Password)> for User {
    fn from((user, hasher): (CreateUser, &Password)) -> Self {
        let pass = hasher.generate(user.email.as_bytes(), user.pass.as_bytes());
        Self {
            id: ObjectId::new(),
            name: user.name,
            email: user.email,
            pass: pass.to_vec(),
        }
    }
}
