use actix_web::HttpResponse;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, DbErr};
use serde_json::json;

use crate::{database::schema::file, services::file_details::FileDetails};

#[derive(Clone)]
pub struct DatabaseUploader {
    conn: DatabaseConnection,
}

impl DatabaseUploader {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let database_conn = Database::connect(db_url).await?;
        Ok(Self {
            conn: database_conn,
        })
    }

    pub async fn file_upload(
        &self,
        file: &FileDetails,
        user_id: i32,
    ) -> Result<HttpResponse, DbErr> {
        let new_file = file::ActiveModel {
            id: Set(file.file_id.clone()),
            name: Set(file.name.clone()),
            file_size: Set(file.file_size),
            file_type: Set(file.file_type.clone()),
            file_extension: Set(file.file_extension.clone()),
            latest_chunk: Set(file.latest_chunk),
            uploaded_at: Set(file.uploaded_at),
            file_id: Set(file.file_id.clone()),
            user_id: Set(user_id),
        };

        match new_file.insert(&self.conn).await {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({"success": true}))),
            Err(e) => Err(e),
        }
    }
}
