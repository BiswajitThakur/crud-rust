use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: ObjectId::new(),
            name,
            email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        Self {
            id: ObjectId::new(),
            name: value.name,
            email: value.email,
        }
    }
}
