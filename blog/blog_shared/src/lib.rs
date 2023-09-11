use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    content: String,
}

impl Post {
    pub fn new(title: String, content: String) -> Post {
        Post { title, content }
    }
}
