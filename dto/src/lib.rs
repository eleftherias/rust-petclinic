use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Owner {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub city: String,
    pub telephone: String,
    pub pets: Vec<Pet>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
    pub kind: Type,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Type {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct NewPet {
    pub name: String,
    pub birth_date: NaiveDate,
    pub kind_id: i32,
}
