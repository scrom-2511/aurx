use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::database::schema::user;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum FileType {
    #[sea_orm(string_value = "image")]
    Image,
    #[sea_orm(string_value = "video")]
    Video,
    #[sea_orm(string_value = "audio")]
    Audio,
    #[sea_orm(string_value = "document")]
    Document,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub file_size: u32,
    pub file_type: FileType,
    pub file_extension: String,
    pub latest_chunk: i32,
    pub file_id: String,
    pub uploaded_at: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "user::Entity", from = "Column::Id", to = "user::Column::UserId")]
    User,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}