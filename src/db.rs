use sqlx::{SqlitePool, Error};

use crate::models::{Manga, Chapter}; // Replace `Manga` with your actual model name

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Database { pool }
    }

    pub async fn create(&self, model: &Manga) -> Result<(), Error> {
        // Implement your create logic here
        unimplemented!()
    }

    pub async fn read(&self, id: i32) -> Result<Option<Manga>, Error> {
        // Implement your read logic here
        unimplemented!()
    }

    pub async fn update(&self, model: &Manga) -> Result<(), Error> {
        // Implement your update logic here
        unimplemented!()
    }

    pub async fn delete(&self, id: i32) -> Result<(), Error> {
        // Implement your delete logic here
        unimplemented!()
    }
}
