use sea_orm::{Database, DatabaseConnection};
use domain::form::model::Form;

pub async fn connect(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url).await.expect("Database connection failed")
}