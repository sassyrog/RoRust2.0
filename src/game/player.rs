pub struct Player {
    pub id: String,
    pub username: String,
}

impl Player {
    pub fn new(id: String, username: String) -> Self {
        Player { id, username }
    }
}
