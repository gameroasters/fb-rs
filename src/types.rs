use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: Option<String>,
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Picture {
    pub height: i32,
    pub is_silhouette: bool,
    pub url: String,
    pub width: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFriends {
    pub data: Option<Vec<User>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResponsePicture {
    pub data: Option<Picture>,
}
