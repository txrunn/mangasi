#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct Manga {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub description: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub id: u64,
    pub manga_id: u64,
    pub title: String,
    pub chapter_number: f64,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub user_id: u64,
    pub manga_id: u64,
    pub chapter_id: u64,
    pub progress: f64,
    // Add other fields as needed
}
